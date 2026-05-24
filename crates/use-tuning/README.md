# use-tuning

Tuning metadata primitives for `RustUse`.

## Experimental

use-tuning is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_tuning::{ConcertPitchStandard, ReferencePitch};

let reference = ReferencePitch::new(440.0)?;

assert_eq!(reference.value(), 440.0);
assert_eq!(ConcertPitchStandard::A440.as_str(), "a440");
# Ok::<(), use_tuning::TuningError>(())
~~~

## Scope

- Tuning systems, temperament kinds, reference pitch metadata, reference notes, concert pitch standards, cents, tuning ratios, microtonal divisions, and equal temperament divisions.

## Relationship to use-acoustics

`use-tuning` models tuning metadata only. `use-acoustics` owns physical sound calculations and `use-wave` owns wave primitives.

## Non-goals

- Wave calculation, acoustic frequency modeling, tuning engines, pitch detection, synthesis, or playback.

## License

Licensed under either Apache-2.0 or MIT.
