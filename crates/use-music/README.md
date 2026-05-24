# use-music

Facade over the focused `RustUse` symbolic music primitive crates.

## Experimental

`use-music` is experimental while the workspace remains below `0.3.0`.

## Example

```rust
use use_music::{BeatsPerMinute, ChordSymbol, NoteName, TimeSignature};

let note = NoteName::new("C#4")?;
let chord = ChordSymbol::new("Cmaj7")?;
let meter = TimeSignature::new(4, 4)?;
let tempo = BeatsPerMinute::new(120.0)?;

assert_eq!(note.as_str(), "C#4");
assert_eq!(chord.as_str(), "Cmaj7");
assert!(meter.is_common_time_like());
assert_eq!(tempo.value(), 120.0);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Re-exports all focused `use-music` crates.
- Keeps implementation logic in the child crates.

## Non-goals

- Audio generation, DSP, synthesis, playback, MIDI device I/O, notation rendering, or full parser behavior.

## License

Licensed under either Apache-2.0 or MIT.
