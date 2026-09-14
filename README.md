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

```rust,no_run
# use embedded_hal_mock::eh1::i2c::Mock;
# #[derive(Debug)]
# struct DocError;
# impl<E: core::fmt::Debug> From<ina4230::Ina4230Error<E>> for DocError {
#     fn from(_: ina4230::Ina4230Error<E>) -> Self { DocError }
# }
# impl From<ina4230::CalibrationError> for DocError {
#     fn from(_: ina4230::CalibrationError) -> Self { DocError }
# }
# macro_rules! info { ($($t:tt)*) => { { let _ = ($($t)*); } } }
# async fn example() -> Result<(), DocError> {
# let i2c = Mock::new(&[]);
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

info!("{} mV, {} mA", bus.to_millivolts(), current.to_milliamps());
# Ok(())
# }
# fn main() { tokio::runtime::Runtime::new().unwrap().block_on(example()).unwrap(); }
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

```rust,no_run
# use embedded_hal_mock::eh1::i2c::Mock;
# use ina4230::{AddrPinState, AddressPins, Channel, Ina4230};
# #[derive(Debug)]
# struct DocError;
# impl<E: core::fmt::Debug> From<ina4230::Ina4230Error<E>> for DocError {
#     fn from(_: ina4230::Ina4230Error<E>) -> Self { DocError }
# }
# async fn example() -> Result<(), DocError> {
# let i2c = Mock::new(&[]);
# let mut sensor = Ina4230::new(i2c, AddressPins { a0: AddrPinState::Gnd, a1: AddrPinState::Gnd });
sensor.set_channel_active(Channel::Ch3, false).await?;
# Ok(())
# }
# fn main() { tokio::runtime::Runtime::new().unwrap().block_on(example()).unwrap(); }
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

```rust,no_run
# use embedded_hal_mock::eh1::i2c::Mock;
# use ina4230::{AddrPinState, AddressPins, Ina4230};
# #[derive(Debug)]
# struct DocError;
# impl<E: core::fmt::Debug> From<ina4230::Ina4230Error<E>> for DocError {
#     fn from(_: ina4230::Ina4230Error<E>) -> Self { DocError }
# }
# macro_rules! warn { ($($t:tt)*) => { { let _ = ($($t)*); } } }
# async fn example() -> Result<(), DocError> {
# let i2c = Mock::new(&[]);
# let mut sensor = Ina4230::new(i2c, AddressPins { a0: AddrPinState::Gnd, a1: AddrPinState::Gnd });
let flags = sensor.read_flags().await?;
if flags.math_overflow() {
    warn!("current and power data may be invalid");
}
if flags.any_energy_overflow() {
    warn!("energy accumulator overflowed");
}
# Ok(())
# }
# fn main() { tokio::runtime::Runtime::new().unwrap().block_on(example()).unwrap(); }
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

**An energy overflow cannot be cleared through this crate.** `CONFIG2.ACC_RST`
is not exposed yet, and the bit is not read-to-clear, so once a channel's
accumulator wraps the flag stays set and the energy readings stay wrong. The
only recovery is `reset()`, which returns every register to its default —
including `SHUNT_CAL`, so every channel must be recalibrated afterwards or it
reports zero current forever (datasheet 8.1.2). Size `CurrentLsb` for the
expected run time if that matters: the accumulator is 32 bits, and once it
wraps the only way back is a full reinitialisation.

The four limit-alert flags do not correspond to the averaged readings. The
device compares each alert limit against *every* conversion rather than
against the averaged result that reaches the output registers (datasheet
6.3.5), so with averaging enabled a limit flag can report an excursion that
appears in no value this crate can read back. That disagreement is correct
behaviour. It cannot arise through this crate today, because `AVG` is not
configurable here and the power-on default is a single sample, but it can if
another controller on the bus has programmed `CONFIG1`.

## Alerts

The device has four alert slots. Each pairs a condition and a target channel
with a threshold, and asserts the ALERT pin when the condition is met.

A slot is not a channel: any slot can watch any channel, which is why
`AlertSlot` and `Channel` are separate types. The datasheet calls the slots
`ALERT1`..`ALERT4` (Table 7-7).

```rust,no_run
# use embedded_hal_mock::eh1::i2c::Mock;
# use ina4230::{AddrPinState, AddressPins, Ina4230};
# #[derive(Debug)]
# struct DocError;
# impl<E: core::fmt::Debug> From<ina4230::Ina4230Error<E>> for DocError {
#     fn from(_: ina4230::Ina4230Error<E>) -> Self { DocError }
# }
# async fn example() -> Result<(), DocError> {
# let i2c = Mock::new(&[]);
# let mut sensor = Ina4230::new(i2c, AddressPins { a0: AddrPinState::Gnd, a1: AddrPinState::Gnd });
use ina4230::{Alert, AlertSlot, BusVoltage, Channel, ShuntVoltage};

// Undervoltage on channel 2, watched by slot 1.
sensor.set_alert(
    AlertSlot::One,
    Channel::Ch2,
    Alert::BusUnder(BusVoltage::from_microvolts(11_000_000)),
).await?;

// Overcurrent on channel 1, expressed as a shunt voltage, watched by slot 2.
sensor.set_alert(
    AlertSlot::Two,
    Channel::Ch1,
    Alert::ShuntOver(ShuntVoltage::from_nanovolts(40_000_000)),
).await?;

sensor.clear_alert(AlertSlot::One).await?;
# Ok(())
# }
# fn main() { tokio::runtime::Runtime::new().unwrap().block_on(example()).unwrap(); }
```

Each `Alert` variant carries its threshold in the unit that variant implies, so
a bus threshold cannot be paired with a shunt condition — there is no way to
write it.

**Shunt and power thresholds need the target channel to be calibrated.** Their
scale comes from the channel's `AdcRange` and `CURRENT_LSB`; `set_alert`
returns `NotCalibrated` otherwise, before touching the bus. Bus thresholds have
a fixed 1.6 mV LSB and work on an uncalibrated channel.

**Recalibrating a channel disarms its shunt and power alerts.** `ALERT_LIMIT`
holds raw counts: shunt limits depend on `AdcRange`, whose two scales differ by
a factor of four, and power limits depend on `CURRENT_LSB`. A slot left armed
across a recalibration could therefore silently enforce a different threshold
than the one it was given. `calibrate` and `calibrate_all` disarm the affected
slots before the range moves; set them again afterwards. Bus alerts are
absolutely scaled and survive untouched.

Thresholds round to the nearest LSB, so a value read back through
`Ina4230::alert` is the one you supplied, not the one the device holds.

## Error handling

`Ina4230Error` is small on purpose: overflow conditions are reported through
`Flags`, not as errors, because an error can carry only one of them.

```rust,no_run
# use embedded_hal_mock::eh1::i2c::Mock;
# use ina4230::{AddrPinState, AddressPins, Channel, CurrentSensor, Ina4230, Ina4230Error};
# macro_rules! info { ($($t:tt)*) => { { let _ = ($($t)*); } } }
# macro_rules! error { ($($t:tt)*) => { { let _ = ($($t)*); } } }
# async fn example() {
# let i2c = Mock::new(&[]);
# let mut sensor = Ina4230::new(i2c, AddressPins { a0: AddrPinState::Gnd, a1: AddrPinState::Gnd });
match sensor.current(Channel::Ch1).await {
    Ok(i) => info!("{} mA", i.to_milliamps()),
    Err(Ina4230Error::NotCalibrated(ch)) => error!("calibrate {:?} first", ch),
    Err(Ina4230Error::Bus(e)) => error!("I²C error: {:?}", e),
    Err(e) => error!("unexpected error: {:?}", e),
}
# }
# fn main() { tokio::runtime::Runtime::new().unwrap().block_on(example()); }
```

`Ina4230Error` is `#[non_exhaustive]`, so a wildcard arm is required: code
outside this crate cannot match it exhaustively, and new variants are not a
breaking change. The arm above is unreachable for `current()` — that call
yields only `Bus` or `NotCalibrated` — but the compiler cannot know that.

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

- **`CONFIG2` alert behaviour**: `CNVR_MASK`, `ENOF_MASK`, `ALERT_LATCH`, and
  `ALERT_POL`.
- **`CONFIG1` conversion timing**: `AVG`, `VBUSCT`, and `VSHCT`. The power-on
  defaults are used: one sample and 1.1 ms bus and shunt conversion times.
- **Energy accumulator reset** (`CONFIG2.ACC_RST`), which also clears the
  energy overflow flags. Until this lands, an energy overflow is unrecoverable
  short of `reset()` and a full recalibration — see "Reading flags".
- **`SMBus` Alert Response** (address `0b0001100`) and **General Call reset**
  (`0x00`, `0x06`). These are bus protocols rather than registers, so they are
  not part of the generated register layer either.

The `FLAGS` register exposes the four alert-limit bits via
`Flags::limit_alerts()`; see "Alerts" for configuring their slots and limits.

### Out of scope

**High-speed I²C** (datasheet 6.5.3) is the third bus-level protocol the
datasheet describes, alongside the two listed above, but it is not a gap in
this driver. The device enters high-speed mode when a controller sends the
reserved master code `0b00001xxx` and leaves it on the next stop condition;
nothing in the sequence is addressed to the INA4230, and the part needs no
register written to participate. It is a property of the bus, and therefore of
whichever `embedded-hal-async` I²C implementation is passed to
`Ina4230::new` — not something this crate can offer or withhold. If the
controller supports 2.94 MHz operation, this driver already works over it.

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
