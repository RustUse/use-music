#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        CircleOfFifthsPosition, KeyAccidentalCount, KeyError, KeyMode, KeyName, KeySignature,
        ParallelKeyRelation, RelativeKeyRelation, Tonic,
    };
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyName(String);

impl KeyName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, KeyError> {
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

impl AsRef<str> for KeyName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for KeyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for KeyName {
    type Err = KeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for KeyName {
    type Error = KeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tonic(String);

impl Tonic {
    pub fn new(value: impl AsRef<str>) -> Result<Self, KeyError> {
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

impl AsRef<str> for Tonic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Tonic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Tonic {
    type Err = KeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Tonic {
    type Error = KeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyAccidentalCount(i8);

impl KeyAccidentalCount {
    pub fn new(value: i8) -> Result<Self, KeyError> {
        if !(-7..=7).contains(&value) {
            return Err(KeyError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> i8 {
        self.0
    }
}

impl fmt::Display for KeyAccidentalCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for KeyAccidentalCount {
    type Err = KeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<i8>()
            .map_err(|_| KeyError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<i8> for KeyAccidentalCount {
    type Error = KeyError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CircleOfFifthsPosition(i8);

impl CircleOfFifthsPosition {
    pub fn new(value: i8) -> Result<Self, KeyError> {
        if !(-7..=7).contains(&value) {
            return Err(KeyError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> i8 {
        self.0
    }
}

impl fmt::Display for CircleOfFifthsPosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for CircleOfFifthsPosition {
    type Err = KeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<i8>()
            .map_err(|_| KeyError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<i8> for CircleOfFifthsPosition {
    type Error = KeyError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum KeyMode {
    Major,
    Minor,
    Modal,
    Atonal,
    Unknown,
}

impl KeyMode {
    pub const ALL: &'static [Self] = &[
        Self::Major,
        Self::Minor,
        Self::Modal,
        Self::Atonal,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Modal => "modal",
            Self::Atonal => "atonal",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for KeyMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for KeyMode {
    type Err = KeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "modal" => Ok(Self::Modal),
            "atonal" => Ok(Self::Atonal),
            "unknown" => Ok(Self::Unknown),
            _ => Err(KeyError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RelativeKeyRelation {
    RelativeMajor,
    RelativeMinor,
}

impl RelativeKeyRelation {
    pub const ALL: &'static [Self] = &[Self::RelativeMajor, Self::RelativeMinor];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelativeMajor => "relative-major",
            Self::RelativeMinor => "relative-minor",
        }
    }
}

impl fmt::Display for RelativeKeyRelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RelativeKeyRelation {
    type Err = KeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "relative-major" => Ok(Self::RelativeMajor),
            "relative-minor" => Ok(Self::RelativeMinor),
            _ => Err(KeyError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParallelKeyRelation {
    ParallelMajor,
    ParallelMinor,
}

impl ParallelKeyRelation {
    pub const ALL: &'static [Self] = &[Self::ParallelMajor, Self::ParallelMinor];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParallelMajor => "parallel-major",
            Self::ParallelMinor => "parallel-minor",
        }
    }
}

impl fmt::Display for ParallelKeyRelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ParallelKeyRelation {
    type Err = KeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "parallel-major" => Ok(Self::ParallelMajor),
            "parallel-minor" => Ok(Self::ParallelMinor),
            _ => Err(KeyError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KeySignature {
    accidental_count: KeyAccidentalCount,
    mode: KeyMode,
}

impl KeySignature {
    pub fn new(accidental_count: i8, mode: KeyMode) -> Result<Self, KeyError> {
        Ok(Self {
            accidental_count: KeyAccidentalCount::new(accidental_count)?,
            mode,
        })
    }

    pub const fn accidental_count(self) -> KeyAccidentalCount {
        self.accidental_count
    }
    pub const fn mode(self) -> KeyMode {
        self.mode
    }
    pub const fn is_sharp_key(self) -> bool {
        self.accidental_count.value() > 0
    }
    pub const fn is_flat_key(self) -> bool {
        self.accidental_count.value() < 0
    }
    pub const fn is_natural_key(self) -> bool {
        self.accidental_count.value() == 0
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for KeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("key metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("key metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("key metadata value is out of range"),
            Self::NonFinite => formatter.write_str("key metadata value must be finite"),
            Self::NonPositive => formatter.write_str("key metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown key metadata label"),
        }
    }
}

impl Error for KeyError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, KeyError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(KeyError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, KeyError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(KeyError::Empty)
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
        CircleOfFifthsPosition, KeyAccidentalCount, KeyError, KeyMode, KeyName, KeySignature,
        ParallelKeyRelation, RelativeKeyRelation, Tonic,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), KeyError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = KeyError>,
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
    fn validates_text_newtypes() -> Result<(), KeyError> {
        let value = KeyName::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(
            <KeyName as TryFrom<&str>>::try_from("example-value")?,
            value
        );
        let value = Tonic::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(<Tonic as TryFrom<&str>>::try_from("example-value")?, value);
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), KeyError> {
        let value = KeyAccidentalCount::new(-7)?;
        assert_eq!(value.value(), -7);
        assert_eq!("-7".parse::<KeyAccidentalCount>()?, value);
        assert_eq!(KeyAccidentalCount::new(8), Err(KeyError::OutOfRange));
        let value = CircleOfFifthsPosition::new(-7)?;
        assert_eq!(value.value(), -7);
        assert_eq!("-7".parse::<CircleOfFifthsPosition>()?, value);
        assert_eq!(CircleOfFifthsPosition::new(8), Err(KeyError::OutOfRange));
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), KeyError> {
        assert_enum_family(KeyMode::ALL)?;
        assert_enum_family(RelativeKeyRelation::ALL)?;
        assert_enum_family(ParallelKeyRelation::ALL)?;
        Ok(())
    }

    #[test]
    fn classifies_key_signatures() -> Result<(), KeyError> {
        let c_major = KeySignature::new(0, KeyMode::Major)?;
        let g_major = KeySignature::new(1, KeyMode::Major)?;
        let f_major = KeySignature::new(-1, KeyMode::Major)?;
        assert!(c_major.is_natural_key());
        assert!(g_major.is_sharp_key());
        assert!(f_major.is_flat_key());
        assert_eq!(KeyAccidentalCount::new(8), Err(KeyError::OutOfRange));
        Ok(())
    }
}
