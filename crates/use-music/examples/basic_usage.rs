use use_music::{BeatsPerMinute, ChordSymbol, NoteName, TimeSignature};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let note = NoteName::new("C#4")?;
    let chord = ChordSymbol::new("Cmaj7")?;
    let meter = TimeSignature::new(4, 4)?;
    let tempo = BeatsPerMinute::new(120.0)?;

    assert_eq!(note.as_str(), "C#4");
    assert_eq!(chord.as_str(), "Cmaj7");
    assert!(meter.is_common_time_like());
    assert_eq!(tempo.value(), 120.0);
    Ok(())
}
