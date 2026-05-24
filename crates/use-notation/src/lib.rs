#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        ClefKind, EndingKind, MeasurePosition, MusicDocumentKind, NotationError, NotationFormat,
        NotationSymbolKind, RepeatMarkKind, ScorePartName, StaffKind, StaffLineCount,
    };
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScorePartName(String);

impl ScorePartName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, NotationError> {
        non_empty_text(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &str {
        self.as_str()
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ScorePartName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ScorePartName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ScorePartName {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ScorePartName {
    type Error = NotationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StaffLineCount(u8);

impl StaffLineCount {
    pub fn new(value: u8) -> Result<Self, NotationError> {
        if !(1..=16).contains(&value) {
            return Err(NotationError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for StaffLineCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for StaffLineCount {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| NotationError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for StaffLineCount {
    type Error = NotationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeasurePosition(u8);

impl MeasurePosition {
    pub fn new(value: u8) -> Result<Self, NotationError> {
        if !(1..=255).contains(&value) {
            return Err(NotationError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for MeasurePosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for MeasurePosition {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| NotationError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for MeasurePosition {
    type Error = NotationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClefKind {
    Treble,
    Bass,
    Alto,
    Tenor,
    Soprano,
    MezzoSoprano,
    Baritone,
    Percussion,
    Tab,
    Neutral,
}

impl ClefKind {
    pub const ALL: &'static [Self] = &[
        Self::Treble,
        Self::Bass,
        Self::Alto,
        Self::Tenor,
        Self::Soprano,
        Self::MezzoSoprano,
        Self::Baritone,
        Self::Percussion,
        Self::Tab,
        Self::Neutral,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Treble => "treble",
            Self::Bass => "bass",
            Self::Alto => "alto",
            Self::Tenor => "tenor",
            Self::Soprano => "soprano",
            Self::MezzoSoprano => "mezzo-soprano",
            Self::Baritone => "baritone",
            Self::Percussion => "percussion",
            Self::Tab => "tab",
            Self::Neutral => "neutral",
        }
    }
}

impl fmt::Display for ClefKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ClefKind {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "treble" => Ok(Self::Treble),
            "bass" => Ok(Self::Bass),
            "alto" => Ok(Self::Alto),
            "tenor" => Ok(Self::Tenor),
            "soprano" => Ok(Self::Soprano),
            "mezzo-soprano" => Ok(Self::MezzoSoprano),
            "baritone" => Ok(Self::Baritone),
            "percussion" => Ok(Self::Percussion),
            "tab" => Ok(Self::Tab),
            "neutral" => Ok(Self::Neutral),
            _ => Err(NotationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StaffKind {
    Standard,
    Grand,
    Percussion,
    Tablature,
    LeadSheet,
    ChordChart,
}

impl StaffKind {
    pub const ALL: &'static [Self] = &[
        Self::Standard,
        Self::Grand,
        Self::Percussion,
        Self::Tablature,
        Self::LeadSheet,
        Self::ChordChart,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Grand => "grand",
            Self::Percussion => "percussion",
            Self::Tablature => "tablature",
            Self::LeadSheet => "lead-sheet",
            Self::ChordChart => "chord-chart",
        }
    }
}

impl fmt::Display for StaffKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for StaffKind {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "standard" => Ok(Self::Standard),
            "grand" => Ok(Self::Grand),
            "percussion" => Ok(Self::Percussion),
            "tablature" => Ok(Self::Tablature),
            "lead-sheet" => Ok(Self::LeadSheet),
            "chord-chart" => Ok(Self::ChordChart),
            _ => Err(NotationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NotationSymbolKind {
    Note,
    Rest,
    Clef,
    KeySignature,
    TimeSignature,
    Dynamic,
    Articulation,
    Repeat,
    Text,
    Custom,
}

impl NotationSymbolKind {
    pub const ALL: &'static [Self] = &[
        Self::Note,
        Self::Rest,
        Self::Clef,
        Self::KeySignature,
        Self::TimeSignature,
        Self::Dynamic,
        Self::Articulation,
        Self::Repeat,
        Self::Text,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Rest => "rest",
            Self::Clef => "clef",
            Self::KeySignature => "key-signature",
            Self::TimeSignature => "time-signature",
            Self::Dynamic => "dynamic",
            Self::Articulation => "articulation",
            Self::Repeat => "repeat",
            Self::Text => "text",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for NotationSymbolKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NotationSymbolKind {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "note" => Ok(Self::Note),
            "rest" => Ok(Self::Rest),
            "clef" => Ok(Self::Clef),
            "key-signature" => Ok(Self::KeySignature),
            "time-signature" => Ok(Self::TimeSignature),
            "dynamic" => Ok(Self::Dynamic),
            "articulation" => Ok(Self::Articulation),
            "repeat" => Ok(Self::Repeat),
            "text" => Ok(Self::Text),
            "custom" => Ok(Self::Custom),
            _ => Err(NotationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NotationFormat {
    MusicXml,
    Midi,
    Abc,
    LilyPond,
    MuseScore,
    GuitarPro,
    PlainText,
    Custom,
}

impl NotationFormat {
    pub const ALL: &'static [Self] = &[
        Self::MusicXml,
        Self::Midi,
        Self::Abc,
        Self::LilyPond,
        Self::MuseScore,
        Self::GuitarPro,
        Self::PlainText,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MusicXml => "music-xml",
            Self::Midi => "midi",
            Self::Abc => "abc",
            Self::LilyPond => "lily-pond",
            Self::MuseScore => "muse-score",
            Self::GuitarPro => "guitar-pro",
            Self::PlainText => "plain-text",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for NotationFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NotationFormat {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "music-xml" => Ok(Self::MusicXml),
            "midi" => Ok(Self::Midi),
            "abc" => Ok(Self::Abc),
            "lily-pond" => Ok(Self::LilyPond),
            "muse-score" => Ok(Self::MuseScore),
            "guitar-pro" => Ok(Self::GuitarPro),
            "plain-text" => Ok(Self::PlainText),
            "custom" => Ok(Self::Custom),
            _ => Err(NotationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MusicDocumentKind {
    Score,
    Part,
    LeadSheet,
    ChordChart,
    Tablature,
    FakeBook,
    Exercise,
    Custom,
}

impl MusicDocumentKind {
    pub const ALL: &'static [Self] = &[
        Self::Score,
        Self::Part,
        Self::LeadSheet,
        Self::ChordChart,
        Self::Tablature,
        Self::FakeBook,
        Self::Exercise,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Score => "score",
            Self::Part => "part",
            Self::LeadSheet => "lead-sheet",
            Self::ChordChart => "chord-chart",
            Self::Tablature => "tablature",
            Self::FakeBook => "fake-book",
            Self::Exercise => "exercise",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for MusicDocumentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MusicDocumentKind {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "score" => Ok(Self::Score),
            "part" => Ok(Self::Part),
            "lead-sheet" => Ok(Self::LeadSheet),
            "chord-chart" => Ok(Self::ChordChart),
            "tablature" => Ok(Self::Tablature),
            "fake-book" => Ok(Self::FakeBook),
            "exercise" => Ok(Self::Exercise),
            "custom" => Ok(Self::Custom),
            _ => Err(NotationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepeatMarkKind {
    RepeatStart,
    RepeatEnd,
    RepeatBoth,
    DalSegno,
    DaCapo,
    Coda,
    Fine,
}

impl RepeatMarkKind {
    pub const ALL: &'static [Self] = &[
        Self::RepeatStart,
        Self::RepeatEnd,
        Self::RepeatBoth,
        Self::DalSegno,
        Self::DaCapo,
        Self::Coda,
        Self::Fine,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepeatStart => "repeat-start",
            Self::RepeatEnd => "repeat-end",
            Self::RepeatBoth => "repeat-both",
            Self::DalSegno => "dal-segno",
            Self::DaCapo => "da-capo",
            Self::Coda => "coda",
            Self::Fine => "fine",
        }
    }
}

impl fmt::Display for RepeatMarkKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RepeatMarkKind {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "repeat-start" => Ok(Self::RepeatStart),
            "repeat-end" => Ok(Self::RepeatEnd),
            "repeat-both" => Ok(Self::RepeatBoth),
            "dal-segno" => Ok(Self::DalSegno),
            "da-capo" => Ok(Self::DaCapo),
            "coda" => Ok(Self::Coda),
            "fine" => Ok(Self::Fine),
            _ => Err(NotationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EndingKind {
    FirstEnding,
    SecondEnding,
    ThirdEnding,
    Custom,
}

impl EndingKind {
    pub const ALL: &'static [Self] = &[
        Self::FirstEnding,
        Self::SecondEnding,
        Self::ThirdEnding,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstEnding => "first-ending",
            Self::SecondEnding => "second-ending",
            Self::ThirdEnding => "third-ending",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for EndingKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EndingKind {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "first-ending" => Ok(Self::FirstEnding),
            "second-ending" => Ok(Self::SecondEnding),
            "third-ending" => Ok(Self::ThirdEnding),
            "custom" => Ok(Self::Custom),
            _ => Err(NotationError::UnknownLabel),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotationError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for NotationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("notation metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("notation metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("notation metadata value is out of range"),
            Self::NonFinite => formatter.write_str("notation metadata value must be finite"),
            Self::NonPositive => formatter.write_str("notation metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown notation metadata label"),
        }
    }
}

impl Error for NotationError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, NotationError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(NotationError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, NotationError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(NotationError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}
#[cfg(test)]
#[allow(
    unused_imports,
    clippy::unnecessary_wraps,
    clippy::assertions_on_constants
)]
mod tests {
    use super::{
        ClefKind, EndingKind, MeasurePosition, MusicDocumentKind, NotationError, NotationFormat,
        NotationSymbolKind, RepeatMarkKind, ScorePartName, StaffKind, StaffLineCount,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), NotationError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = NotationError>,
    {
        for variant in variants {
            let label = variant.to_string();
            assert_eq!(label.parse::<T>()?, *variant);
            assert_eq!(label.replace('-', "_").parse::<T>()?, *variant);
            assert_eq!(label.replace('-', " ").parse::<T>()?, *variant);
        }
        Ok(())
    }

    #[test]
    fn validates_text_newtypes() -> Result<(), NotationError> {
        let value = ScorePartName::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(
            <ScorePartName as TryFrom<&str>>::try_from("example-value")?,
            value
        );
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), NotationError> {
        let value = StaffLineCount::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<StaffLineCount>()?, value);
        assert_eq!(StaffLineCount::new(17), Err(NotationError::OutOfRange));
        let value = MeasurePosition::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<MeasurePosition>()?, value);
        assert_eq!(MeasurePosition::new(0), Err(NotationError::OutOfRange));
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), NotationError> {
        assert_enum_family(ClefKind::ALL)?;
        assert_enum_family(StaffKind::ALL)?;
        assert_enum_family(NotationSymbolKind::ALL)?;
        assert_enum_family(NotationFormat::ALL)?;
        assert_enum_family(MusicDocumentKind::ALL)?;
        assert_enum_family(RepeatMarkKind::ALL)?;
        assert_enum_family(EndingKind::ALL)?;
        Ok(())
    }
}
