# use-meter

Meter metadata primitives for `RustUse`.

## Experimental

use-meter is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_meter::TimeSignature;

let signature = TimeSignature::new(4, 4)?;

assert!(signature.is_common_time_like());
assert!(signature.is_simple());
# Ok::<(), use_meter::MeterError>(())
~~~

## Scope

- Time signatures, meter kinds, beat units, beats per measure, measure numbers, barlines, pickup measures, and metric accents.

## Relationship to use-acoustics

`use-meter` models music meter metadata only. `use-time` owns general time primitives, and `use-acoustics` owns physical sound calculations.

## Non-goals

- Playback clocks, schedulers, sequencing, or general-purpose time calculations.

## License

Licensed under either Apache-2.0 or MIT.
