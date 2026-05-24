#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        ChurchMode, ModalBrightness, ModeDegree, ModeError, ModeFamily, ModeKind, ModeName,
    };
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModeName(String);

impl ModeName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, ModeError> {
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

impl AsRef<str> for ModeName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ModeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ModeName {
    type Err = ModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ModeName {
    type Error = ModeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModeDegree(u8);

impl ModeDegree {
    pub fn new(value: u8) -> Result<Self, ModeError> {
        if !(1..=32).contains(&value) {
            return Err(ModeError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for ModeDegree {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ModeDegree {
    type Err = ModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| ModeError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for ModeDegree {
    type Error = ModeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModeKind {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    Major,
    Minor,
    Custom,
}

impl ModeKind {
    pub const ALL: &'static [Self] = &[
        Self::Ionian,
        Self::Dorian,
        Self::Phrygian,
        Self::Lydian,
        Self::Mixolydian,
        Self::Aeolian,
        Self::Locrian,
        Self::Major,
        Self::Minor,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ionian => "ionian",
            Self::Dorian => "dorian",
            Self::Phrygian => "phrygian",
            Self::Lydian => "lydian",
            Self::Mixolydian => "mixolydian",
            Self::Aeolian => "aeolian",
            Self::Locrian => "locrian",
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ModeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ModeKind {
    type Err = ModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "ionian" => Ok(Self::Ionian),
            "dorian" => Ok(Self::Dorian),
            "phrygian" => Ok(Self::Phrygian),
            "lydian" => Ok(Self::Lydian),
            "mixolydian" => Ok(Self::Mixolydian),
            "aeolian" => Ok(Self::Aeolian),
            "locrian" => Ok(Self::Locrian),
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "custom" => Ok(Self::Custom),
            _ => Err(ModeError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChurchMode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl ChurchMode {
    pub const ALL: &'static [Self] = &[
        Self::Ionian,
        Self::Dorian,
        Self::Phrygian,
        Self::Lydian,
        Self::Mixolydian,
        Self::Aeolian,
        Self::Locrian,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ionian => "ionian",
            Self::Dorian => "dorian",
            Self::Phrygian => "phrygian",
            Self::Lydian => "lydian",
            Self::Mixolydian => "mixolydian",
            Self::Aeolian => "aeolian",
            Self::Locrian => "locrian",
        }
    }
}

impl fmt::Display for ChurchMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ChurchMode {
    type Err = ModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "ionian" => Ok(Self::Ionian),
            "dorian" => Ok(Self::Dorian),
            "phrygian" => Ok(Self::Phrygian),
            "lydian" => Ok(Self::Lydian),
            "mixolydian" => Ok(Self::Mixolydian),
            "aeolian" => Ok(Self::Aeolian),
            "locrian" => Ok(Self::Locrian),
            _ => Err(ModeError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModeFamily {
    Diatonic,
    MelodicMinor,
    HarmonicMinor,
    Pentatonic,
    Custom,
}

impl ModeFamily {
    pub const ALL: &'static [Self] = &[
        Self::Diatonic,
        Self::MelodicMinor,
        Self::HarmonicMinor,
        Self::Pentatonic,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diatonic => "diatonic",
            Self::MelodicMinor => "melodic-minor",
            Self::HarmonicMinor => "harmonic-minor",
            Self::Pentatonic => "pentatonic",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ModeFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ModeFamily {
    type Err = ModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "diatonic" => Ok(Self::Diatonic),
            "melodic-minor" => Ok(Self::MelodicMinor),
            "harmonic-minor" => Ok(Self::HarmonicMinor),
            "pentatonic" => Ok(Self::Pentatonic),
            "custom" => Ok(Self::Custom),
            _ => Err(ModeError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModalBrightness {
    VeryDark,
    Dark,
    Neutral,
    Bright,
    VeryBright,
    Unknown,
}

impl ModalBrightness {
    pub const ALL: &'static [Self] = &[
        Self::VeryDark,
        Self::Dark,
        Self::Neutral,
        Self::Bright,
        Self::VeryBright,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VeryDark => "very-dark",
            Self::Dark => "dark",
            Self::Neutral => "neutral",
            Self::Bright => "bright",
            Self::VeryBright => "very-bright",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for ModalBrightness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ModalBrightness {
    type Err = ModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "very-dark" => Ok(Self::VeryDark),
            "dark" => Ok(Self::Dark),
            "neutral" => Ok(Self::Neutral),
            "bright" => Ok(Self::Bright),
            "very-bright" => Ok(Self::VeryBright),
            "unknown" => Ok(Self::Unknown),
            _ => Err(ModeError::UnknownLabel),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModeError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for ModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("mode metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("mode metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("mode metadata value is out of range"),
            Self::NonFinite => formatter.write_str("mode metadata value must be finite"),
            Self::NonPositive => formatter.write_str("mode metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown mode metadata label"),
        }
    }
}

impl Error for ModeError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, ModeError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(ModeError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, ModeError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(ModeError::Empty)
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
        ChurchMode, ModalBrightness, ModeDegree, ModeError, ModeFamily, ModeKind, ModeName,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), ModeError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = ModeError>,
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
    fn validates_text_newtypes() -> Result<(), ModeError> {
        let value = ModeName::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(
            <ModeName as TryFrom<&str>>::try_from("example-value")?,
            value
        );
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), ModeError> {
        let value = ModeDegree::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<ModeDegree>()?, value);
        assert_eq!(ModeDegree::new(33), Err(ModeError::OutOfRange));
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), ModeError> {
        assert_enum_family(ModeKind::ALL)?;
        assert_enum_family(ChurchMode::ALL)?;
        assert_enum_family(ModeFamily::ALL)?;
        assert_enum_family(ModalBrightness::ALL)?;
        Ok(())
    }
}
