# use-rhythm

Rhythm metadata primitives for `RustUse`.

## Experimental

use-rhythm is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_rhythm::{DottedDuration, DurationValue, NoteDuration};

let note = NoteDuration::new(DurationValue::Eighth);
let dotted = DottedDuration::new(DurationValue::Half, 1);

assert!(note.is_shorter_than_quarter_like());
assert_eq!(dotted.dot_count(), 1);
# Ok::<(), use_rhythm::RhythmError>(())
~~~

## Scope

- Symbolic note durations, rest durations, duration values, beat divisions, tuplets, dotted durations, rhythmic positions, and syncopation labels.

## Relationship to use-acoustics

`use-rhythm` models symbolic music durations only. `use-time` owns general time primitives, and `use-acoustics` owns physical sound calculations.

## Non-goals

- Sequencing, scheduling, playback, clocking, or general-purpose time duration utilities.

## License

Licensed under either Apache-2.0 or MIT.
