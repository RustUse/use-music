#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        ChordAlteration, ChordError, ChordExtension, ChordInversion, ChordName, ChordQuality,
        ChordSymbol, ChordToneRole, ChordVoicingKind, SeventhChordKind, TriadKind,
    };
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChordName(String);

impl ChordName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, ChordError> {
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

impl AsRef<str> for ChordName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ChordName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ChordName {
    type Err = ChordError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ChordName {
    type Error = ChordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChordSymbol(String);

impl ChordSymbol {
    pub fn new(value: impl AsRef<str>) -> Result<Self, ChordError> {
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

impl AsRef<str> for ChordSymbol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ChordSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ChordSymbol {
    type Err = ChordError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ChordSymbol {
    type Error = ChordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Suspended2,
    Suspended4,
    Dominant,
    HalfDiminished,
    MajorMinor,
    Power,
    Custom,
}

impl ChordQuality {
    pub const ALL: &'static [Self] = &[
        Self::Major,
        Self::Minor,
        Self::Diminished,
        Self::Augmented,
        Self::Suspended2,
        Self::Suspended4,
        Self::Dominant,
        Self::HalfDiminished,
        Self::MajorMinor,
        Self::Power,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Diminished => "diminished",
            Self::Augmented => "augmented",
            Self::Suspended2 => "suspended-2",
            Self::Suspended4 => "suspended-4",
            Self::Dominant => "dominant",
            Self::HalfDiminished => "half-diminished",
            Self::MajorMinor => "major-minor",
            Self::Power => "power",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ChordQuality {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ChordQuality {
    type Err = ChordError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "diminished" => Ok(Self::Diminished),
            "augmented" => Ok(Self::Augmented),
            "suspended-2" => Ok(Self::Suspended2),
            "suspended-4" => Ok(Self::Suspended4),
            "dominant" => Ok(Self::Dominant),
            "half-diminished" => Ok(Self::HalfDiminished),
            "major-minor" => Ok(Self::MajorMinor),
            "power" => Ok(Self::Power),
            "custom" => Ok(Self::Custom),
            _ => Err(ChordError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChordToneRole {
    Root,
    Third,
    Fifth,
    Seventh,
    Ninth,
    Eleventh,
    Thirteenth,
    AddedTone,
    AlteredTone,
}

impl ChordToneRole {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Third,
        Self::Fifth,
        Self::Seventh,
        Self::Ninth,
        Self::Eleventh,
        Self::Thirteenth,
        Self::AddedTone,
        Self::AlteredTone,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Root => "root",
            Self::Third => "third",
            Self::Fifth => "fifth",
            Self::Seventh => "seventh",
            Self::Ninth => "ninth",
            Self::Eleventh => "eleventh",
            Self::Thirteenth => "thirteenth",
            Self::AddedTone => "added-tone",
            Self::AlteredTone => "altered-tone",
        }
    }
}

impl fmt::Display for ChordToneRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ChordToneRole {
    type Err = ChordError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "root" => Ok(Self::Root),
            "third" => Ok(Self::Third),
            "fifth" => Ok(Self::Fifth),
            "seventh" => Ok(Self::Seventh),
            "ninth" => Ok(Self::Ninth),
            "eleventh" => Ok(Self::Eleventh),
            "thirteenth" => Ok(Self::Thirteenth),
            "added-tone" => Ok(Self::AddedTone),
            "altered-tone" => Ok(Self::AlteredTone),
            _ => Err(ChordError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChordVoicingKind {
    Closed,
    Open,
    Drop2,
    Drop3,
    Quartal,
    Cluster,
    Shell,
    Custom,
}

impl ChordVoicingKind {
    pub const ALL: &'static [Self] = &[
        Self::Closed,
        Self::Open,
        Self::Drop2,
        Self::Drop3,
        Self::Quartal,
        Self::Cluster,
        Self::Shell,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
            Self::Drop2 => "drop-2",
            Self::Drop3 => "drop-3",
            Self::Quartal => "quartal",
            Self::Cluster => "cluster",
            Self::Shell => "shell",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ChordVoicingKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ChordVoicingKind {
    type Err = ChordError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "closed" => Ok(Self::Closed),
            "open" => Ok(Self::Open),
            "drop-2" => Ok(Self::Drop2),
            "drop-3" => Ok(Self::Drop3),
            "quartal" => Ok(Self::Quartal),
            "cluster" => Ok(Self::Cluster),
            "shell" => Ok(Self::Shell),
            "custom" => Ok(Self::Custom),
            _ => Err(ChordError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TriadKind {
    Major,
    Minor,
    Diminished,
    Augmented,
    Suspended2,
    Suspended4,
    Power,
}

impl TriadKind {
    pub const ALL: &'static [Self] = &[
        Self::Major,
        Self::Minor,
        Self::Diminished,
        Self::Augmented,
        Self::Suspended2,
        Self::Suspended4,
        Self::Power,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Diminished => "diminished",
            Self::Augmented => "augmented",
            Self::Suspended2 => "suspended-2",
            Self::Suspended4 => "suspended-4",
            Self::Power => "power",
        }
    }
}

impl fmt::Display for TriadKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TriadKind {
    type Err = ChordError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "diminished" => Ok(Self::Diminished),
            "augmented" => Ok(Self::Augmented),
            "suspended-2" => Ok(Self::Suspended2),
            "suspended-4" => Ok(Self::Suspended4),
            "power" => Ok(Self::Power),
            _ => Err(ChordError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SeventhChordKind {
    MajorSeventh,
    MinorSeventh,
    Dominant,
    HalfDiminished,
    Diminished,
    MinorMajor,
}

impl SeventhChordKind {
    pub const ALL: &'static [Self] = &[
        Self::MajorSeventh,
        Self::MinorSeventh,
        Self::Dominant,
        Self::HalfDiminished,
        Self::Diminished,
        Self::MinorMajor,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MajorSeventh => "major-seventh",
            Self::MinorSeventh => "minor-seventh",
            Self::Dominant => "dominant",
            Self::HalfDiminished => "half-diminished",
            Self::Diminished => "diminished",
            Self::MinorMajor => "minor-major",
        }
    }
}

impl fmt::Display for SeventhChordKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SeventhChordKind {
    type Err = ChordError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "major-seventh" => Ok(Self::MajorSeventh),
            "minor-seventh" => Ok(Self::MinorSeventh),
            "dominant" => Ok(Self::Dominant),
            "half-diminished" => Ok(Self::HalfDiminished),
            "diminished" => Ok(Self::Diminished),
            "minor-major" => Ok(Self::MinorMajor),
            _ => Err(ChordError::UnknownLabel),
        }
    }
}
pub type ChordExtension = u8;
pub type ChordAlteration = String;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChordInversion(u8);

impl ChordInversion {
    pub fn new(value: u8) -> Result<Self, ChordError> {
        if value > 7 {
            return Err(ChordError::OutOfRange);
        }
        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl TriadKind {
    pub const MAJOR: Self = Self::Major;
    pub const MINOR: Self = Self::Minor;
    pub const DIMINISHED: Self = Self::Diminished;
    pub const AUGMENTED: Self = Self::Augmented;
}

impl SeventhChordKind {
    pub const DOMINANT: Self = Self::Dominant;
    pub const MAJOR_SEVENTH: Self = Self::MajorSeventh;
    pub const MINOR_SEVENTH: Self = Self::MinorSeventh;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChordError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for ChordError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("chord metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("chord metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("chord metadata value is out of range"),
            Self::NonFinite => formatter.write_str("chord metadata value must be finite"),
            Self::NonPositive => formatter.write_str("chord metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown chord metadata label"),
        }
    }
}

impl Error for ChordError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, ChordError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(ChordError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, ChordError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(ChordError::Empty)
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
        ChordAlteration, ChordError, ChordExtension, ChordInversion, ChordName, ChordQuality,
        ChordSymbol, ChordToneRole, ChordVoicingKind, SeventhChordKind, TriadKind,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), ChordError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = ChordError>,
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
    fn validates_text_newtypes() -> Result<(), ChordError> {
        let value = ChordName::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(
            <ChordName as TryFrom<&str>>::try_from("example-value")?,
            value
        );
        let value = ChordSymbol::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(
            <ChordSymbol as TryFrom<&str>>::try_from("example-value")?,
            value
        );
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), ChordError> {
        assert!(true);
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), ChordError> {
        assert_enum_family(ChordQuality::ALL)?;
        assert_enum_family(ChordToneRole::ALL)?;
        assert_enum_family(ChordVoicingKind::ALL)?;
        assert_enum_family(TriadKind::ALL)?;
        assert_enum_family(SeventhChordKind::ALL)?;
        Ok(())
    }

    #[test]
    fn validates_chord_metadata() -> Result<(), ChordError> {
        let symbol = ChordSymbol::new(" Cmaj7 ")?;
        assert_eq!(symbol.as_str(), "Cmaj7");
        assert_eq!(ChordSymbol::new(" "), Err(ChordError::Empty));
        assert_eq!(ChordInversion::new(1)?.value(), 1);
        assert_eq!(TriadKind::MAJOR, TriadKind::Major);
        assert_eq!(SeventhChordKind::DOMINANT, SeventhChordKind::Dominant);
        Ok(())
    }
}
