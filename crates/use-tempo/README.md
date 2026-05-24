# use-tempo

Tempo metadata primitives for `RustUse`.

## Experimental

use-tempo is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_tempo::{BeatsPerMinute, TempoMarking};

let bpm = BeatsPerMinute::new(120.0)?;

assert_eq!(bpm.value(), 120.0);
assert_eq!(TempoMarking::Allegro.as_str(), "allegro");
# Ok::<(), use_tempo::TempoError>(())
~~~

## Scope

- Finite positive BPM values, tempo markings, ranges, tempo changes, tempo-map points, metronome marks, and rubato labels.

## Relationship to use-acoustics

`use-tempo` models musical tempo metadata only. `use-time` owns general timing utilities, and `use-acoustics` owns physical sound calculations.

## Non-goals

- Clocks, schedulers, playback, sequencing, transport, or timing engines.

## License

Licensed under either Apache-2.0 or MIT.
