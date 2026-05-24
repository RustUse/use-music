# use-chord

Chord metadata primitives for `RustUse`.

## Experimental

use-chord is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_chord::{ChordQuality, ChordSymbol, TriadKind};

let symbol = ChordSymbol::new("Cmaj7")?;

assert_eq!(symbol.as_str(), "Cmaj7");
assert_eq!(ChordQuality::Major.as_str(), "major");
assert_eq!(TriadKind::MAJOR, TriadKind::Major);
# Ok::<(), use_chord::ChordError>(())
~~~

## Scope

- Chord names, symbols, quality labels, extensions, alterations, inversions, tone roles, voicing kinds, triads, and seventh chord kinds.
- Common triad and seventh-chord constants.

## Relationship to use-acoustics

`use-music` keeps symbolic music metadata separate from `use-acoustics`, which owns physical sound calculations such as frequency, wavelength, decibels, sound pressure, sound intensity, period, angular frequency, and speed-of-sound approximations.

## Non-goals

- Full chord-symbol parsing, harmonic analysis, voicing generation, MIDI output, or synthesis.

## License

Licensed under either Apache-2.0 or MIT.
