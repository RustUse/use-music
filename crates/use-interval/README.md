# use-music-interval

Musical interval metadata primitives for `RustUse`.

## Experimental

use-music-interval is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_interval::{IntervalName, SimpleInterval};

let third = SimpleInterval::major_third();
let name = IntervalName::new("major third")?;

assert_eq!(third.semitones().value(), 4);
assert_eq!(name.as_str(), "major third");
# Ok::<(), use_interval::IntervalError>(())
~~~

## Scope

- Musical interval quality, number, names, semitone distances, diatonic step distances, and direction labels.
- Common simple interval constructors such as major third, perfect fifth, tritone, and octave.

## Relationship to use-acoustics

`use-music-interval` models symbolic music intervals. It does not calculate frequencies or wave ratios; physical sound calculations belong in `use-acoustics` and generic interval/range semantics belong in `use-math`.

## Non-goals

- Generic mathematical intervals, harmonic analysis, chord-scale analysis, audio intervals, or frequency-ratio calculation.

## License

Licensed under either Apache-2.0 or MIT.
