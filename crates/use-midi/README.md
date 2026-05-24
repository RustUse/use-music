# use-midi

MIDI metadata primitives for `RustUse`.

## Experimental

use-midi is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_midi::{MidiChannel, MidiMessageKind, MidiNoteNumber};

let channel = MidiChannel::new(1)?;
let note = MidiNoteNumber::new(60)?;

assert_eq!(channel.value(), 1);
assert_eq!(note.value(), 60);
assert_eq!(MidiMessageKind::NoteOn.as_str(), "note-on");
# Ok::<(), use_midi::MidiError>(())
~~~

## Scope

- MIDI versions, channels, note numbers, velocities, controller numbers, program numbers, message kinds, event kinds, device labels, port names, profiles, properties, and UMP labels.
- MIDI 2.0 metadata labels such as UMP, profiles, per-note expression, and property exchange.

## Relationship to use-acoustics

`use-midi` models protocol metadata only. `use-acoustics` owns physical sound calculations and `use-wave` owns wave primitives.

## Non-goals

- Opening MIDI ports, sending or receiving MIDI messages, depending on MIDI device libraries, playback, sequencing, or synthesis.

## License

Licensed under either Apache-2.0 or MIT.
