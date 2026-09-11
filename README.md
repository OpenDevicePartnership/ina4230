[![check](https://github.com/OpenDevicePartnership/ina4230/actions/workflows/check.yml/badge.svg)](https://github.com/OpenDevicePartnership/ina4230/actions/workflows/check.yml)
[![no-std](https://github.com/OpenDevicePartnership/ina4230/actions/workflows/nostd.yml/badge.svg)](https://github.com/OpenDevicePartnership/ina4230/actions/workflows/nostd.yml)
[![device-driver-pregen-check](https://github.com/OpenDevicePartnership/ina4230/actions/workflows/device-driver.yml/badge.svg)](https://github.com/OpenDevicePartnership/ina4230/actions/workflows/device-driver.yml)
[![release-plz](https://github.com/OpenDevicePartnership/ina4230/actions/workflows/release-plz.yml/badge.svg)](https://github.com/OpenDevicePartnership/ina4230/actions/workflows/release-plz.yml)
[![crates.io](https://img.shields.io/crates/v/ina4230)](https://crates.io/crates/ina4230)
[![license](https://img.shields.io/crates/l/ina4230)](https://github.com/OpenDevicePartnership/ina4230/blob/main/LICENSE)

# INA4230 Rust Device Driver

A `#[no_std]` platform-agnostic driver for the [INA4230](https://www.ti.com/lit/ds/symlink/ina4230.pdf)
48 V quad-channel current, voltage, power, and energy monitor, based on the
[`embedded-hal`](https://docs.rs/embedded-hal) traits.

## Design

The driver is split into a pure core and a thin shell.

**`units` and `convert`** hold every physical quantity, the validated
calibration inputs, and all the register decoding. They have no bus, no
`async`, and no HAL dependency. Calibration and decoding are integer
arithmetic; floating point appears only in display-oriented `to_*` methods.
These modules build and test on the host as readily as on the target. This is
where the thinking lives, and where the tests are.

**`Ina4230`** is the shell. Each method moves bytes to or from the device and
hands them to a pure function. It contains no arithmetic.

Measurements are returned as newtypes over integers, in whichever unit keeps
the conversion from the raw register *exact*:

| Type           | Unit | Rendering methods                    |
| -------------- | ---- | ------------------------------------ |
| `ShuntVoltage` | nV   | `to_millivolts`                      |
| `BusVoltage`   | µV   | `to_millivolts`, `to_volts`          |
| `Current`      | nA   | `to_milliamps`, `to_amps`            |
| `Power`        | nW   | `to_milliwatts`, `to_watts`          |
| `Energy`       | nJ   | `to_millijoules`, `to_joules`        |

`f32` appears only in those `to_*` methods. They are for display. For
arithmetic, use each type's base-unit accessor — `as_nanovolts`,
`as_microvolts`, `as_nanoamps`, `as_nanowatts`, `as_nanojoules` — which are
lossless. The coarser convenience accessors (`as_microamps`, `as_microwatts`,
`as_microjoules`) divide and therefore truncate toward zero.

## Features

- Full register coverage via a pre-generated `src/device.rs`, built from
  `INA4230.ddsl` with `device-driver-cli` and checked in sync by CI
- Async-first I²C interface (`embedded-hal-async`)
- Four independent measurement channels (bus voltage, shunt voltage, current,
  power, energy)
- Per-channel calibration with independent shunt resistor, current resolution,
  and ADC range
- Optional `defmt` logging support

## Usage

```toml
[dependencies]
ina4230 = "0.1.0"
embedded-hal-async = "1"
```

```rust,ignore
use ina4230::{
    AdcRange, AddrPinState, AddressPins, Calibration, Channel, CurrentLsb,
    CurrentSensor, Ina4230, ShuntResistance, VoltageSensor,
};

// i2c implements embedded_hal_async::i2c::I2c
let mut sensor = Ina4230::new(i2c, AddressPins {
    a0: AddrPinState::Gnd,
    a1: AddrPinState::Gnd,
});

// Calibration is validated up front, away from the bus.
let cal = Calibration::new(
    CurrentLsb::from_nanoamps(500_000)?,      // 500 µA/LSB
    ShuntResistance::from_microohms(8_000)?,  // 8 mΩ
    AdcRange::Range0,
)?;

sensor.reset().await?;
sensor.calibrate(Channel::Ch1, cal).await?;

// Wait for a conversion. See the note on reading flags below.
while !sensor.read_flags().await?.conversion_ready() {}

let bus = sensor.bus_voltage(Channel::Ch1).await?;
let current = sensor.current(Channel::Ch1).await?;

defmt::info!("{} mV, {} mA", bus.to_millivolts(), current.to_milliamps());
```

## Configuration

Two hardware-specific parameters are needed per channel, plus a choice of ADC
range. All three are checked once, when the `Calibration` is built; nothing
downstream re-checks them.

### Shunt resistance

The resistance of the shunt fitted on that channel, in microohms:

```rust
# use ina4230::ShuntResistance;
let shunt = ShuntResistance::from_microohms(8_000)?; // 8 mΩ
# Ok::<(), ina4230::CalibrationError>(())
```

Use a precision resistor (0.1% tolerance or better). For very low values, use
Kelvin (4-wire) connections to eliminate lead resistance errors.

### Current resolution

`CURRENT_LSB` sets the resolution of the current measurement. Smaller gives
finer resolution and a lower full-scale range; larger gives a wider range and
coarser resolution.

Datasheet Equation 2 gives the minimum as `MAX_CURRENT / 2^15`:

```rust
# use ina4230::CurrentLsb;
// For a 10 A maximum: 305_176 nA/LSB.
let minimum = CurrentLsb::min_for_max_current(10_000_000_000)?;
# Ok::<(), ina4230::CalibrationError>(())
```

That is a floor, not a recommendation. Equation 2 divides by 2^15 = 32768 while
the `CURRENT` register saturates at 32767, so the minimum lands about 0.003%
below the requested full scale. The datasheet expects you to round up to a
convenient number — its worked example takes a 305.17578 µA minimum for 10 A
and uses 500 µA — and requires the selected value to stay *strictly below*
eight times the minimum to avoid losing resolution. Rounding up also closes the
gap.

### ADC range

| `AdcRange`         | Full-scale range | LSB    |
| ------------------ | ---------------- | ------ |
| `Range0` (default) | ±81.92 mV        | 2.5 µV |
| `Range1`           | ±20.48 mV        | 625 nV |

`Range0` suits most applications. Use `Range1` for higher resolution when
measuring small currents through a large shunt. Selecting `Range1` divides
`SHUNT_CAL` by 4 and adjusts the shunt LSB automatically; `CONFIG2.RANGE` is
written to hardware when the channel is calibrated.

### Calibration

The calibration register tells the device the shunt value used to derive
current from the measured differential voltage, and sets the resolution of the
current, power, and energy registers.

```rust
# use ina4230::{AdcRange, Calibration, CurrentLsb, ShuntResistance};
let cal = Calibration::new(
    CurrentLsb::from_nanoamps(500_000)?,
    ShuntResistance::from_microohms(8_000)?,
    AdcRange::Range0,
)?;
assert_eq!(cal.shunt_cal().as_u16(), 1280); // datasheet §8.2.2.3
# Ok::<(), ina4230::CalibrationError>(())
```

Building a `Calibration` is the only fallible step in configuring the device.
It reports `ShuntCalOverflow` or `ShuntCalUnderflow` rather than silently
clamping, and rejects a `SHUNT_CAL` of zero because the device then reports
zero current indefinitely (datasheet §8.1.2).

Call `calibrate()` before reading current, power, or energy. Bus voltage does
not require it; shunt voltage does, because its scale depends on the ADC range.
Each channel is independent, and `calibrate_all()` programs all four with a
single `CONFIG2` update.

Calibration must be reprogrammed after power-up, a power cycle, a device
enable, or a `reset()`. `reset()` clears the driver's cached calibration to
match, so a subsequent measurement fails with `NotCalibrated` rather than
returning a confidently wrong number.

The cache records only successful writes made through this driver. It cannot
observe a power cycle, an EN-pin toggle, a General Call reset, or writes by
another bus controller. After any of those, recalibrate the affected channels
or construct a new driver before requesting scaled measurements.

Calibration writes are also not atomic: `calibrate()` writes `CONFIG2.RANGE`
and then `SHUNT_CAL`, and `calibrate_all()` writes `CONFIG2` once followed by
four calibration registers. A bus error can leave the device partly programmed.
Each cache entry is updated only after its own write succeeds, so on error
either retry until it succeeds or inspect `calibration()` to see how far it
got.

### Channel management

All four channels are active after power-up. Unused channels can be disabled to
shorten the conversion cycle:

```rust,ignore
sensor.set_channel_active(Channel::Ch3, false).await?;
```

## Examples

The `examples/` directory drives a real INA4230 from a host machine through a
[Pico de Gallo](https://github.com/OpenDevicePartnership/pico-de-gallo) USB
bridge, using `pico-de-gallo-hal` as the `embedded-hal-async` implementation.
Each example documents its own wiring against the Pico de Gallo **v1.1** box
header at the top of the file.

| Example          | What it shows                                                        |
| ---------------- | -------------------------------------------------------------------- |
| `scan`           | Probe all 16 address strappings and identify what answered            |
| `single_channel` | Calibrate one channel; read bus, shunt, current and power             |
| `four_channel`   | Four independent shunts and current ranges via `calibrate_all`        |
| `energy`         | Accumulate energy over time and interpret the `FLAGS` snapshot        |
| `adc_range`      | `Range0` against `Range1` on the same shunt, and what resolution buys |

```sh
cargo run --example scan
```

Start with `scan`: it proves the wiring and the address before any measurement
example can work. A common first mistake is leaving the `EN` pin floating, in
which case the device is disabled and never acknowledges.

These need hardware, so they are not run in CI — only compiled.

## Reading flags

`read_flags()` returns the whole `FLAGS` register:

```rust,ignore
let flags = sensor.read_flags().await?;
if flags.math_overflow() {
    warn!("current and power data may be invalid");
}
if flags.any_energy_overflow() {
    warn!("energy accumulator overflowed");
}
```

**This read has side effects.** Reading `FLAGS` clears the conversion-ready
flag and any latched alert flags (datasheet Table 7-20, and
`CONFIG2.ALERT_LATCH`). There is no way to poll `CVRF` without reading the
other flags at the same time, which is why the API returns the whole register
instead of per-bit accessors that would discard the rest of the snapshot.

The math-overflow and energy-overflow bits are not specified as read-to-clear;
energy overflow is cleared through `CONFIG2.ACC_RST`. Those conditions persist
in the device across reads. Latched alerts do not, so if alert information
matters, inspect every `Flags` value a polling loop returns rather than only
the last one.

## Error handling

`Ina4230Error` is small on purpose: overflow conditions are reported through
`Flags`, not as errors, because an error can carry only one of them.

```rust,ignore
match sensor.current(Channel::Ch1).await {
    Ok(i) => info!("{} mA", i.to_milliamps()),
    Err(Ina4230Error::NotCalibrated(ch)) => error!("calibrate {:?} first", ch),
    Err(Ina4230Error::Bus(e)) => error!("I²C error: {:?}", e),
}
```

`NotCalibrated` is detected before any bus traffic is generated.

## I²C addresses

The address is selected by the A0 and A1 pin strapping. `AddressPins` uses
named fields because both are the same type and a transposed pair yields a
valid-looking but wrong address:

```rust
# use ina4230::{AddrPinState, Address, AddressPins};
let addr = Address::from_pins(AddressPins {
    a0: AddrPinState::Gnd,
    a1: AddrPinState::Sda,
});
assert_eq!(addr.as_u8(), 0x48);
```

Datasheet Table 6-1 is a regular encoding, `0x40 | (A1 << 2) | A0`, with
`GND = 0`, `VS = 1`, `SDA = 2`, `SCL = 3`:

| A1  | A0  | Address | A1  | A0  | Address |
| --- | --- | ------- | --- | --- | ------- |
| GND | GND | `0x40`  | SDA | GND | `0x48`  |
| GND | VS  | `0x41`  | SDA | VS  | `0x49`  |
| GND | SDA | `0x42`  | SDA | SDA | `0x4A`  |
| GND | SCL | `0x43`  | SDA | SCL | `0x4B`  |
| VS  | GND | `0x44`  | SCL | GND | `0x4C`  |
| VS  | VS  | `0x45`  | SCL | VS  | `0x4D`  |
| VS  | SDA | `0x46`  | SCL | SDA | `0x4E`  |
| VS  | SCL | `0x47`  | SCL | SCL | `0x4F`  |

Note the column order: A1 first, matching the datasheet.

## Not yet implemented

The following register controls and device protocols are defined by
`INA4230.ddsl` or the datasheet, but have no high-level API yet:

- **Alert configuration** (`ALERT_CONFIG1..4`, addresses `0x07`, `0x0F`,
  `0x17`, `0x1F`). Selects the alert function — shunt over/under limit, bus
  over/under limit, power over limit — and the channel it applies to.
  Encodings 0, 6 and 7 are all documented as "reserved, no effect", so they
  decode to a single `NoEffect` variant.
- **Alert limits** (`ALERT_LIMIT1..4`, addresses `0x06`, `0x0E`, `0x16`,
  `0x1E`). The format follows the result register the selected alert function
  refers to: signed 16-bit for shunt limits, unsigned 15-bit for bus limits
  (bit 15 reserved), and unsigned 16-bit for power limits. Representing that
  reinterpretation safely is the interesting part of the design, and the
  reason these are not simply exposed as a `u16` setter.
- **`CONFIG2` alert behaviour**: `CNVR_MASK`, `ENOF_MASK`, `ALERT_LATCH`, and
  `ALERT_POL`.
- **`CONFIG1` timing and operating mode**: `AVG`, `VBUSCT`, `VSHCT`, and
  `MODE`. The power-on defaults are used: one sample, 1.1 ms bus and shunt
  conversion times, and continuous shunt-and-bus conversion.
- **Energy accumulator reset** (`CONFIG2.ACC_RST`), which also clears the
  energy overflow flags.
- **`SMBus` Alert Response** (address `0b0001100`) and **General Call reset**
  (`0x00`, `0x06`). These are bus protocols rather than registers, so they are
  not part of the generated register layer either.

The `FLAGS` register already exposes the four alert-limit bits via
`Flags::limit_alerts()`, so alert conditions are observable even though they
cannot yet be configured through this crate.

## Regenerating `src/device.rs`

```sh
cargo install device-driver-cli
ddc build rust -s INA4230.ddsl -o src/device.rs --rust-defmt-feature=defmt
rustfmt --edition 2024 src/device.rs
```

CI verifies that the committed file matches the generator output.

## MSRV

Rust `1.94` and up, bounded by `device-driver` 2.1.

## License

Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution submitted for
inclusion in the work by you shall be licensed under the terms of the
MIT license.
