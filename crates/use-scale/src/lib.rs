#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        ChromaticScale, DiatonicScale, PentatonicScale, ScaleDegree, ScaleError, ScaleKind,
        ScaleName, ScalePattern, ScaleStepPattern, ScaleToneCount,
    };
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScaleName(String);

impl ScaleName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, ScaleError> {
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

impl AsRef<str> for ScaleName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ScaleName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ScaleName {
    type Err = ScaleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ScaleName {
    type Error = ScaleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScaleDegree(u8);

impl ScaleDegree {
    pub fn new(value: u8) -> Result<Self, ScaleError> {
        if !(1..=64).contains(&value) {
            return Err(ScaleError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for ScaleDegree {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ScaleDegree {
    type Err = ScaleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| ScaleError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for ScaleDegree {
    type Error = ScaleError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScaleToneCount(u8);

impl ScaleToneCount {
    pub fn new(value: u8) -> Result<Self, ScaleError> {
        if !(1..=64).contains(&value) {
            return Err(ScaleError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for ScaleToneCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ScaleToneCount {
    type Err = ScaleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| ScaleError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for ScaleToneCount {
    type Error = ScaleError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ScaleKind {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Chromatic,
    MajorPentatonic,
    MinorPentatonic,
    Blues,
    WholeTone,
    Diminished,
    Custom,
}

impl ScaleKind {
    pub const ALL: &'static [Self] = &[
        Self::Major,
        Self::NaturalMinor,
        Self::HarmonicMinor,
        Self::MelodicMinor,
        Self::Chromatic,
        Self::MajorPentatonic,
        Self::MinorPentatonic,
        Self::Blues,
        Self::WholeTone,
        Self::Diminished,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::NaturalMinor => "natural-minor",
            Self::HarmonicMinor => "harmonic-minor",
            Self::MelodicMinor => "melodic-minor",
            Self::Chromatic => "chromatic",
            Self::MajorPentatonic => "major-pentatonic",
            Self::MinorPentatonic => "minor-pentatonic",
            Self::Blues => "blues",
            Self::WholeTone => "whole-tone",
            Self::Diminished => "diminished",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ScaleKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ScaleKind {
    type Err = ScaleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "major" => Ok(Self::Major),
            "natural-minor" => Ok(Self::NaturalMinor),
            "harmonic-minor" => Ok(Self::HarmonicMinor),
            "melodic-minor" => Ok(Self::MelodicMinor),
            "chromatic" => Ok(Self::Chromatic),
            "major-pentatonic" => Ok(Self::MajorPentatonic),
            "minor-pentatonic" => Ok(Self::MinorPentatonic),
            "blues" => Ok(Self::Blues),
            "whole-tone" => Ok(Self::WholeTone),
            "diminished" => Ok(Self::Diminished),
            "custom" => Ok(Self::Custom),
            _ => Err(ScaleError::UnknownLabel),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScalePattern {
    steps: Vec<u8>,
}

impl ScalePattern {
    pub fn new(steps: impl Into<Vec<u8>>) -> Result<Self, ScaleError> {
        let steps = steps.into();
        if steps.is_empty() {
            return Err(ScaleError::Empty);
        }
        Ok(Self { steps })
    }

    pub fn steps(&self) -> &[u8] {
        &self.steps
    }

    pub fn tone_count(&self) -> ScaleToneCount {
        ScaleToneCount(u8::try_from(self.steps.len()).unwrap_or(u8::MAX))
    }

    pub fn is_heptatonic(&self) -> bool {
        self.steps.len() == 7
    }
    pub fn is_pentatonic(&self) -> bool {
        self.steps.len() == 5
    }
    pub fn is_chromatic(&self) -> bool {
        self.steps.len() == 12
    }
}

pub type ScaleStepPattern = ScalePattern;
pub type DiatonicScale = ScalePattern;
pub type PentatonicScale = ScalePattern;
pub type ChromaticScale = ScalePattern;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScaleError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for ScaleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("scale metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("scale metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("scale metadata value is out of range"),
            Self::NonFinite => formatter.write_str("scale metadata value must be finite"),
            Self::NonPositive => formatter.write_str("scale metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown scale metadata label"),
        }
    }
}

impl Error for ScaleError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, ScaleError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(ScaleError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, ScaleError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(ScaleError::Empty)
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
        ChromaticScale, DiatonicScale, PentatonicScale, ScaleDegree, ScaleError, ScaleKind,
        ScaleName, ScalePattern, ScaleStepPattern, ScaleToneCount,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), ScaleError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = ScaleError>,
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
    fn validates_text_newtypes() -> Result<(), ScaleError> {
        let value = ScaleName::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(
            <ScaleName as TryFrom<&str>>::try_from("example-value")?,
            value
        );
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), ScaleError> {
        let value = ScaleDegree::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<ScaleDegree>()?, value);
        assert_eq!(ScaleDegree::new(65), Err(ScaleError::OutOfRange));
        let value = ScaleToneCount::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<ScaleToneCount>()?, value);
        assert_eq!(ScaleToneCount::new(65), Err(ScaleError::OutOfRange));
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), ScaleError> {
        assert_enum_family(ScaleKind::ALL)?;
        Ok(())
    }

    #[test]
    fn classifies_scale_patterns() -> Result<(), ScaleError> {
        let major = ScalePattern::new([2, 2, 1, 2, 2, 2, 1])?;
        let pentatonic = ScalePattern::new([2, 2, 3, 2, 3])?;
        let chromatic = ScalePattern::new([1; 12])?;
        assert!(major.is_heptatonic());
        assert!(pentatonic.is_pentatonic());
        assert!(chromatic.is_chromatic());
        assert_eq!(major.tone_count().value(), 7);
        Ok(())
    }
}
