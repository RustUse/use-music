#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        MidiNoteNumber, PitchClass, PitchClassNumber, PitchError, PitchName, PitchNumber,
        PitchRegister, PitchSpelling, is_valid_midi_note_number,
    };
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PitchClassNumber(u8);

impl PitchClassNumber {
    pub fn new(value: u8) -> Result<Self, PitchError> {
        if value > 11 {
            return Err(PitchError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for PitchClassNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for PitchClassNumber {
    type Err = PitchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| PitchError::InvalidFormat)?;
        Self::new(parsed)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PitchNumber(i16);

impl PitchNumber {
    pub const fn new(value: i16) -> Self {
        Self(value)
    }

    pub const fn value(self) -> i16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MidiNoteNumber(u8);

impl MidiNoteNumber {
    pub fn new(value: u8) -> Result<Self, PitchError> {
        if value > 127 {
            return Err(PitchError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }

    pub fn pitch_class(self) -> PitchClassNumber {
        PitchClassNumber::new(self.0 % 12).expect("MIDI pitch class is always in 0..=11")
    }

    pub const fn octave(self) -> i8 {
        (self.0 / 12).cast_signed() - 1
    }
}

impl fmt::Display for MidiNoteNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for MidiNoteNumber {
    type Err = PitchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| PitchError::InvalidFormat)?;
        Self::new(parsed)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PitchName(String);

pub type PitchSpelling = PitchName;
pub type PitchClass = PitchClassNumber;

impl PitchName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, PitchError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            return Err(PitchError::Empty);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PitchName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PitchName {
    type Err = PitchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for PitchName {
    type Error = PitchError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PitchRegister {
    SubContra,
    Contra,
    Great,
    Small,
    OneLine,
    TwoLine,
    ThreeLine,
    FourLine,
    FiveLine,
    Unknown,
}

impl PitchRegister {
    pub const ALL: &'static [Self] = &[
        Self::SubContra,
        Self::Contra,
        Self::Great,
        Self::Small,
        Self::OneLine,
        Self::TwoLine,
        Self::ThreeLine,
        Self::FourLine,
        Self::FiveLine,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SubContra => "sub-contra",
            Self::Contra => "contra",
            Self::Great => "great",
            Self::Small => "small",
            Self::OneLine => "one-line",
            Self::TwoLine => "two-line",
            Self::ThreeLine => "three-line",
            Self::FourLine => "four-line",
            Self::FiveLine => "five-line",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for PitchRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PitchRegister {
    type Err = PitchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value
            .trim()
            .to_ascii_lowercase()
            .replace(['_', ' '], "-")
            .as_str()
        {
            "sub-contra" => Ok(Self::SubContra),
            "contra" => Ok(Self::Contra),
            "great" => Ok(Self::Great),
            "small" => Ok(Self::Small),
            "one-line" => Ok(Self::OneLine),
            "two-line" => Ok(Self::TwoLine),
            "three-line" => Ok(Self::ThreeLine),
            "four-line" => Ok(Self::FourLine),
            "five-line" => Ok(Self::FiveLine),
            "unknown" => Ok(Self::Unknown),
            _ => Err(PitchError::UnknownLabel),
        }
    }
}

pub const fn is_valid_midi_note_number(value: u8) -> bool {
    value <= 127
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PitchError {
    Empty,
    InvalidFormat,
    OutOfRange,
    UnknownLabel,
}

impl fmt::Display for PitchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pitch metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("pitch metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("pitch metadata value is out of range"),
            Self::UnknownLabel => formatter.write_str("unknown pitch metadata label"),
        }
    }
}

impl Error for PitchError {}

#[cfg(test)]
#[allow(
    unused_imports,
    clippy::unnecessary_wraps,
    clippy::assertions_on_constants
)]
mod tests {
    use super::{
        MidiNoteNumber, PitchClassNumber, PitchError, PitchName, PitchRegister,
        is_valid_midi_note_number,
    };

    #[test]
    fn validates_pitch_classes_and_midi_numbers() -> Result<(), PitchError> {
        assert_eq!(PitchClassNumber::new(11)?.value(), 11);
        assert_eq!(PitchClassNumber::new(12), Err(PitchError::OutOfRange));

        let middle_c = MidiNoteNumber::new(60)?;
        assert_eq!(middle_c.pitch_class().value(), 0);
        assert_eq!(middle_c.octave(), 4);
        assert!(is_valid_midi_note_number(127));
        Ok(())
    }

    #[test]
    fn validates_pitch_names_and_registers() -> Result<(), PitchError> {
        let name = PitchName::new(" C#4 ")?;
        assert_eq!(name.as_str(), "C#4");
        assert_eq!("one line".parse::<PitchRegister>()?, PitchRegister::OneLine);
        Ok(())
    }
}
