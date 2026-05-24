# use-articulation

Articulation metadata primitives for `RustUse`.

## Experimental

use-articulation is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_articulation::{ArticulationKind, PerformanceTechnique};

assert_eq!(ArticulationKind::Staccato.as_str(), "staccato");
assert_eq!("palm mute".parse::<PerformanceTechnique>()?, PerformanceTechnique::PalmMute);
# Ok::<(), use_articulation::ArticulationError>(())
~~~

## Scope

- Articulation, ornament, phrase-mark, slur, tie, and performance-technique labels.

## Relationship to use-acoustics

`use-music` keeps symbolic music metadata separate from `use-acoustics`, which owns physical sound calculations such as frequency, wavelength, decibels, sound pressure, sound intensity, period, angular frequency, and speed-of-sound approximations.

## Non-goals

- Rendering articulations, synthesizing performance, sequencing, or playback.

## License

Licensed under either Apache-2.0 or MIT.
