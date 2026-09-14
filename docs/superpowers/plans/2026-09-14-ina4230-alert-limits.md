# INA4230 Alert Limits Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let callers arm the INA4230's four alert slots with typed thresholds, so an over/under condition on shunt voltage, bus voltage or power asserts the ALERT pin.

**Architecture:** A new `Alert` enum whose variants each carry the threshold in the unit that variant implies, making a mismatched function/threshold pair unrepresentable. Alert registers are re-indexed by a new `AlertSlot` type rather than `Channel`, because a slot may watch any channel. Encoding is the fallible inverse of the existing decode functions and lives beside them in the pure `convert` module. The driver caches the slot-to-alert mapping so that recalibrating a channel can disarm any slot whose scale just moved.

**Tech Stack:** Rust 1.94, `no_std`, `embedded-hal-async`, `device-driver` 2.1 (DDSL codegen), `embedded-hal-mock`, `proptest`, `tokio` for async tests.

**Design doc:** `docs/superpowers/specs/2026-09-14-ina4230-alert-limits-design.md`

**Scope:** Step 1 of the design only — slots, limits, invalidation. Step 2 (`AlertPinConfig` and the `CONFIG2` pin bits) is deliberately out of scope; adding it later is not a breaking change.

---

## File Structure

| File | Responsibility | Action |
| --- | --- | --- |
| `INA4230.ddsl` | Register description. Gains an `alert-slot` enum and an `alert-regs` block; loses the two alert registers from `channel-regs`. | Modify |
| `src/device.rs` | Generated register layer. Never hand-edited. | Regenerate |
| `src/alert.rs` | Alert-domain types: `AlertSlot`, `Alert`. | Create |
| `src/units.rs` | Physical quantities. Only change is promoting five constructors to `pub`. | Modify |
| `src/convert.rs` | Pure conversions. Gains three `encode_*` functions and two rounding helpers. | Modify |
| `src/lib.rs` | The shell. Error variant, alert cache, `set_alert`/`clear_alert`/`alert`, invalidation, `Flags::limit_alert`. | Modify |
| `tests/core.rs` | Pure-core tests. Encode round-trips, rounding, rejection. | Modify |
| `tests/shell.rs` | Mock-bus tests. Register addressing, cache behaviour, invalidation. | Modify |
| `README.md` | User-facing docs. | Modify |

`src/alert.rs` is a new file rather than an extension of `units.rs`: `units.rs` is already ~600 lines and is about physical quantities, while these are configuration types. They change together and are a distinct responsibility.

**Naming collision to be aware of:** the DDSL `alert-slot` enum generates `device::AlertSlot`, and we also define a public `AlertSlot` in `src/alert.rs`. This mirrors the existing `Channel` / `device::Channel` split, which is bridged by `impl From<Channel> for device::Channel` at `src/lib.rs:527`. Do the same for slots; do not try to reuse the generated type in the public API.

---

## Task 1: Re-index the alert registers by slot

**Files:**
- Modify: `INA4230.ddsl:12-17` (add enum after), `INA4230.ddsl:100-151` (remove), new block after `channel-regs`
- Regenerate: `src/device.rs`

This task has no new tests. It is a pure restructure: addresses and strides are unchanged, and no call site in `src/` uses the alert accessors yet. The existing suite plus `device-driver-pregen-check` are the regression net.

- [ ] **Step 1: Add the `alert-slot` enum**

In `INA4230.ddsl`, immediately after the closing `},` of the `channel` enum (line 17), insert:

```
    /// Alert slot selector.
    ///
    /// The four ALERT_CONFIG/ALERT_LIMIT register pairs are *not* per-channel.
    /// Datasheet Table 7-20 describes each LIMITn_ALERT flag as "independent of
    /// channel", and Table 7-8 gives ALERT_CONFIG a CHANNEL field selecting
    /// which channel the slot watches. Slot 2 may watch channel 4.
    enum alert-slot {
        one: 0,
        two: 1,
        three: 2,
        four: 3,
    },
```

- [ ] **Step 2: Remove the alert registers from `channel-regs`**

Delete lines 100-151 of `INA4230.ddsl` — the `alert-limit` register, the `alert-config` register, and the `alert-function` and `alert-channel` enums nested inside it. The `calibration` register at line 90-99 becomes the last entry in the block, so make sure its trailing `},` is still followed by the block's own closing `},`.

- [ ] **Step 3: Add the `alert-regs` block**

Immediately after the closing `},` of the `channel-regs` block, insert:

```
    /// Alert slot register bank.
    ///
    /// Datasheet Table 7-1: ALERT_LIMIT at 0x06, 0x0E, 0x16, 0x1E and
    /// ALERT_CONFIG at 0x07, 0x0F, 0x17, 0x1F. Same stride as `channel-regs`,
    /// but the index is an alert slot, not a channel.
    block alert-regs[alert-slot stride 8] {
        address-offset: 0,

        /// Alert limit register.
        ///
        /// Datasheet 7.1.5: the format follows the result register the
        /// selected alert function refers to. Shunt voltage limits are signed
        /// 16-bit, bus voltage limits are unsigned 15-bit (bit 15 reserved),
        /// and power limits are unsigned 16-bit.
        register alert-limit {
            address: 0x06,
            reset: 0x00,
            fields: fieldset _ {
                size-bytes: 2,

                /// Alert threshold, in the format of the corresponding result
                /// register.
                field limit 15:0,
            },
        },
        /// Alert configuration register.
        register alert-config {
            address: 0x07,
            reset: 0x00,
            fields: fieldset _ {
                size-bytes: 2,

                /// Channel assignment for this alert.
                field channel 4:3 -> _ as enum alert-channel {
                    ch1: 0,
                    ch2: 1,
                    ch3: 2,
                    ch4: 3,
                },
                /// Active alert function selection.
                field alert-mask 2:0 -> _ as
                    /// Which condition asserts the ALERT pin.
                    ///
                    /// Datasheet Table 7-8 documents encodings 0, 6 and 7 all
                    /// as "reserved, no effect". They are aliases of one
                    /// another rather than illegal states, and 0 is the
                    /// power-on value, so all three decode to `no-effect`
                    /// and the conversion is infallible.
                    enum alert-function {
                        /// Reserved, no effect. Encodings 6 and 7 behave
                        /// identically and collapse into this variant.
                        no-effect: default 0,
                        shunt-over-limit: 1,
                        shunt-under-limit: 2,
                        bus-over-limit: 3,
                        bus-under-limit: 4,
                        power-over-limit: 5,
                    },
            },
        },
    },
```

- [ ] **Step 4: Regenerate the register layer**

Run:
```bash
ddc build rust -s INA4230.ddsl -o src/device.rs --rust-defmt-feature=defmt
rustfmt --edition 2024 src/device.rs
```

If `ddc` is not installed: `cargo install device-driver-cli`.

- [ ] **Step 5: Confirm the accessor moved and the addresses did not**

Run:
```bash
rg "pub fn (alert_regs|channel_regs)" src/device.rs
```
Expected: both present. `alert_regs` takes `index: AlertSlot`, `channel_regs` takes `index: Channel`.

Run:
```bash
cargo test
```
Expected: all existing tests pass unchanged. The alert accessors had no callers, so nothing else moves.

- [ ] **Step 6: Commit**

```bash
git add INA4230.ddsl src/device.rs
git commit -m "refactor(ddsl): index the alert registers by slot, not channel"
```

---

## Task 2: `AlertSlot`

**Files:**
- Create: `src/alert.rs`
- Modify: `src/lib.rs` (add `pub mod alert;` and re-export)
- Test: `tests/core.rs`

- [ ] **Step 1: Write the failing test**

Append to `tests/core.rs`:

```rust
// ── Alert slots ───────────────────────────────────────────────────────────────

#[test]
fn alert_slot_indexes_are_zero_based_and_ordered() {
    assert_eq!(AlertSlot::ALL.len(), 4);
    for (i, slot) in AlertSlot::ALL.into_iter().enumerate() {
        assert_eq!(slot.index(), i);
    }
    assert_eq!(AlertSlot::One.index(), 0);
    assert_eq!(AlertSlot::Four.index(), 3);
}
```

Add `AlertSlot` to the existing `use ina4230::{...}` list at the top of `tests/core.rs`.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test core alert_slot_indexes_are_zero_based_and_ordered`
Expected: FAIL, `cannot find type AlertSlot in this scope`.

- [ ] **Step 3: Create `src/alert.rs`**

```rust
//! Alert slot configuration.
//!
//! The INA4230 has four alert slots. Each slot pairs an `ALERT_CONFIG`
//! register, which selects a condition and the channel to watch, with an
//! `ALERT_LIMIT` register holding the threshold for that condition.
//!
//! A slot is not a channel. Datasheet Table 7-20 describes each limit flag as
//! "independent of channel", and Table 7-8 gives `ALERT_CONFIG` a `CHANNEL`
//! field, so slot 2 may watch channel 4.

use crate::device;

/// One of the four alert slots.
///
/// Distinct from [`crate::Channel`]: a slot selects which of the four
/// `ALERT_CONFIG`/`ALERT_LIMIT` register pairs is addressed, and each pair
/// names its own target channel.
///
/// The datasheet calls these `ALERT1`..`ALERT4` (Table 7-7) and
/// `LIMIT1`..`LIMIT4` (Table 7-9). The variants are bare numbers because the
/// type name already supplies the noun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AlertSlot {
    /// Slot 1: `ALERT_LIMIT` 0x06, `ALERT_CONFIG` 0x07.
    One = 0,
    /// Slot 2: `ALERT_LIMIT` 0x0E, `ALERT_CONFIG` 0x0F.
    Two = 1,
    /// Slot 3: `ALERT_LIMIT` 0x16, `ALERT_CONFIG` 0x17.
    Three = 2,
    /// Slot 4: `ALERT_LIMIT` 0x1E, `ALERT_CONFIG` 0x1F.
    Four = 3,
}

impl AlertSlot {
    /// Every slot, in index order.
    pub const ALL: [Self; 4] = [Self::One, Self::Two, Self::Three, Self::Four];

    /// Zero-based index, usable for array lookup and register striding.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }
}

impl From<AlertSlot> for device::AlertSlot {
    fn from(slot: AlertSlot) -> Self {
        match slot {
            AlertSlot::One => Self::One,
            AlertSlot::Two => Self::Two,
            AlertSlot::Three => Self::Three,
            AlertSlot::Four => Self::Four,
        }
    }
}
```

- [ ] **Step 4: Wire the module into the crate**

In `src/lib.rs`, add `pub mod alert;` next to the existing `pub mod convert;` (line 18 area), and extend the re-export block at lines 20-23:

```rust
pub use crate::alert::AlertSlot;
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --test core alert_slot_indexes_are_zero_based_and_ordered`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/alert.rs src/lib.rs tests/core.rs
git commit -m "feat(alert): add AlertSlot"
```

---

## Task 3: Make the measurement types constructible

**Files:**
- Modify: `src/units.rs:423`, `:450`, `:481`, `:518`, `:555`
- Test: `tests/core.rs`

- [ ] **Step 1: Write the failing test**

Append to `tests/core.rs`:

```rust
#[test]
fn measurement_types_are_constructible_by_callers() {
    assert_eq!(ShuntVoltage::from_nanovolts(-80_000_000).as_nanovolts(), -80_000_000);
    assert_eq!(BusVoltage::from_microvolts(12_000_000).as_microvolts(), 12_000_000);
    assert_eq!(Current::from_nanoamps(6_000_000_000).as_nanoamps(), 6_000_000_000);
    assert_eq!(Power::from_nanowatts(72_000_000_000).as_nanowatts(), 72_000_000_000);
    assert_eq!(Energy::from_nanojoules(259_200_000_000_000).as_nanojoules(), 259_200_000_000_000);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test core measurement_types_are_constructible_by_callers`
Expected: FAIL, `associated function \`from_nanovolts\` is private`.

- [ ] **Step 3: Promote the constructors**

In `src/units.rs`, change `pub(crate) const fn` to `pub const fn` on exactly these five lines: 423 (`ShuntVoltage::from_nanovolts`), 450 (`BusVoltage::from_microvolts`), 481 (`Current::from_nanoamps`), 518 (`Power::from_nanowatts`), 555 (`Energy::from_nanojoules`).

Add to each one's doc comment:

```rust
    /// Construct from a raw value in the type's base unit.
    ///
    /// Values outside the device's measurable range are accepted here and
    /// rejected later, when a threshold is encoded against a specific
    /// calibration — the representable range depends on `AdcRange` or
    /// `CURRENT_LSB`, neither of which is known at this point.
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test core measurement_types_are_constructible_by_callers`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/units.rs tests/core.rs
git commit -m "feat(units): make the measurement newtypes constructible"
```

---

## Task 4: Rounding helpers

**Files:**
- Modify: `src/convert.rs`
- Test: `tests/core.rs`

These are private to `convert`, so they are tested through the encode functions in Tasks 5-7. This task adds them and the module-doc amendment in one commit so the later tasks stay focused.

- [ ] **Step 1: Amend the module doc**

In `src/convert.rs`, replace lines 3-7:

```rust
//! Every decode function in this module is pure and *total*: given a value of
//! the input type it always produces an output, with no failure case and no
//! panic. The encode functions are pure but *not* total — a caller-supplied
//! threshold may not be representable in a 16-bit register at the channel's
//! configured scale, so they return [`Option`]. Purity is the property that
//! matters: it is what keeps this module testable by walking its input domain
//! on the host, with no bus involved.
```

- [ ] **Step 2: Add the helpers**

Append to `src/convert.rs`:

```rust
/// Divide, rounding to nearest, with halves rounding away from zero.
///
/// Matches the rounding `Calibration::new` already uses for `SHUNT_CAL`.
///
/// `d` is always a non-zero LSB constant, so this cannot trap. Callers pass
/// `n` widened from `i32`, so `n + d / 2` cannot overflow `i64`.
const fn div_round_nearest_i64(n: i64, d: i64) -> i64 {
    if (n < 0) == (d < 0) {
        (n + d / 2) / d
    } else {
        (n - d / 2) / d
    }
}

/// Unsigned counterpart of [`div_round_nearest_i64`].
///
/// Written with an explicit remainder rather than `(n + d / 2) / d`, because
/// `encode_power_limit` passes a full-range `u64` and that form overflows near
/// `u64::MAX`. `r < d` and `d` is at most `32 * u32::MAX`, so `r * 2` stays
/// well inside `u64`.
const fn div_round_nearest_u64(n: u64, d: u64) -> u64 {
    let q = n / d;
    let r = n % d;
    if r * 2 >= d { q + 1 } else { q }
}
```

- [ ] **Step 3: Verify it still builds**

Run: `cargo clippy --all-targets --all-features`
Expected: exit 0. The helpers are unused for now; they are `const fn` and private, so add nothing. If clippy warns `dead_code`, that is expected to clear in Task 5 — if you want a clean intermediate commit, combine Tasks 4 and 5.

- [ ] **Step 4: Commit**

```bash
git add src/convert.rs
git commit -m "refactor(convert): add rounding helpers and scope the totality claim"
```

---

## Task 5: `encode_shunt_limit`

**Files:**
- Modify: `src/convert.rs`
- Test: `tests/core.rs`

- [ ] **Step 1: Write the failing tests**

Append to `tests/core.rs`:

```rust
// ── Limit encoding ────────────────────────────────────────────────────────────

#[test]
fn shunt_limit_matches_the_datasheet_worked_example() {
    // Datasheet 7.1.6: -80 mV / 2.5 uV = 32000, two's complement 0x8300.
    let v = ShuntVoltage::from_nanovolts(-80_000_000);
    let raw = convert::encode_shunt_limit(v, AdcRange::Range0).unwrap();
    assert_eq!(raw, -32_000);
    assert_eq!(raw as u16, 0x8300);
}

#[test]
fn shunt_limit_round_trips_every_representable_code() {
    for range in [AdcRange::Range0, AdcRange::Range1] {
        for raw in i16::MIN..=i16::MAX {
            let decoded = convert::decode_shunt_voltage(raw, range);
            assert_eq!(
                convert::encode_shunt_limit(decoded, range),
                Some(raw),
                "raw {raw} at {range:?}"
            );
        }
    }
}

#[test]
fn shunt_limit_rejects_values_beyond_full_scale() {
    // Range0 full scale is -81.92 mV ..= 81.9175 mV.
    assert!(convert::encode_shunt_limit(ShuntVoltage::from_nanovolts(81_920_000), AdcRange::Range0).is_none());
    assert!(convert::encode_shunt_limit(ShuntVoltage::from_nanovolts(-81_922_501), AdcRange::Range0).is_none());
    // The same value is comfortably out of range on the 4x finer Range1.
    assert!(convert::encode_shunt_limit(ShuntVoltage::from_nanovolts(40_000_000), AdcRange::Range1).is_none());
}

#[test]
fn shunt_limit_rounds_to_nearest_away_from_zero() {
    let enc = |nv| convert::encode_shunt_limit(ShuntVoltage::from_nanovolts(nv), AdcRange::Range0);
    assert_eq!(enc(2_500), Some(1)); // exact
    assert_eq!(enc(3_749), Some(1)); // below the halfway point
    assert_eq!(enc(3_750), Some(2)); // exactly halfway, away from zero
    assert_eq!(enc(-3_750), Some(-2));
    assert_eq!(enc(-3_749), Some(-1));
}

proptest! {
    /// Range violation is the only failure mode, and there is no panic for any
    /// `i32` a caller can construct.
    #[test]
    fn shunt_limit_never_panics(nv in i32::MIN..=i32::MAX) {
        for range in [AdcRange::Range0, AdcRange::Range1] {
            let v = ShuntVoltage::from_nanovolts(nv);
            if let Some(raw) = convert::encode_shunt_limit(v, range) {
                // Anything accepted must decode back to within half an LSB.
                let back = convert::decode_shunt_voltage(raw, range).as_nanovolts();
                let lsb = range.shunt_lsb_nv();
                prop_assert!((i64::from(back) - i64::from(nv)).abs() <= i64::from(lsb) / 2 + 1);
            }
        }
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test core shunt_limit`
Expected: FAIL, `cannot find function \`encode_shunt_limit\``.

- [ ] **Step 3: Implement**

Append to `src/convert.rs`:

```rust
/// Encode a shunt-voltage threshold for `ALERT_LIMIT`.
///
/// Inverse of [`decode_shunt_voltage`]. Rounds to nearest, matching
/// `Calibration::new`.
///
/// Returns [`None`] if the threshold does not fit the signed 16-bit register
/// at `range`: -81.92 mV to 81.9175 mV on [`AdcRange::Range0`], -20.48 mV to
/// 20.479375 mV on [`AdcRange::Range1`].
#[must_use]
pub const fn encode_shunt_limit(v: ShuntVoltage, range: AdcRange) -> Option<i16> {
    let raw = div_round_nearest_i64(v.as_nanovolts() as i64, range.shunt_lsb_nv() as i64);
    if raw < i16::MIN as i64 || raw > i16::MAX as i64 {
        return None;
    }
    #[allow(clippy::cast_possible_truncation)]
    Some(raw as i16)
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test core shunt_limit`
Expected: PASS, 4 tests.

- [ ] **Step 5: Commit**

```bash
git add src/convert.rs tests/core.rs
git commit -m "feat(convert): encode shunt-voltage alert limits"
```

---

## Task 6: `encode_bus_limit`

**Files:**
- Modify: `src/convert.rs`
- Test: `tests/core.rs`

- [ ] **Step 1: Write the failing tests**

Append to `tests/core.rs`:

```rust
#[test]
fn bus_limit_round_trips_every_representable_code() {
    // The limit is unsigned 15-bit (datasheet 7.1.5), so only 0..=0x7FFF.
    for raw in 0..=0x7FFFu16 {
        let decoded = convert::decode_bus_voltage(raw);
        assert_eq!(convert::encode_bus_limit(decoded), Some(raw), "raw {raw}");
    }
}

#[test]
fn bus_limit_full_scale_is_the_adc_range() {
    // 32767 * 1.6 mV = 52.4272 V, exactly the range given in datasheet 8.1.1.
    assert_eq!(convert::encode_bus_limit(BusVoltage::from_microvolts(52_427_200)), Some(0x7FFF));
    assert!(convert::encode_bus_limit(BusVoltage::from_microvolts(52_428_001)).is_none());
}

#[test]
fn bus_limit_rounds_to_nearest() {
    let enc = |uv| convert::encode_bus_limit(BusVoltage::from_microvolts(uv));
    assert_eq!(enc(1_600), Some(1));
    assert_eq!(enc(2_399), Some(1));
    assert_eq!(enc(2_400), Some(2));
}

proptest! {
    /// Range violation is the only failure mode, and there is no panic for any
    /// `u32` a caller can construct — including values near `u32::MAX`, where
    /// the intermediate `+ d / 2` must not wrap.
    #[test]
    fn bus_limit_never_panics(uv in 0u32..=u32::MAX) {
        let _ = convert::encode_bus_limit(BusVoltage::from_microvolts(uv));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test core bus_limit`
Expected: FAIL, `cannot find function \`encode_bus_limit\``.

- [ ] **Step 3: Implement**

Append to `src/convert.rs`:

```rust
/// Highest `ALERT_LIMIT` code for a bus-voltage threshold.
///
/// Datasheet 7.1.5 specifies bus limits as unsigned *15*-bit. That is not an
/// inconsistency against the 16-bit result register: `32767 x 1.6 mV =
/// 52.4272 V`, exactly the bus measurement range in 8.1.1, so reserved bit 15
/// covers codes the ADC never produces.
const BUS_LIMIT_MAX: u64 = 0x7FFF;

/// Encode a bus-voltage threshold for `ALERT_LIMIT`.
///
/// Inverse of [`decode_bus_voltage`]. Rounds to nearest.
///
/// Returns [`None`] above 52.4272 V. Needs no calibration: the bus LSB is a
/// fixed 1.6 mV.
#[must_use]
pub const fn encode_bus_limit(v: BusVoltage) -> Option<u16> {
    let raw = div_round_nearest_u64(v.as_microvolts() as u64, BUS_LSB_UV as u64);
    if raw > BUS_LIMIT_MAX {
        return None;
    }
    #[allow(clippy::cast_possible_truncation)]
    Some(raw as u16)
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test core bus_limit`
Expected: PASS, 3 tests.

- [ ] **Step 5: Commit**

```bash
git add src/convert.rs tests/core.rs
git commit -m "feat(convert): encode bus-voltage alert limits"
```

---

## Task 7: `encode_power_limit`

**Files:**
- Modify: `src/convert.rs`
- Test: `tests/core.rs`

- [ ] **Step 1: Write the failing tests**

Append to `tests/core.rs`:

```rust
#[test]
fn power_limit_round_trips_every_representable_code() {
    let cal = example_cal();
    for raw in 0..=u16::MAX {
        let decoded = convert::decode_power(raw, cal);
        assert_eq!(convert::encode_power_limit(decoded, cal), Some(raw), "raw {raw}");
    }
}

#[test]
fn power_limit_rejects_values_beyond_full_scale() {
    let cal = example_cal();
    // Full scale is 65535 * 32 * CURRENT_LSB nW.
    let full_scale = 65_535u64 * 32 * u64::from(cal.current_lsb().as_nanoamps());
    assert!(convert::encode_power_limit(Power::from_nanowatts(full_scale), cal).is_some());
    assert!(convert::encode_power_limit(Power::from_nanowatts(full_scale * 2), cal).is_none());
}

proptest! {
    #[test]
    fn power_limit_never_panics(nw in 0u64..u64::MAX) {
        let _ = convert::encode_power_limit(Power::from_nanowatts(nw), example_cal());
    }
}
```

`example_cal()` already exists in `tests/core.rs`. If it does not, add it, mirroring `tests/shell.rs:29-36`:

```rust
fn example_cal() -> Calibration {
    Calibration::new(
        CurrentLsb::from_nanoamps(500_000).unwrap(),
        ShuntResistance::from_microohms(8_000).unwrap(),
        AdcRange::Range0,
    )
    .unwrap()
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test core power_limit`
Expected: FAIL, `cannot find function \`encode_power_limit\``.

- [ ] **Step 3: Implement**

Append to `src/convert.rs`:

```rust
/// Encode a power threshold for `ALERT_LIMIT`.
///
/// Inverse of [`decode_power`]. Rounds to nearest.
///
/// Returns [`None`] above `65535 x 32 x CURRENT_LSB`. The scale comes from the
/// target channel's calibration, so the same threshold may be representable on
/// one channel and not another.
#[must_use]
pub const fn encode_power_limit(p: Power, cal: Calibration) -> Option<u16> {
    let lsb = POWER_LSB_MULTIPLIER * cal.current_lsb().as_nanoamps() as u64;
    let raw = div_round_nearest_u64(p.as_nanowatts(), lsb);
    if raw > u16::MAX as u64 {
        return None;
    }
    #[allow(clippy::cast_possible_truncation)]
    Some(raw as u16)
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test core power_limit`
Expected: PASS, 3 tests.

- [ ] **Step 5: Commit**

```bash
git add src/convert.rs tests/core.rs
git commit -m "feat(convert): encode power alert limits"
```

---

## Task 8: The `Alert` enum

**Files:**
- Modify: `src/alert.rs`, `src/lib.rs` (re-export)
- Test: `tests/core.rs`

- [ ] **Step 1: Write the failing test**

Append to `tests/core.rs`:

```rust
#[test]
fn alert_variants_map_onto_the_datasheet_encodings() {
    // Datasheet Table 7-8, ALERT_MASK: 1 = SOL, 2 = SUL, 3 = BOL, 4 = BUL, 5 = POL.
    let v = ShuntVoltage::from_nanovolts(1_000_000);
    let b = BusVoltage::from_microvolts(12_000_000);
    let p = Power::from_nanowatts(1_000_000_000);
    assert_eq!(Alert::ShuntOver(v).mask_encoding(), 1);
    assert_eq!(Alert::ShuntUnder(v).mask_encoding(), 2);
    assert_eq!(Alert::BusOver(b).mask_encoding(), 3);
    assert_eq!(Alert::BusUnder(b).mask_encoding(), 4);
    assert_eq!(Alert::PowerOver(p).mask_encoding(), 5);
}

#[test]
fn only_shunt_and_power_alerts_need_calibration() {
    let v = ShuntVoltage::from_nanovolts(1_000_000);
    let b = BusVoltage::from_microvolts(12_000_000);
    let p = Power::from_nanowatts(1_000_000_000);
    assert!(Alert::ShuntOver(v).needs_calibration());
    assert!(Alert::ShuntUnder(v).needs_calibration());
    assert!(Alert::PowerOver(p).needs_calibration());
    assert!(!Alert::BusOver(b).needs_calibration());
    assert!(!Alert::BusUnder(b).needs_calibration());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test core alert_variants`
Expected: FAIL, `cannot find type \`Alert\``.

- [ ] **Step 3: Implement**

Append to `src/alert.rs`:

```rust
use crate::units::{BusVoltage, Power, ShuntVoltage};

/// A condition that asserts the ALERT pin, together with its threshold.
///
/// Each variant carries the threshold in the unit that variant implies, so a
/// bus threshold cannot be paired with a shunt function — there is no way to
/// spell it. Datasheet 7.1.5 makes `ALERT_LIMIT` reinterpret its format
/// according to the selected function, and this is how that reinterpretation
/// is made safe.
///
/// The five variants are `ALERT_MASK` encodings 1 through 5 (Table 7-8).
/// Encodings 0, 6 and 7 are all "reserved, no effect" and are aliases rather
/// than distinct states, so they are not variants: use
/// [`Ina4230::clear_alert`](crate::Ina4230::clear_alert) to disarm a slot, and
/// expect [`None`] from [`Ina4230::alert`](crate::Ina4230::alert).
///
/// Deliberately not `#[non_exhaustive]`: the hardware defines exactly these
/// five functions and cannot grow one, so an exhaustive `match` should keep
/// telling you when you have missed a case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alert {
    /// Shunt voltage rose above the threshold. `ALERT_MASK` = 1 (SOL).
    ShuntOver(ShuntVoltage),
    /// Shunt voltage fell below the threshold. `ALERT_MASK` = 2 (SUL).
    ShuntUnder(ShuntVoltage),
    /// Bus voltage rose above the threshold. `ALERT_MASK` = 3 (BOL).
    BusOver(BusVoltage),
    /// Bus voltage fell below the threshold. `ALERT_MASK` = 4 (BUL).
    BusUnder(BusVoltage),
    /// Power rose above the threshold. `ALERT_MASK` = 5 (POL).
    PowerOver(Power),
}

impl Alert {
    /// This alert's `ALERT_MASK` encoding (datasheet Table 7-8).
    #[must_use]
    pub const fn mask_encoding(self) -> u8 {
        match self {
            Self::ShuntOver(_) => 1,
            Self::ShuntUnder(_) => 2,
            Self::BusOver(_) => 3,
            Self::BusUnder(_) => 4,
            Self::PowerOver(_) => 5,
        }
    }

    /// Whether encoding this alert's threshold requires the target channel's
    /// calibration.
    ///
    /// Shunt thresholds scale with `AdcRange` and power thresholds with
    /// `CURRENT_LSB`, both of which live in the calibration. Bus thresholds
    /// have a fixed 1.6 mV LSB and need nothing.
    #[must_use]
    pub const fn needs_calibration(self) -> bool {
        matches!(self, Self::ShuntOver(_) | Self::ShuntUnder(_) | Self::PowerOver(_))
    }

    /// The generated `ALERT_MASK` enum value for this alert.
    pub(crate) const fn alert_function(self) -> device::AlertFunction {
        match self {
            Self::ShuntOver(_) => device::AlertFunction::ShuntOverLimit,
            Self::ShuntUnder(_) => device::AlertFunction::ShuntUnderLimit,
            Self::BusOver(_) => device::AlertFunction::BusOverLimit,
            Self::BusUnder(_) => device::AlertFunction::BusUnderLimit,
            Self::PowerOver(_) => device::AlertFunction::PowerOverLimit,
        }
    }
}

/// `ALERT_CONFIG.CHANNEL` uses its own generated enum, distinct from the one
/// indexing `channel-regs`, so it needs its own conversion.
impl From<crate::units::Channel> for device::AlertChannel {
    fn from(channel: crate::units::Channel) -> Self {
        match channel {
            crate::units::Channel::Ch1 => Self::Ch1,
            crate::units::Channel::Ch2 => Self::Ch2,
            crate::units::Channel::Ch3 => Self::Ch3,
            crate::units::Channel::Ch4 => Self::Ch4,
        }
    }
}
```

Add `Alert` to the `pub use crate::alert::` line in `src/lib.rs`.

The generated variant names above are verified against `src/device.rs:2162-2175`
(`AlertFunction`) and `:2219-2228` (`AlertChannel`). `AlertChannel` is a
*separate* generated enum from `device::Channel`; `set_channel` on the
`ALERT_CONFIG` writer takes the former, which is why the conversion above is
needed and why `impl From<Channel> for device::Channel` at `src/lib.rs:527`
cannot be reused.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test core alert_variants only_shunt_and_power`
Expected: PASS, 2 tests.

- [ ] **Step 5: Commit**

```bash
git add src/alert.rs src/lib.rs tests/core.rs
git commit -m "feat(alert): add the Alert enum"
```

---

## Task 9: `LimitOutOfRange` and `#[non_exhaustive]`

**Files:**
- Modify: `src/lib.rs:35-57`
- Test: `tests/shell.rs`

- [ ] **Step 1: Write the failing test**

Append to `tests/shell.rs`:

```rust
#[test]
fn limit_out_of_range_reports_the_slot() {
    let e: Ina4230Error<ErrorKind> = Ina4230Error::LimitOutOfRange(AlertSlot::Two);
    assert_eq!(e, Ina4230Error::LimitOutOfRange(AlertSlot::Two));
    assert_ne!(e, Ina4230Error::LimitOutOfRange(AlertSlot::Three));
}
```

Add `AlertSlot` to the `use ina4230::{...}` list in `tests/shell.rs`.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test shell limit_out_of_range_reports_the_slot`
Expected: FAIL, `no variant named \`LimitOutOfRange\``.

- [ ] **Step 3: Implement**

In `src/lib.rs`, add the attribute and the variant:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Ina4230Error<I2cError> {
    /// An error occurred on the I²C bus.
    Bus(I2cError),
    /// A shunt-voltage, current, power, or energy reading was requested on a
    /// channel that has not been calibrated.
    ///
    /// Shunt voltage is included because its scale depends on the channel's
    /// configured [`AdcRange`], which is part of the calibration.
    ///
    /// Detected before any bus traffic is generated.
    NotCalibrated(Channel),
    /// An alert threshold does not fit `ALERT_LIMIT` at the target channel's
    /// scale.
    ///
    /// The representable range depends on the alert kind: +/-81.92 mV or
    /// +/-20.48 mV for shunt thresholds depending on [`AdcRange`], 0 to
    /// 52.4272 V for bus thresholds, and `65535 x 32 x CURRENT_LSB` for power
    /// thresholds.
    ///
    /// Detected before any bus traffic is generated.
    LimitOutOfRange(AlertSlot),
}
```

Extend the `sensor::Error` impl at lines 50-57 with the new arm:

```rust
            Self::LimitOutOfRange(_) => sensor::ErrorKind::InvalidInput,
```

`InvalidInput` is documented upstream as "the sensor was configured with
invalid input", which is precisely this case. Verified against
`embedded-sensors-hal-0.1.0/src/sensor.rs:33-46`, whose variants are
`Peripheral`, `NotReady`, `Saturated`, `InvalidInput` and `Other`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test`
Expected: PASS. The `#[non_exhaustive]` attribute does not affect in-crate matches, and `tests/` is an external consumer that only constructs and compares these values, so no wildcard arm is needed yet.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs tests/shell.rs
git commit -m "feat!: add Ina4230Error::LimitOutOfRange and seal the enum"
```

---

## Task 10: The alert cache and `alert()`

**Files:**
- Modify: `src/lib.rs:252-279` (struct and `new`), add getter near `calibration()` at `:480`
- Test: `tests/shell.rs`

- [ ] **Step 1: Write the failing test**

Append to `tests/shell.rs`:

```rust
#[tokio::test]
async fn alert_slots_start_empty_and_cost_no_bus_traffic_to_read() {
    let dev = sensor(&[]);
    for slot in AlertSlot::ALL {
        assert_eq!(dev.alert(slot), None);
    }
    dev.release().done();
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test shell alert_slots_start_empty`
Expected: FAIL, `no method named \`alert\``.

- [ ] **Step 3: Implement**

In `src/lib.rs`, add the field to the struct after `calibration`:

```rust
    /// Per-slot alert configuration, as programmed through this driver.
    ///
    /// Carries the same caveat as [`Ina4230::calibration`]: a record of this
    /// driver's own successful writes, not a reflection of the device.
    ///
    /// Cached because invalidation needs it. When a channel is recalibrated,
    /// [`Ina4230::calibrate`] has to know which slots target that channel and
    /// which of those are shunt or power alerts. Without this, that question
    /// would cost four register reads on every calibration.
    alerts: [Option<(Channel, Alert)>; 4],
```

In `new`, add `alerts: [None; 4],` next to `calibration: [None; 4],`.

Add the getter beside `calibration()`:

```rust
    /// The alert configured for `slot`, as programmed through this driver.
    ///
    /// [`None`] means the slot is disarmed — `ALERT_MASK` is one of the
    /// reserved no-effect encodings.
    ///
    /// Reads the driver's cache, so it costs no bus traffic and cannot see a
    /// power cycle, an EN-pin toggle, a General Call reset, or another
    /// controller on the bus.
    #[must_use]
    pub const fn alert(&self, slot: AlertSlot) -> Option<(Channel, Alert)> {
        self.alerts[slot.index()]
    }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test shell alert_slots_start_empty`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs tests/shell.rs
git commit -m "feat(alert): cache per-slot alert configuration"
```

---

## Task 11: `set_alert` and `clear_alert`

**Files:**
- Modify: `src/lib.rs`
- Test: `tests/shell.rs`

- [ ] **Step 1: Write the failing tests**

Append to `tests/shell.rs`:

```rust
#[tokio::test]
async fn set_alert_writes_limit_then_config_at_the_slot_addresses() {
    // Slot 2: ALERT_LIMIT 0x0E, ALERT_CONFIG 0x0F.
    // 12 V / 1.6 mV = 7500 = 0x1D4C. Watching Ch3 => CHANNEL = 0b10,
    // BusOver => ALERT_MASK = 3, so ALERT_CONFIG = 0b10_011 = 0x13.
    let mut dev = sensor(&[
        Transaction::write(ADDR, vec![0x0E, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x0F, 0x00, 0x13]),
    ]);

    let alert = Alert::BusOver(BusVoltage::from_microvolts(12_000_000));
    dev.set_alert(AlertSlot::Two, Channel::Ch3, alert).await.unwrap();
    assert_eq!(dev.alert(AlertSlot::Two), Some((Channel::Ch3, alert)));
    dev.release().done();
}

#[tokio::test]
async fn shunt_and_power_alerts_on_an_uncalibrated_channel_do_not_touch_the_bus() {
    // No expectations: any transaction panics.
    let mut dev = sensor(&[]);
    let ch = Channel::Ch1;
    let v = ShuntVoltage::from_nanovolts(1_000_000);
    let p = Power::from_nanowatts(1_000_000_000);

    assert_eq!(
        dev.set_alert(AlertSlot::One, ch, Alert::ShuntOver(v)).await,
        Err(Ina4230Error::NotCalibrated(ch))
    );
    assert_eq!(
        dev.set_alert(AlertSlot::One, ch, Alert::PowerOver(p)).await,
        Err(Ina4230Error::NotCalibrated(ch))
    );
    assert_eq!(dev.alert(AlertSlot::One), None);
    dev.release().done();
}

#[tokio::test]
async fn an_unrepresentable_threshold_does_not_touch_the_bus() {
    let mut dev = sensor(&[]);
    let too_big = BusVoltage::from_microvolts(60_000_000); // > 52.4272 V
    assert_eq!(
        dev.set_alert(AlertSlot::Four, Channel::Ch1, Alert::BusOver(too_big)).await,
        Err(Ina4230Error::LimitOutOfRange(AlertSlot::Four))
    );
    assert_eq!(dev.alert(AlertSlot::Four), None);
    dev.release().done();
}

#[tokio::test]
async fn clear_alert_zeroes_the_mask_and_drops_the_cache_entry() {
    let mut dev = sensor(&[
        Transaction::write(ADDR, vec![0x0E, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x0F, 0x00, 0x13]),
        Transaction::write(ADDR, vec![0x0F, 0x00, 0x00]),
    ]);

    let alert = Alert::BusOver(BusVoltage::from_microvolts(12_000_000));
    dev.set_alert(AlertSlot::Two, Channel::Ch3, alert).await.unwrap();
    dev.clear_alert(AlertSlot::Two).await.unwrap();
    assert_eq!(dev.alert(AlertSlot::Two), None);
    dev.release().done();
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test shell set_alert clear_alert`
Expected: FAIL, `no method named \`set_alert\``.

- [ ] **Step 3: Implement**

Add to the `impl Ina4230` block in `src/lib.rs`:

```rust
    /// Arm `slot` to watch `channel` for `alert`.
    ///
    /// Writes `ALERT_LIMIT` and then `ALERT_CONFIG`, so the threshold is in
    /// place before the condition is enabled.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::NotCalibrated`] if `alert` is a shunt or power
    /// condition and `channel` has no calibration — those thresholds scale with
    /// [`AdcRange`] and `CURRENT_LSB`. Bus conditions need no calibration.
    ///
    /// Returns [`Ina4230Error::LimitOutOfRange`] if the threshold does not fit
    /// `ALERT_LIMIT` at that scale.
    ///
    /// Both are detected before any bus traffic is generated.
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. This operation
    /// is not transactional: if the `ALERT_LIMIT` write lands and the
    /// `ALERT_CONFIG` write does not, the slot stays disarmed with a new
    /// threshold loaded, and the cache is not updated.
    pub async fn set_alert(
        &mut self,
        slot: AlertSlot,
        channel: Channel,
        alert: Alert,
    ) -> Result<(), Ina4230Error<I2c::Error>> {
        let raw = self.encode_alert(slot, channel, alert)?;

        self.device
            .alert_regs(slot.into())
            .alert_limit()
            .write_async(|w| w.set_limit(raw))
            .await?;

        self.device
            .alert_regs(slot.into())
            .alert_config()
            .write_async(|w| {
                w.set_channel(channel.into());
                w.set_alert_mask(alert.alert_function());
            })
            .await?;

        self.alerts[slot.index()] = Some((channel, alert));
        Ok(())
    }

    /// Disarm `slot`.
    ///
    /// Writes `ALERT_MASK = 0`, one of the reserved no-effect encodings
    /// (datasheet Table 7-8). `ALERT_LIMIT` is left alone; it has no effect
    /// while the mask is clear.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. The cache
    /// entry is dropped only once the write lands.
    pub async fn clear_alert(&mut self, slot: AlertSlot) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device
            .alert_regs(slot.into())
            .alert_config()
            .write_async(|w| w.set_alert_mask(device::AlertFunction::NoEffect))
            .await?;

        self.alerts[slot.index()] = None;
        Ok(())
    }

    /// Encode an alert threshold, failing before any bus traffic.
    fn encode_alert(
        &self,
        slot: AlertSlot,
        channel: Channel,
        alert: Alert,
    ) -> Result<u16, Ina4230Error<I2c::Error>> {
        let encoded = match alert {
            Alert::ShuntOver(v) | Alert::ShuntUnder(v) => {
                let cal = self.require_calibration(channel)?;
                #[allow(clippy::cast_sign_loss)]
                convert::encode_shunt_limit(v, cal.adc_range()).map(|raw| raw as u16)
            }
            Alert::BusOver(v) | Alert::BusUnder(v) => convert::encode_bus_limit(v),
            Alert::PowerOver(p) => {
                let cal = self.require_calibration(channel)?;
                convert::encode_power_limit(p, cal)
            }
        };
        encoded.ok_or(Ina4230Error::LimitOutOfRange(slot))
    }
```

Add `Alert` and `AlertSlot` to the `use crate::alert::{...}` import at the top of `src/lib.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test shell`
Expected: PASS, all shell tests including the four new ones.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs tests/shell.rs
git commit -m "feat(alert): set and clear alert slots"
```

---

## Task 12: Invalidate alert slots on recalibration

**Files:**
- Modify: `src/lib.rs:436-461` (`calibrate`), `:481-...` (`calibrate_all`), `:291` (`reset`)
- Test: `tests/shell.rs`

This is the task the design exists for. Read section 5 of the spec before starting.

- [ ] **Step 1: Write the failing tests**

Append to `tests/shell.rs`:

```rust
#[tokio::test]
async fn recalibration_disarms_shunt_alerts_and_leaves_bus_alerts_armed() {
    let cal = example_cal();
    let mut dev = sensor(&[
        // calibrate Ch1 the first time
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        // arm slot 1 with a shunt alert on Ch1: 1 mV / 2.5 uV = 400 = 0x0190
        Transaction::write(ADDR, vec![0x06, 0x01, 0x90]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x01]),
        // arm slot 2 with a bus alert on Ch1: CHANNEL = 0b00, BOL = 3
        Transaction::write(ADDR, vec![0x0E, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x0F, 0x00, 0x03]),
        // recalibrate Ch1: slot 1 is disarmed first, slot 2 is not touched
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]),
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
    ]);

    dev.calibrate(Channel::Ch1, cal).await.unwrap();

    let shunt = Alert::ShuntOver(ShuntVoltage::from_nanovolts(1_000_000));
    let bus = Alert::BusOver(BusVoltage::from_microvolts(12_000_000));
    dev.set_alert(AlertSlot::One, Channel::Ch1, shunt).await.unwrap();
    dev.set_alert(AlertSlot::Two, Channel::Ch1, bus).await.unwrap();

    dev.calibrate(Channel::Ch1, cal).await.unwrap();

    assert_eq!(dev.alert(AlertSlot::One), None, "shunt alert must be disarmed");
    assert_eq!(
        dev.alert(AlertSlot::Two),
        Some((Channel::Ch1, bus)),
        "bus alert is absolutely scaled and must survive"
    );
    dev.release().done();
}

#[tokio::test]
async fn recalibration_with_no_alerts_emits_no_extra_writes() {
    let cal = example_cal();
    // Exactly the three transactions calibrate has always issued.
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
    ]);
    dev.calibrate(Channel::Ch1, cal).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn a_failed_disarm_aborts_before_the_range_is_rewritten() {
    let cal = example_cal();
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x06, 0x01, 0x90]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x01]),
        // the disarm fails; no CONFIG2 write may follow
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]).with_error(ErrorKind::Other),
    ]);

    dev.calibrate(Channel::Ch1, cal).await.unwrap();
    let shunt = Alert::ShuntOver(ShuntVoltage::from_nanovolts(1_000_000));
    dev.set_alert(AlertSlot::One, Channel::Ch1, shunt).await.unwrap();

    assert!(dev.calibrate(Channel::Ch1, cal).await.is_err());
    assert_eq!(
        dev.alert(AlertSlot::One),
        Some((Channel::Ch1, shunt)),
        "the disarm did not land, so the cache must still show it armed"
    );
    dev.release().done();
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test shell recalibration a_failed_disarm`
Expected: FAIL. The first test fails on an unexpected transaction, because `calibrate` does not disarm anything yet.

- [ ] **Step 3: Implement**

Add the helper to `impl Ina4230` in `src/lib.rs`:

```rust
    /// Disarm every slot whose threshold is scaled by `channel`'s calibration.
    ///
    /// Shunt and power thresholds are stored as raw counts scaled by
    /// [`AdcRange`] and `CURRENT_LSB`. Recalibrating moves that scale, so a
    /// slot left armed would enforce a different threshold than the one it was
    /// given — for the two ADC ranges, a factor of four. Bus thresholds are
    /// absolute and are left alone.
    ///
    /// Called before `CONFIG2.RANGE` moves, so that a failure here leaves every
    /// still-armed slot armed against the scale it was programmed for.
    async fn disarm_scaled_alerts(&mut self, channel: Channel) -> Result<(), Ina4230Error<I2c::Error>> {
        for slot in AlertSlot::ALL {
            match self.alerts[slot.index()] {
                Some((ch, alert)) if ch == channel && alert.needs_calibration() => {
                    self.clear_alert(slot).await?;
                }
                _ => {}
            }
        }
        Ok(())
    }
```

In `calibrate`, insert before the existing cache invalidation at line 443:

```rust
        // Disarm anything whose scale is about to move, before it moves. A
        // failure here returns with CONFIG2.RANGE untouched, so whatever is
        // still armed is still armed against the scale it was given.
        self.disarm_scaled_alerts(channel).await?;
```

In `calibrate_all`, insert the same loop across every channel before `self.calibration = [None; 4];`:

```rust
        // One CONFIG2 write moves all four range bits, so every channel's
        // scaled alerts are about to become wrong.
        for channel in Channel::ALL {
            self.disarm_scaled_alerts(channel).await?;
        }
```

In `reset`, add beside the existing cache clear:

```rust
        self.alerts = [None; 4];
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test`
Expected: PASS, everything.

- [ ] **Step 5: Verify the tests actually catch the regression**

Temporarily comment out the `self.disarm_scaled_alerts(channel).await?;` line in `calibrate`, then run:

Run: `cargo test --test shell recalibration_disarms_shunt_alerts_and_leaves_bus_alerts_armed`
Expected: FAIL. Restore the line and confirm it passes again. This mirrors the practice recorded in `44093f1`.

- [ ] **Step 6: Commit**

```bash
git add src/lib.rs tests/shell.rs
git commit -m "fix(alert): disarm scaled alert slots before recalibrating a channel"
```

---

## Task 13: `Flags::limit_alert`

**Files:**
- Modify: `src/lib.rs` (the `impl Flags` block around line 144)
- Test: `tests/shell.rs`

- [ ] **Step 1: Write the failing test**

Append to `tests/shell.rs`:

```rust
#[tokio::test]
async fn limit_alert_is_indexed_by_slot() {
    // FLAGS 0x22, bit 13 = LIMIT2_ALERT.
    let mut dev = sensor(&[Transaction::write_read(ADDR, vec![0x22], vec![0x20, 0x00])]);
    let flags = dev.read_flags().await.unwrap();
    assert!(!flags.limit_alert(AlertSlot::One));
    assert!(flags.limit_alert(AlertSlot::Two));
    assert!(!flags.limit_alert(AlertSlot::Three));
    assert!(!flags.limit_alert(AlertSlot::Four));
    assert_eq!(flags.limit_alerts(), [false, true, false, false]);
    dev.release().done();
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test shell limit_alert_is_indexed_by_slot`
Expected: FAIL, `no method named \`limit_alert\``.

- [ ] **Step 3: Implement**

Add to `impl Flags` in `src/lib.rs`, directly above the existing `limit_alerts`:

```rust
    /// Whether `slot`'s alert limit has been exceeded.
    ///
    /// The channel this refers to is whichever one the slot's `ALERT_CONFIG`
    /// selects (datasheet Table 7-8), not one derived from the slot number —
    /// Table 7-20 describes these flags as independent of channel.
    ///
    /// See [`Flags::limit_alerts`] for why a limit flag need not agree with the
    /// averaged measurement registers.
    #[must_use]
    pub const fn limit_alert(self, slot: AlertSlot) -> bool {
        self.limit_alerts[slot.index()]
    }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test shell limit_alert_is_indexed_by_slot`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs tests/shell.rs
git commit -m "feat(alert): index limit flags by slot"
```

---

## Task 14: Documentation

**Files:**
- Modify: `README.md:318-347` ("Not yet implemented")
- Modify: `README.md` (new "Alerts" section after "Reading flags")

- [ ] **Step 1: Remove the implemented entries from "Not yet implemented"**

Delete the **Alert configuration** and **Alert limits** bullets. Keep the `CONFIG2` alert-behaviour bullet, the `CONFIG1` bullet, the `ACC_RST` bullet, and the SMBus/General Call bullet. Replace the closing paragraph about `Flags::limit_alerts()` with a pointer to the new section.

- [ ] **Step 2: Add the "Alerts" section**

Insert after the "Reading flags" section:

````markdown
## Alerts

The device has four alert slots. Each pairs a condition and a target channel
with a threshold, and asserts the ALERT pin when the condition is met.

A slot is not a channel: any slot can watch any channel, which is why
`AlertSlot` and `Channel` are separate types.

```rust,ignore
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
```

Each `Alert` variant carries its threshold in the unit that variant implies, so
a bus threshold cannot be paired with a shunt condition — there is no way to
write it.

**Shunt and power thresholds need the target channel to be calibrated.** Their
scale comes from the channel's `AdcRange` and `CURRENT_LSB`; `set_alert`
returns `NotCalibrated` otherwise, before touching the bus. Bus thresholds have
a fixed 1.6 mV LSB and work on an uncalibrated channel.

**Recalibrating a channel disarms its shunt and power alerts.** `ALERT_LIMIT`
holds raw counts, and the two ADC ranges differ by a factor of four, so a slot
left armed across a recalibration would silently enforce a different threshold
than the one it was given. `calibrate` and `calibrate_all` disarm the affected
slots before the range moves; set them again afterwards. Bus alerts are
absolutely scaled and survive untouched.

Thresholds round to the nearest LSB, so a value read back through
`Ina4230::alert` is the one you supplied, not the one the device holds.
````

- [ ] **Step 3: Verify the docs build and the doctests still pass**

Run:
```bash
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc --no-deps --all-features
cargo test --doc
```
Expected: both exit 0.

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs(readme): document alert slots, limits and invalidation"
```

---

## Final verification

- [ ] **Run the full CI set locally**

```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc --no-deps --all-features
cargo deny check
ddc build rust -s INA4230.ddsl -o .device-check.rs --rust-defmt-feature=defmt && rustfmt --edition 2024 .device-check.rs && diff .device-check.rs src/device.rs && rm .device-check.rs
```

All must exit 0. The last one is what `device-driver-pregen-check` runs.

The regeneration check writes its temporary file **inside the repository**, not
to `/tmp` or `$env:TEMP`. `rustfmt` discovers `rustfmt.toml` by walking up from
the file it is formatting, and this repo sets `max_width = 120`. Formatting a
file outside the tree silently uses the 100-column default and produces a
spurious diff against a perfectly good `src/device.rs`.

- [ ] **Confirm the semver break is the expected one**

The release PR will report a breaking change from `Ina4230Error` gaining a
variant and `#[non_exhaustive]`. That is expected and drives `0.1 -> 0.2`. If
`semver_check` reports anything else, investigate before merging.

- [ ] **Open the pull request**

Only now, per the repository's workflow: the alert work is reviewed as a single
pull request once the implementation is complete.
