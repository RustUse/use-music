# use-pitch

Symbolic pitch metadata primitives for `RustUse`.

## Experimental

use-pitch is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_pitch::{MidiNoteNumber, PitchClassNumber, PitchRegister};

let note = MidiNoteNumber::new(60)?;
let class = PitchClassNumber::new(0)?;

assert_eq!(note.pitch_class().value(), class.value());
assert_eq!(note.octave(), 4);
assert_eq!(PitchRegister::OneLine.as_str(), "one-line");
# Ok::<(), use_pitch::PitchError>(())
~~~

## Scope

- Pitch classes, pitch class numbers, pitch numbers, MIDI note-number metadata, pitch names, spellings, and register labels.
- Small helpers for pitch class and octave extraction from MIDI-style note numbers.

## Relationship to use-acoustics

`use-pitch` models symbolic and numeric pitch metadata only. Physical frequency and wave calculations belong in `use-acoustics` and `use-wave`.

## Non-goals

- Frequency, wavelength, pitch detection, tuning engines, audio playback, synthesis, or MIDI device I/O.

## License

Licensed under either Apache-2.0 or MIT.
