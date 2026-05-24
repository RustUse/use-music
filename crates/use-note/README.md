# use-note

Musical note spelling metadata primitives for `RustUse`.

## Experimental

use-note is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_note::{Accidental, NoteLetter, NoteName, Octave};

let note = NoteName::new("C#4")?;
let octave = Octave::new(4)?;

assert_eq!(note.as_str(), "C#4");
assert_eq!(note.letter(), NoteLetter::C);
assert_eq!(note.accidental(), Accidental::Sharp);
assert_eq!(note.octave(), Some(octave));
# Ok::<(), use_note::NoteError>(())
~~~

## Scope

- Note letters, accidentals, octave values, note names, note spellings, and scientific pitch notation.
- Lightweight parsing for common spellings such as `C`, `C#`, `Db`, `F##`, `Gbb`, `A4`, and `C#4`.

## Relationship to use-acoustics

`use-note` models symbolic note spellings only. Physical frequency and sound calculations belong in `use-acoustics` and wave behavior belongs in `use-wave`.

## Non-goals

- Calculating frequency, wavelength, pitch detection, audio playback, MIDI I/O, or synthesis.

## License

Licensed under either Apache-2.0 or MIT.
