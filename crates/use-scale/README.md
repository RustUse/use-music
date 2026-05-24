# use-scale

Scale pattern metadata primitives for `RustUse`.

## Experimental

use-scale is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_scale::{ScaleKind, ScaleName, ScalePattern};

let name = ScaleName::new("C major")?;
let pattern = ScalePattern::new([2, 2, 1, 2, 2, 2, 1])?;

assert_eq!(name.as_str(), "C major");
assert_eq!(ScaleKind::Major.as_str(), "major");
assert!(pattern.is_heptatonic());
# Ok::<(), use_scale::ScaleError>(())
~~~

## Scope

- Scale names, kinds, degrees, tone counts, and semitone step patterns.
- Small helpers for heptatonic, pentatonic, and chromatic classification.

## Relationship to use-acoustics

`use-music` keeps symbolic music metadata separate from `use-acoustics`, which owns physical sound calculations such as frequency, wavelength, decibels, sound pressure, sound intensity, period, angular frequency, and speed-of-sound approximations.

## Non-goals

- Audio generation, scale playback, full harmonic analysis, or automatic composition.

## License

Licensed under either Apache-2.0 or MIT.
