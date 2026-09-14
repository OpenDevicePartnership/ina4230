//! Accumulate energy over time and watch for overflow.
//!
//! The `ENERGY` register is a 32-bit accumulator that the device advances on
//! every conversion. This example samples it, reports the delta and the
//! average power over the interval, and demonstrates the one place in this
//! driver where reading a register has a side effect.
//!
//! # Reading FLAGS has side effects
//!
//! [`Ina4230::read_flags`] clears the conversion-ready bit and any latched
//! alert flags. There is no way to poll conversion-ready without reading the
//! rest of the register, so this example inspects the whole [`Flags`] snapshot
//! on every poll rather than looking at one bit and discarding the others.
//!
//! Energy overflow is *not* read-to-clear: it persists in the device until
//! `CONFIG2.ACC_RST` is written. [`Ina4230::reset_energy_accumulators`] is
//! that command, and unlike [`Ina4230::reset`] it preserves the calibration,
//! so this example recovers from a wrap and carries on from a fresh baseline.
//!
//! # Wiring
//!
//! Pico de Gallo **v1.1** 2×12 box header. Viewed from above with the USB
//! connector pointing up, pin 1 is top-right; odd pins are the top row.
//!
//! ```text
//! Pico de Gallo v1.1 header        INA4230 (YBJ DSBGA)
//! -------------------------        -------------------
//! Pin  1  VREF (+3V3)  ----------  VS   (A4)   supply, 1.7 V to 5.5 V
//! Pin  1  VREF (+3V3)  ----------  EN   (B3)   logic high enables the device
//! Pin  2  GND          ----------  GND  (B4)
//! Pin  3  SDA (GPIO 2) ----------  SDA  (D4)   4.7 kOhm pull-up on the board
//! Pin  4  SCL (GPIO 3) ----------  SCL  (C4)   4.7 kOhm pull-up on the board
//! Pin  2  GND          ----------  A0   (C2)   \  GND/GND selects 0x40
//! Pin  2  GND          ----------  A1   (C3)   /
//! ```
//!
//! Channel 1 sense side, high-side configuration:
//!
//! ```text
//!   supply ---+--[ 10 mOhm shunt ]--+--- load
//!             |                     |
//!          IN+1 (A3)             IN-1 (A2)
//! ```
//!
//! A steady, non-trivial load makes this example far more interesting than an
//! idle rail: energy only accumulates while current flows.
//!
//! # Safety
//!
//! The sense inputs tolerate a common-mode voltage up to 50 V, but Pico de
//! Gallo is a 3.3 V board. Only VS, GND, SDA, SCL, EN, A0 and A1 may be shared
//! with the bridge. Never connect the monitored rail, or either side of the
//! shunt, to the header.
//!
//! # Running
//!
//! ```sh
//! cargo run --example energy
//! ```

// Energy values are printed for a human, never fed back into arithmetic. The
// accumulator's full scale does exceed an f64 mantissa, so the cast really is
// lossy at the top of the range; it costs a fraction of a joule out of
// billions, which does not matter for a display value.
#![allow(clippy::cast_precision_loss)]

use std::time::{Duration, Instant};

use ina4230::{
    AdcRange, AddrPinState, AddressPins, Calibration, Channel, CurrentLsb, Energy, EnergySensor, Ina4230,
    ShuntResistance,
};
use pico_de_gallo_hal::{Hal, I2cFrequency};

/// Shunt fitted on channel 1, in microohms. 10 mOhm.
const SHUNT_UOHM: u32 = 10_000;

/// Current resolution, in nanoamps per LSB. 200 uA, suiting a 5 A full scale.
const CURRENT_LSB_NA: u32 = 200_000;

/// How long to sample for.
const RUN_FOR: Duration = Duration::from_secs(30);

/// How often to report.
const SAMPLE_EVERY: Duration = Duration::from_secs(2);

#[tokio::main]
async fn main() {
    let mut hal = Hal::new_validated().expect("no Pico de Gallo found, or firmware too old");
    hal.i2c_set_config(I2cFrequency::Fast)
        .expect("failed to set I2C to 400 kHz");

    let calibration = Calibration::new(
        CurrentLsb::from_nanoamps(CURRENT_LSB_NA).expect("current LSB out of range"),
        ShuntResistance::from_microohms(SHUNT_UOHM).expect("shunt must be non-zero"),
        AdcRange::Range0,
    )
    .expect("no usable SHUNT_CAL for this shunt and current LSB");

    let mut sensor = Ina4230::new(
        hal.i2c(),
        AddressPins {
            a0: AddrPinState::Gnd,
            a1: AddrPinState::Gnd,
        },
    );

    assert!(
        sensor.is_present().await.expect("I2C read failed"),
        "device at {:#04X} is not an INA4230",
        sensor.address().as_u8()
    );

    // Reset zeroes the energy accumulator along with everything else, giving a
    // clean baseline. It also clears the cached calibration, so calibrate
    // after, never before.
    sensor.reset().await.expect("reset failed");
    sensor
        .calibrate(Channel::Ch1, calibration)
        .await
        .expect("calibration write failed");

    // The ENERGY LSB is 32 x CURRENT_LSB, so the accumulator's full range is
    // known up front. Worth printing: it tells you how long you can run before
    // it wraps.
    let energy_lsb_nj = u64::from(CURRENT_LSB_NA) * 32;
    let full_scale_nj = u64::from(u32::MAX) * energy_lsb_nj;
    println!("ENERGY LSB     : {energy_lsb_nj} nJ");
    println!("accumulator max: {:.3} J", full_scale_nj as f64 / 1_000_000_000.0);
    println!();

    let started = Instant::now();
    let mut previous: Option<Energy> = None;
    let mut last_sample = Instant::now();

    println!("{:>8} {:>14} {:>14} {:>12}", "elapsed", "energy", "delta", "avg power");

    while started.elapsed() < RUN_FOR {
        let flags = sensor.read_flags().await.expect("flags read failed");

        // Inspect the whole snapshot: the alert bits it carries are cleared by
        // this very read and will not be reported again.
        if flags.math_overflow() {
            eprintln!("warning: math overflow, current and power data may be invalid");
        }
        if flags.energy_overflow(Channel::Ch1) {
            eprintln!("warning: energy accumulator overflowed and has wrapped");
            eprintln!("         clearing it and starting a new integration interval");

            // Clears the accumulator and the sticky flag in one CONFIG2 write,
            // leaving SHUNT_CAL and the cached calibration alone. The next
            // delta would be meaningless against the pre-reset total, so drop
            // the baseline too.
            sensor
                .reset_energy_accumulators(&[Channel::Ch1])
                .await
                .expect("energy accumulator reset failed");
            previous = None;
        }
        if !flags.conversion_ready() {
            continue;
        }

        if last_sample.elapsed() < SAMPLE_EVERY {
            continue;
        }
        let interval = last_sample.elapsed();
        last_sample = Instant::now();

        let energy = sensor.energy(Channel::Ch1).await.expect("energy read failed");

        // Work in the integer base unit; only render to f64 for printing.
        let delta_nj = previous.map_or(0, |p: Energy| energy.as_nanojoules().saturating_sub(p.as_nanojoules()));
        let avg_power_mw = delta_nj as f64 / interval.as_secs_f64() / 1_000_000.0;
        previous = Some(energy);

        println!(
            "{:>7.1}s {:>12.4} J {:>12.4} J {:>9.3} mW",
            started.elapsed().as_secs_f32(),
            energy.to_joules(),
            delta_nj as f64 / 1_000_000_000.0,
            avg_power_mw,
        );
    }
}
