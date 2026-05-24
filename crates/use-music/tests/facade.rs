use use_music::{
    articulation, chord, dynamics, interval, key, meter, midi, mode, notation, note, pitch, rhythm,
    scale, tempo, tuning,
};

#[test]
fn facade_reexports_every_child_crate() -> Result<(), Box<dyn std::error::Error>> {
    let note_name = note::NoteName::new("C#4")?;
    let pitch_class = pitch::PitchClassNumber::new(0)?;
    let interval_number = interval::IntervalNumber::new(3)?;
    let scale_name = scale::ScaleName::new("C major")?;
    let mode_name = mode::ModeName::new("D dorian")?;
    let key_signature = key::KeySignature::new(0, key::KeyMode::Major)?;
    let chord_symbol = chord::ChordSymbol::new("Cmaj7")?;
    let duration = rhythm::NoteDuration::new(rhythm::DurationValue::Quarter);
    let time_signature = meter::TimeSignature::new(4, 4)?;
    let tempo = tempo::BeatsPerMinute::new(120.0)?;
    let dynamic = dynamics::DynamicMarking::Mf;
    let articulation = articulation::ArticulationKind::Staccato;
    let clef = notation::ClefKind::Treble;
    let midi_channel = midi::MidiChannel::new(1)?;
    let reference_pitch = tuning::ReferencePitch::new(440.0)?;

    assert_eq!(note_name.as_str(), "C#4");
    assert_eq!(pitch_class.value(), 0);
    assert_eq!(interval_number.value(), 3);
    assert_eq!(scale_name.as_str(), "C major");
    assert_eq!(mode_name.as_str(), "D dorian");
    assert!(key_signature.is_natural_key());
    assert_eq!(chord_symbol.as_str(), "Cmaj7");
    assert_eq!(duration.value(), rhythm::DurationValue::Quarter);
    assert!(time_signature.is_common_time_like());
    assert_eq!(tempo.value(), 120.0);
    assert_eq!(dynamic.as_str(), "mf");
    assert_eq!(articulation.as_str(), "staccato");
    assert_eq!(clef.as_str(), "treble");
    assert_eq!(midi_channel.value(), 1);
    assert_eq!(reference_pitch.value(), 440.0);
    Ok(())
}
