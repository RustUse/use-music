# use-dynamics

Dynamic marking metadata primitives for `RustUse`.

## Experimental

use-dynamics is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_dynamics::{DynamicMarking, ExpressionMarking};

let expression = ExpressionMarking::new("dolce")?;

assert_eq!(expression.as_str(), "dolce");
assert_eq!(DynamicMarking::Mf.as_str(), "mf");
# Ok::<(), use_dynamics::DynamicsError>(())
~~~

## Scope

- Musical dynamic markings, dynamic levels, changes, hairpins, accents, and expression markings.

## Relationship to use-acoustics

`use-dynamics` models musical loudness intent only. `use-acoustics` owns decibels, sound pressure, and sound intensity calculations.

## Non-goals

- Acoustic sound pressure, decibels, loudness calculation, audio playback, or synthesis.

## License

Licensed under either Apache-2.0 or MIT.
