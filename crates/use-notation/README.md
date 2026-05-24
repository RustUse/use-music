# use-notation

Notation metadata primitives for `RustUse`.

## Experimental

use-notation is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_notation::{ClefKind, ScorePartName, StaffLineCount};

let part = ScorePartName::new("Violin I")?;
let lines = StaffLineCount::new(5)?;

assert_eq!(part.as_str(), "Violin I");
assert_eq!(lines.value(), 5);
assert_eq!(ClefKind::Treble.as_str(), "treble");
# Ok::<(), use_notation::NotationError>(())
~~~

## Scope

- Clefs, staff kinds, staff-line counts, score part names, measure positions, notation symbols, notation formats, music-document kinds, repeat marks, and endings.

## Relationship to use-acoustics

`use-music` keeps symbolic music metadata separate from `use-acoustics`, which owns physical sound calculations such as frequency, wavelength, decibels, sound pressure, sound intensity, period, angular frequency, and speed-of-sound approximations.

## Non-goals

- Rendering notation, editing scores, parsing full `MusicXML`, `ABC`, `LilyPond`, MuseScore, Guitar Pro, or DAW files.

## License

Licensed under either Apache-2.0 or MIT.
