#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        Accidental, EnharmonicSpelling, NaturalNote, NoteClass, NoteError, NoteLetter, NoteName,
        NoteSpelling, Octave, ScientificPitchNotation,
    };
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NoteLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl NoteLetter {
    pub const ALL: &'static [Self] = &[
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
        }
    }
}

impl fmt::Display for NoteLetter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NoteLetter {
    type Err = NoteError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_uppercase().as_str() {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            _ => Err(NoteError::InvalidFormat),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Accidental {
    Natural,
    Sharp,
    Flat,
    DoubleSharp,
    DoubleFlat,
}

impl Accidental {
    pub const ALL: &'static [Self] = &[
        Self::Natural,
        Self::Sharp,
        Self::Flat,
        Self::DoubleSharp,
        Self::DoubleFlat,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Natural => "",
            Self::Sharp => "#",
            Self::Flat => "b",
            Self::DoubleSharp => "##",
            Self::DoubleFlat => "bb",
        }
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Octave(i8);

impl Octave {
    pub const MIN: i8 = -1;
    pub const MAX: i8 = 10;

    pub fn new(value: i8) -> Result<Self, NoteError> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(NoteError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> i8 {
        self.0
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for Octave {
    type Err = NoteError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<i8>()
            .map_err(|_| NoteError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<i8> for Octave {
    type Error = NoteError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NoteName {
    value: String,
    letter: NoteLetter,
    accidental: Accidental,
    octave: Option<Octave>,
}

pub type NoteSpelling = NoteName;
pub type ScientificPitchNotation = NoteName;
pub type NaturalNote = NoteLetter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnharmonicSpelling {
    preferred: NoteName,
    alternate: NoteName,
}

impl EnharmonicSpelling {
    pub const fn new(preferred: NoteName, alternate: NoteName) -> Self {
        Self {
            preferred,
            alternate,
        }
    }

    pub const fn preferred(&self) -> &NoteName {
        &self.preferred
    }

    pub const fn alternate(&self) -> &NoteName {
        &self.alternate
    }
}

impl NoteName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, NoteError> {
        value.as_ref().parse()
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn value(&self) -> &str {
        self.as_str()
    }

    pub const fn letter(&self) -> NoteLetter {
        self.letter
    }

    pub const fn accidental(&self) -> Accidental {
        self.accidental
    }

    pub const fn octave(&self) -> Option<Octave> {
        self.octave
    }

    pub const fn note_class(&self) -> NoteClass {
        match self.accidental {
            Accidental::Natural => NoteClass::Natural,
            Accidental::Sharp | Accidental::Flat => NoteClass::Chromatic,
            Accidental::DoubleSharp | Accidental::DoubleFlat => NoteClass::Enharmonic,
        }
    }
}

impl AsRef<str> for NoteName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for NoteName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NoteName {
    type Err = NoteError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(NoteError::Empty);
        }

        let mut chars = trimmed.chars();
        let letter_char = chars.next().ok_or(NoteError::Empty)?;
        let letter = NoteLetter::from_str(&letter_char.to_string())?;
        let rest: String = chars.collect();
        let (accidental_text, octave_text) = split_accidental_and_octave(&rest)?;
        let accidental = parse_accidental(accidental_text)?;
        let octave = if octave_text.is_empty() {
            None
        } else {
            Some(octave_text.parse::<Octave>()?)
        };

        Ok(Self {
            value: format!(
                "{}{}{}",
                letter,
                accidental,
                octave.map_or_else(String::new, |value| value.to_string())
            ),
            letter,
            accidental,
            octave,
        })
    }
}

impl TryFrom<&str> for NoteName {
    type Error = NoteError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NoteClass {
    Natural,
    Chromatic,
    Enharmonic,
}

fn split_accidental_and_octave(value: &str) -> Result<(&str, &str), NoteError> {
    let octave_start = value
        .char_indices()
        .find_map(|(index, character)| {
            (character == '-' || character.is_ascii_digit()).then_some(index)
        })
        .unwrap_or(value.len());
    let accidental = &value[..octave_start];
    let octave = &value[octave_start..];

    if !octave.is_empty() && octave == "-" {
        return Err(NoteError::InvalidFormat);
    }

    Ok((accidental, octave))
}

fn parse_accidental(value: &str) -> Result<Accidental, NoteError> {
    match value {
        "" => Ok(Accidental::Natural),
        "#" => Ok(Accidental::Sharp),
        "b" => Ok(Accidental::Flat),
        "##" | "x" => Ok(Accidental::DoubleSharp),
        "bb" => Ok(Accidental::DoubleFlat),
        _ => Err(NoteError::InvalidFormat),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NoteError {
    Empty,
    InvalidFormat,
    OutOfRange,
}

impl fmt::Display for NoteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("note spelling cannot be empty"),
            Self::InvalidFormat => formatter.write_str("note spelling has an invalid format"),
            Self::OutOfRange => formatter.write_str("octave is out of range"),
        }
    }
}

impl Error for NoteError {}

#[cfg(test)]
#[allow(
    unused_imports,
    clippy::unnecessary_wraps,
    clippy::assertions_on_constants
)]
mod tests {
    use super::{Accidental, NoteClass, NoteError, NoteLetter, NoteName, Octave};

    #[test]
    fn parses_common_note_spellings() -> Result<(), NoteError> {
        for spelling in ["C", "C#", "Db", "F##", "Gbb", "A4", "C#4"] {
            assert_eq!(spelling.parse::<NoteName>()?.to_string(), spelling);
        }

        let note = NoteName::new(" C#4 ")?;
        assert_eq!(note.letter(), NoteLetter::C);
        assert_eq!(note.accidental(), Accidental::Sharp);
        assert_eq!(note.octave(), Some(Octave::new(4)?));
        assert_eq!(note.note_class(), NoteClass::Chromatic);
        Ok(())
    }

    #[test]
    fn validates_octave_range() {
        assert_eq!(Octave::new(-1).map(Octave::value), Ok(-1));
        assert_eq!(Octave::new(10).map(Octave::value), Ok(10));
        assert_eq!(Octave::new(-2), Err(NoteError::OutOfRange));
        assert_eq!(Octave::new(11), Err(NoteError::OutOfRange));
    }

    #[test]
    fn rejects_invalid_spellings() {
        assert_eq!(NoteName::new(""), Err(NoteError::Empty));
        assert_eq!(NoteName::new("H"), Err(NoteError::InvalidFormat));
        assert_eq!(NoteName::new("C###"), Err(NoteError::InvalidFormat));
        assert_eq!(NoteName::new("C11"), Err(NoteError::OutOfRange));
    }
}
