# INA4230 Rust Device Driver

A `#[no_std]` platform-agnostic driver for the [INA4230](https://www.ti.com/lit/ds/symlink/ina4230.pdf)
quad-channel power and energy sense monitor, based on the [`embedded-hal`](https://docs.rs/embedded-hal) traits.

Higher-level measurement APIs are built on the [embedded-sensors](https://github.com/OpenDevicePartnership/embedded-sensors)
async sensor traits (`VoltageSensor`, `CurrentSensor`, `PowerSensor`, `EnergySensor`).

## Features

- Full register coverage via pre-generated `src/device.rs` (generated from `INA4230.toml` using `device-driver-cli`)
- Async-first I²C interface (`embedded-hal-async`)
- Four independent measurement channels (bus voltage, shunt voltage, current, power, energy)
- Per-channel calibration with independent shunt resistor, current range, and ADC range
- ADC range selection (±81.92 mV or ±20.48 mV full scale) written to hardware on calibration
- Channel enable/disable support
- INA4230-specific error variants (`NotCalibrated`, `MathOverflow`, `EnergyOverflow`)
- Optional `defmt` logging support
- `no_std` compatible

## Usage

```toml
[dependencies]
ina4230 = "0.1.0"
embedded-hal-async = "1"
```

```rust,ignore
use ina4230::{AddrPinState, AdcRange, Channel, Ina4230};

// i2c implements embedded_hal_async::i2c::I2c
// AddrPinState selects the I²C address via A0 and A1 pin strapping on the device
let mut sensor = Ina4230::new(i2c, AddrPinState::Gnd, AddrPinState::Gnd);

// Reset, then calibrate each channel before taking measurements
sensor.reset().await?;
sensor.calibrate(Channel::Ch1, CURRENT_LSB_CH1, SHUNT_OHMS_CH1, AdcRange::Range0).await?;

// Wait for conversion
while !sensor.conversion_ready().await? {}

// Read channel 1
let vbus_mv = sensor.bus_voltage(Channel::Ch1).await?;
let ima      = sensor.current(Channel::Ch1).await?;
let pmw      = sensor.power(Channel::Ch1).await?;
```

## Configuration

Before taking measurements, two hardware-specific parameters must be set per channel.

### Shunt Resistor Value

Set `SHUNT_OHMS_CHx` to the resistance of the shunt resistor fitted on that channel, in ohms:

```rust
const SHUNT_OHMS_CH1: f32 = 0.010;  // 10 mΩ shunt on channel 1
```

Use a precision resistor (0.1% tolerance or better) for accurate results.
For very low value shunts, use Kelvin (4-wire) connections to eliminate
lead resistance errors.

### Current Resolution

`CURRENT_LSB` determines the resolution of the current measurement. A smaller
value gives finer resolution but a lower full-scale range; a larger value gives
a wider range but coarser resolution. The current register is 16-bit signed,
giving 32767 steps above zero.

Set `MAX_CURRENT_CHx` to the maximum current you expect on that channel — the
driver calculates the optimal `CURRENT_LSB` automatically:

```rust
// Example: expecting up to 100 mA on channel 1
const MAX_CURRENT_CH1: f32 = 0.1;            // amps
const CURRENT_LSB_CH1: f32 = MAX_CURRENT_CH1 / 32767.0;  // ~3.05 µA/LSB
```

The driver uses `CURRENT_LSB` to compute the `SHUNT_CAL` register value
written during `calibrate()`:

```text
SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT)
```

And to convert the raw current register reading back to milliamperes:

```text
Current [mA] = raw × CURRENT_LSB × 1000
```

If `MAX_CURRENT_CHx` is set too low, the current register will saturate and
readings will be clamped. If set too high, resolution will be unnecessarily
coarse.

### ADC Range

The INA4230 supports two shunt input voltage ranges, configured via `AdcRange`:

| `AdcRange`         | Full-scale range | LSB    |
| ------------------ | ---------------- | ------ |
| `Range0` (default) | ±81.92 mV        | 2.5 µV |
| `Range1`           | ±20.48 mV        | 625 nV |

`Range0` is the default and suits most applications. Use `Range1` for higher
resolution when measuring small currents through a large shunt resistor.

When using `Range1`, the `SHUNT_CAL` register value is automatically divided
by 4, the shunt voltage LSB is adjusted to 625 nV, and `CONFIG2.RANGE` is
updated in hardware:

```text
Range0: SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT)
Range1: SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT) / 4
```

### Calibration

The calibration register serves two purposes: it provides the device with
the shunt resistor value used to calculate current from the measured
differential voltage, and it sets the resolution of the current and power
registers through the `CURRENT_LSB` and `Power_LSB` values.

Call `calibrate()` before taking current, power, or energy measurements.
Each channel is calibrated independently:

```rust,ignore
sensor.calibrate(Channel::Ch1, CURRENT_LSB_CH1, SHUNT_OHMS_CH1, AdcRange::Range0).await?;
sensor.calibrate(Channel::Ch2, CURRENT_LSB_CH2, SHUNT_OHMS_CH2, AdcRange::Range0).await?;
sensor.calibrate(Channel::Ch3, CURRENT_LSB_CH3, SHUNT_OHMS_CH3, AdcRange::Range1).await?;
sensor.calibrate(Channel::Ch4, CURRENT_LSB_CH4, SHUNT_OHMS_CH4, AdcRange::Range0).await?;
```

Or use `calibrate_all()` to configure all four channels in one call:

```rust,ignore
sensor.calibrate_all([
    (CURRENT_LSB_CH1, SHUNT_OHMS_CH1, AdcRange::Range0),
    (CURRENT_LSB_CH2, SHUNT_OHMS_CH2, AdcRange::Range0),
    (CURRENT_LSB_CH3, SHUNT_OHMS_CH3, AdcRange::Range1),
    (CURRENT_LSB_CH4, SHUNT_OHMS_CH4, AdcRange::Range0),
]).await?;
```

The calibration register must be programmed after initial power up, power
cycle events, or device enable to receive valid current, power, and energy
results. Bus voltage and shunt voltage readings do not require calibration.

### Channel Management

All four channels are active by default after power up. Unused channels can
be disabled to reduce conversion time:

```rust,ignore
sensor.set_channel_active(Channel::Ch3, false).await?;
sensor.set_channel_active(Channel::Ch4, false).await?;
```

## Error Handling

The driver returns `Ina4230Error` which covers both I²C bus errors and
device-level conditions:

```rust,ignore
match sensor.current(Channel::Ch1).await {
    Ok(ma) => info!("Current: {} mA", ma),
    Err(Ina4230Error::NotCalibrated) => error!("Call calibrate() first"),
    Err(Ina4230Error::MathOverflow) => error!("Current exceeds full-scale range"),
    Err(Ina4230Error::EnergyOverflow(ch)) => error!("Energy overflow on {:?}", ch),
    Err(Ina4230Error::Bus(e)) => error!("I²C error: {:?}", e),
}
```

Use `check_flags()` to explicitly poll for overflow conditions:

```rust,ignore
if let Err(e) = sensor.check_flags().await {
    warn!("INA4230 flag: {:?}", e);
}
```

## I²C Addresses

The I²C address is selected by the A0 and A1 pin strapping on the device.
Pass two `AddrPinState` values to `Ina4230::new()` — the first for A0, the
second for A1:

```rust,ignore
// A0=GND, A1=GND → address 0x40
let sensor = Ina4230::new(i2c, AddrPinState::Gnd, AddrPinState::Gnd);

// A0=VS, A1=SDA → address 0x46
let sensor = Ina4230::new(i2c, AddrPinState::Vs, AddrPinState::Sda);
```

All 16 address combinations (per datasheet Table 6-1):

| A0  | A1  | Address |
| --- | --- | ------- |
| GND | GND | `0x40`  |
| VS  | GND | `0x41`  |
| GND | SDA | `0x42`  |
| GND | SCL | `0x43`  |
| GND | VS  | `0x44`  |
| VS  | VS  | `0x45`  |
| VS  | SDA | `0x46`  |
| VS  | SCL | `0x47`  |
| SDA | GND | `0x48`  |
| SDA | VS  | `0x49`  |
| SDA | SDA | `0x4A`  |
| SDA | SCL | `0x4B`  |
| SCL | GND | `0x4C`  |
| SCL | VS  | `0x4D`  |
| SCL | SDA | `0x4E`  |
| SCL | SCL | `0x4F`  |

## Regenerating `src/device.rs`

```sh
cargo install device-driver-cli
device-driver-cli -m INA4230.toml -o generated.rs -d Device
rustfmt --edition 2024 generated.rs
cp generated.rs src/device.rs
```

## MSRV

Rust `1.94` and up.

## License

Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution submitted for
inclusion in the work by you shall be licensed under the terms of the
MIT license.

License: MIT
