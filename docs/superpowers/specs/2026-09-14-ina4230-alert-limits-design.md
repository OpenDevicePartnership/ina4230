# Alert limits and alert configuration — design

- **Date:** 2026-09-14
- **Status:** Approved, not yet implemented
- **Scope:** `ALERT_CONFIG1..4`, `ALERT_LIMIT1..4`, and the `CONFIG2` ALERT-pin
  behaviour bits
- **Datasheet:** SBOSAD4, June 2024

## Motivation

Alert configuration is the largest documented gap in the driver. The README
lists it first under "Not yet implemented" and states why it was deferred:
the limit register reinterprets its format according to the alert function it
is paired with, and representing that safely is the design problem.

The driver is at `0.1.0` with `semver_check = true`, so breaking changes are
detected and cost a `0.x -> 0.(x+1)` bump. The goal here is to settle the
public alert surface while that is still cheap, even if parts of it are
implemented later.

## Non-goals

- `CONFIG1` timing and operating mode (`AVG`, `VBUSCT`, `VSHCT`, `MODE`).
- Energy accumulator reset (`CONFIG2.ACC_RST`). Tracked separately; it is the
  subject of the energy-overflow recovery gap documented in the README.
- SMBus Alert Response and General Call reset.
- High-speed I2C, which the README records as out of scope: participation is a
  property of the `embedded-hal-async` implementation, not of this crate.

## 1. Slot model

### Decision

Introduce `AlertSlot` (`S1`..`S4`) as the index for the four alert register
pairs. Move `alert-limit` and `alert-config` out of
`channel-regs[channel stride 8]` into a new `alert-regs[slot stride 8]` block
based at `0x06` and `0x07`.

### Rationale

The four alert register pairs are *not* per-channel. Datasheet Table 7-20
describes each `LIMITn_ALERT` flag as "independent of channel", and Table 7-8
gives `ALERT_CONFIG` a `CHANNEL` field in bits 4-3 selecting which channel the
slot watches. Alert slot 2 can watch channel 4.

The current DDSL places these registers inside the per-channel block, so the
generated accessor is reached as `channel_regs(channel).alert_config()`. That
names an alert slot a channel and invites exactly the transposition class of
bug already fixed once in this driver for the I2C address table.

Addresses and stride are unchanged — `0x06/0x0E/0x16/0x1E` and
`0x07/0x0F/0x17/0x1F`, stride 8 — so this is purely a renaming of what the
index means. `src/device.rs` regenerates to equivalent register access;
`device-driver-pregen-check` verifies the committed file matches.

### Consequence

`Flags` gains `limit_alert(AlertSlot) -> bool` alongside the existing
`limit_alerts() -> [bool; 4]`, matching the shape of `energy_overflow(Channel)`.
The rustdoc merged in #12 already states in prose that these flags belong to a
register rather than a channel; `AlertSlot` makes that statement expressible in
the type system.

## 2. Public types

### Decision

```rust
pub enum Alert {
    ShuntOver(ShuntVoltage),
    ShuntUnder(ShuntVoltage),
    BusOver(BusVoltage),
    BusUnder(BusVoltage),
    PowerOver(Power),
}

async fn set_alert(&mut self, slot: AlertSlot, channel: Channel, alert: Alert)
    -> Result<(), Ina4230Error<E>>;
async fn clear_alert(&mut self, slot: AlertSlot) -> Result<(), Ina4230Error<E>>;
fn alert(&self, slot: AlertSlot) -> Option<(Channel, Alert)>;
```

### Rationale

Each variant carries the threshold in the unit that variant implies, so a bus
threshold cannot be paired with a shunt function — there is no way to express
it. This is the direct answer to the reinterpretation problem the README
identifies.

The five variants map one-to-one onto `ALERT_MASK` encodings 1-5. Encodings 0,
6 and 7 are all "reserved, no effect" (Table 7-8) and are aliases rather than
distinct states, so they never become a variant: writing is `clear_alert`, and
reading them back yields `None`. This matches `no-effect: default 0` in the
existing DDSL enum.

The channel is passed separately because it is genuinely orthogonal to the
function — any slot may watch any channel.

### Caching

The driver caches the `slot -> (Channel, Alert)` mapping in
`[Option<(Channel, Alert)>; 4]`, alongside the existing calibration cache, and
`alert(slot)` reads from it.

This is required rather than convenient: invalidation (section 5) has to know
which slots target a recalibrated channel and which of those are shunt or power
alerts. Without a cache that question costs four register reads on every
`calibrate` call, on a path that is otherwise two writes.

The cache carries the same caveat the calibration cache already documents: it
is a record of this driver's own successful writes, not a reflection of the
device. It cannot see a power cycle, an EN-pin toggle, a General Call reset, or
another controller on the bus.

Note the asymmetry with `AlertPinConfig` in section 6, which is deliberately
not cached. The rule is to cache only what another operation needs in order to
be correct. Alert slots qualify because invalidation depends on them; pin
config does not, so it is read from the device on demand and cannot go stale.

Alternatives rejected:

- A separate `AlertFunction` enum plus an `AlertThreshold` union, validated at
  the call site. Faithful to the register's two fields, but manufactures a
  runtime mismatch error that this design makes unrepresentable, for identical
  expressiveness.
- A typestate builder. More discoverable and more room for per-function options,
  but a larger surface to freeze at 1.0 for options that do not exist.

## 3. Threshold construction

### Decision

Promote the five `pub(crate)` constructors to `pub`:
`ShuntVoltage::from_nanovolts`, `BusVoltage::from_microvolts`,
`Current::from_nanoamps`, `Power::from_nanowatts`, `Energy::from_nanojoules`.

### Rationale

The measurement newtypes are currently output-only — callers can observe them
but not construct them. `Alert` requires callers to express a threshold, so
those constructors have to be reachable.

Separate threshold types were considered. The usual justification is
validate-at-construction, and that only half-applies: a bus limit is absolutely
bounded and could be checked at construction, but shunt and power limits depend
on the target channel's `AdcRange` and `CurrentLsb`, which are not known until
`set_alert`. Two of three validate late regardless, so three extra types would
buy one early check.

Making the constructors public reclassifies these types from "measurements the
device produced" to "quantities". Nothing in the crate relies on the former.
It also removes a real limitation: a downstream crate currently cannot fabricate
a `Current` to unit-test its own code against this API.

## 4. Encoding and fallibility

### Decision

Three encode functions join `convert.rs` as inverses of the existing decodes.

| Limit | Formula | Representable range | Calibration needed |
| --- | --- | --- | --- |
| Shunt | `nv / range.shunt_lsb_nv()` | +/-81.92 mV (Range0), +/-20.48 mV (Range1) | `AdcRange` |
| Bus | `uv / 1600` | 0 to 52.4272 V, unsigned 15-bit | none |
| Power | `nw / (32 * current_lsb_na)` | 0 to `65535 * 32 * CURRENT_LSB` | `CurrentLsb` |

Rounding is to nearest, matching `Calibration::new`, which already rounds to
nearest rather than truncating.

A threshold that does not fit the register returns a new variant,
`Ina4230Error::LimitOutOfRange(AlertSlot)`, mirroring the single-field shape of
the existing `NotCalibrated(Channel)`. Shunt and power alerts on an
uncalibrated channel return `Ina4230Error::NotCalibrated(channel)`, detected
before any bus traffic, as the read path already does. Bus alerts succeed on an
uncalibrated channel.

### Rationale

The bus limit being unsigned 15-bit against a 16-bit result register looks like
an inconsistency but is not: `32767 * 1.6 mV = 52.4272 V`, exactly the ADC range
given in section 8.1.1. Reserved bit 15 covers codes the ADC never produces.
This should be stated in the rustdoc, because it otherwise reads as a datasheet
error.

`convert.rs` currently documents its functions as "pure and *total*". Encoding
cannot be total. The encode functions still belong there — purity is the
property that matters, since it preserves the pure-core/imperative-shell split
and allows exhaustive host testing. The module doc changes to say the decode
direction is total and the encode direction is fallible, with the reason.
Totality was a consequence of every function being a decode, not a goal.

## 5. Invalidation on recalibration

### Decision

`calibrate` and `calibrate_all` disarm cached shunt and power alert slots
targeting an affected channel, by writing `ALERT_MASK = 0`, **before** changing
`CONFIG2.RANGE`. Bus alerts are left untouched.

Sequence for `calibrate(channel, cal)`:

1. From the alert cache, collect the shunt and power slots targeting `channel`.
2. For each, write `ALERT_MASK = 0` and drop that slot's cache entry once its
   own write lands. On failure, return immediately.
3. Drop the cached calibration for `channel`.
4. Write `CONFIG2.RANGE`.
5. Write `SHUNT_CAL`.
6. Repopulate the calibration cache on success.

The cache is read in step 1 and cleared per-slot in step 2, not up front — it
is the only record of which slots need disarming, so clearing it first would
discard the information the step depends on.

Returning early in step 2 is safe in a way that is worth being explicit about:
`CONFIG2.RANGE` has not been touched, so any slot still armed is still armed
against the scale it was programmed for. The operation aborts with the device
consistent rather than half-rescaled. Dropping each cache entry only after its
own write lands keeps the cache accurate if the loop stops partway.

`calibrate_all` does the same across all four channels, since its single
`CONFIG2` write moves every range bit. `reset()` drops the whole alert cache;
the device clears the registers itself.

### Rationale

`ALERT_LIMIT` holds raw counts. What a count means comes from the target
channel's calibration — `AdcRange` for shunt limits, a 4x difference between the
two ranges, and `CurrentLsb` for power limits. Nothing in the device couples
them, so recalibrating silently changes what an armed threshold means.

Dropping only the cached entry would be useless: the device would still arm a
mis-scaled threshold on the ALERT pin, which is the hazard being closed.
Invalidation must reach the register.

Ordering matters for the same reason it does in `44093f1`, which moved
calibration invalidation before the first write: between a range change and a
later disarm there is a window where the device enforces the old threshold
against the new scale. Disarming first removes the window.

Only shunt and power limits depend on calibration. Bus limits are absolute, so
clearing them would be gratuitous.

If no shunt or power slots are cached, this costs zero extra bus traffic, so
the common path — calibrate, then configure alerts — is unaffected.

## 6. ALERT pin behaviour

### Decision

```rust
pub enum AlertLatch { Transparent, Latched }
pub enum AlertPolarity { ActiveLow, ActiveHigh }

#[derive(Default)]
pub struct AlertPinConfig {
    pub polarity: AlertPolarity,    // ActiveLow
    pub latch: AlertLatch,          // Transparent
    pub on_conversion_ready: bool,  // CNVR_MASK, false
    pub on_energy_overflow: bool,   // ENOF_MASK, false
}

async fn set_alert_pin_config(&mut self, cfg: AlertPinConfig) -> Result<(), Ina4230Error<E>>;
async fn read_alert_pin_config(&mut self) -> Result<AlertPinConfig, Ina4230Error<E>>;
```

### Rationale

Enum where the two states are named peers; bool where the field enables the
thing its own name describes. `polarity` and `latch` are peers —
`latch: false` does not convey that the pin deasserts when the condition
clears, whereas `AlertLatch::Transparent` does. The two mask bits are additive
sources, genuinely enable/disable of what the field already names, and a
two-variant enum per bit would add a public type that says nothing the field
name does not.

`Default` maps exactly onto the `CONFIG2` reset value `0x0000`: transparent,
active low, both sources off. It is the literal power-on state rather than an
invented convention.

Written with a single `modify_async` on `CONFIG2` touching only bits 7-4, the
same read-modify-write pattern `calibrate` uses. RMW is required because
`CONFIG2` also holds `RANGE` in bits 3-0. It is safe across `RST` (bit 15) and
`ACC_RST` (bits 11-8) because both are write-1, self-clearing bits that read
back 0, so writing the read value back is a no-op — no risk of re-triggering a
reset or clearing an energy accumulator.

Read straight from the device and deliberately **not** cached. `calibration` is
cached because every measurement read needs it to scale a value without a bus
round-trip; pin config is never on that path, so a cache would buy nothing and
inherit the coherence problem the calibration cache already documents. Because
nothing is cached, `reset()` needs no invalidation for it — the device returns
to `Default` by itself.

No interaction with `calibrate`: disjoint bits, both RMW. One interaction with
`read_flags`: under `AlertLatch::Latched` a latched alert holds until `FLAGS` is
read and the condition clears (section 7.1.2, bit 5), which is the behaviour the
rustdoc merged in #12 describes. The two need cross-links.

## 7. Exhaustiveness attributes

### Decision

- `Ina4230Error` becomes `#[non_exhaustive]`.
- `Alert`, `AlertPinConfig`, `AlertLatch` and `AlertPolarity` do **not**.

### Rationale

`#[non_exhaustive]` belongs on types whose domain is open. `Ina4230Error` is
open — new failure modes are discoverable, and this design already adds one.
Marking it now costs one breaking change instead of one per future variant.

The alert types are closed by the hardware. `ALERT_MASK` has exactly five
usable encodings and `CONFIG2` exactly four alert bits; neither can grow,
because the silicon is fixed. Marking them would be actively harmful: on a
struct it forbids external literal construction entirely, including
`..Default::default()`, and on an enum it forces a wildcard arm in every
external `match`, suppressing the missed-variant error that makes the enum
worth having.

## 8. Testing

- **Exhaustive round-trip.** For each `AdcRange`, walk all 65,536 `i16` codes:
  decode then encode, asserting the original raw value. Same for the 32,768
  valid bus codes. Proves the encoder is an exact inverse wherever a value is
  representable. The existing suite already walks 131,072 shunt codes in well
  under a millisecond.
- **Datasheet vector.** Section 7.1.6 supplies one: -80 mV / 2.5 uV = 32000,
  two's complement `0x8300`. Derived from TI's worked example rather than from
  this implementation.
- **Rounding.** Concrete cases pinning round-to-nearest, including exact `.5`
  boundaries.
- **Rejection.** proptest that out-of-range thresholds always error and never
  panic, and that range violations are the only failure mode.
- **Mock behaviour.** A shunt or power alert on an uncalibrated channel issues
  zero bus transactions, following `uncalibrated_reads_do_not_touch_the_bus`; a
  bus alert on the same channel succeeds; recalibration disarms shunt and power
  slots on that channel and leaves bus slots armed; recalibration with no cached
  alerts emits no extra writes. Each checked against a reverted fix to confirm
  it catches the regression.
- **Abort before rescaling.** A bus error during the disarm loop returns before
  `CONFIG2.RANGE` is written, so a slot that is still armed is still armed
  against the scale it was programmed for. The mock asserts that no `CONFIG2`
  write is issued, and that the cache retains exactly the slots whose disarm did
  not land.
- **Slot is not channel.** A slot deliberately targeting a channel other than
  its own index, guarding the transposition the DDSL restructure removes. This
  is the `address_pin_order_is_not_symmetric` of this feature.

## 9. Implementation order

1. **Slots and limits.** DDSL restructure, `AlertSlot`, `Alert`, the alert
   cache, public constructors, encode functions, `Ina4230Error` changes,
   `Flags::limit_alert`, invalidation, tests. This carries all of the risk and
   all of the breakage.
2. **ALERT pin config.** `AlertPinConfig` and its two enums, one `CONFIG2`
   read-modify-write, cross-links to `read_flags`.

Step 2 adds a struct and new methods, neither of which is a breaking change, so
deferring it costs nothing in semver terms and keeps the first change focused.

## Semver impact

Breaking, driving `0.1 -> 0.2`:

- `Ina4230Error` gains a variant and `#[non_exhaustive]`.
- Alert register accessors change index type from `Channel` to `AlertSlot`.

Non-breaking: the new constructors, `Alert`, `AlertSlot`, `set_alert`,
`clear_alert`, `alert`, `Flags::limit_alert`, and everything in step 2.

`semver_check = true` will flag the breaking items on the release PR, which is
the expected and correct outcome.
