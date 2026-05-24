#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AccentDynamicKind, DynamicChangeKind, DynamicLevel, DynamicMarking, DynamicsError,
        ExpressionMarking, HairpinKind,
    };
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExpressionMarking(String);

impl ExpressionMarking {
    pub fn new(value: impl AsRef<str>) -> Result<Self, DynamicsError> {
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

impl AsRef<str> for ExpressionMarking {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ExpressionMarking {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ExpressionMarking {
    type Err = DynamicsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ExpressionMarking {
    type Error = DynamicsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DynamicLevel(u8);

impl DynamicLevel {
    pub fn new(value: u8) -> Result<Self, DynamicsError> {
        if !(0..=127).contains(&value) {
            return Err(DynamicsError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for DynamicLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for DynamicLevel {
    type Err = DynamicsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| DynamicsError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for DynamicLevel {
    type Error = DynamicsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DynamicMarking {
    Pppp,
    Ppp,
    Pp,
    P,
    Mp,
    Mf,
    F,
    Ff,
    Fff,
    Ffff,
    Sfz,
    Fp,
    Custom,
}

impl DynamicMarking {
    pub const ALL: &'static [Self] = &[
        Self::Pppp,
        Self::Ppp,
        Self::Pp,
        Self::P,
        Self::Mp,
        Self::Mf,
        Self::F,
        Self::Ff,
        Self::Fff,
        Self::Ffff,
        Self::Sfz,
        Self::Fp,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pppp => "pppp",
            Self::Ppp => "ppp",
            Self::Pp => "pp",
            Self::P => "p",
            Self::Mp => "mp",
            Self::Mf => "mf",
            Self::F => "f",
            Self::Ff => "ff",
            Self::Fff => "fff",
            Self::Ffff => "ffff",
            Self::Sfz => "sfz",
            Self::Fp => "fp",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for DynamicMarking {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DynamicMarking {
    type Err = DynamicsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "pppp" => Ok(Self::Pppp),
            "ppp" => Ok(Self::Ppp),
            "pp" => Ok(Self::Pp),
            "p" => Ok(Self::P),
            "mp" => Ok(Self::Mp),
            "mf" => Ok(Self::Mf),
            "f" => Ok(Self::F),
            "ff" => Ok(Self::Ff),
            "fff" => Ok(Self::Fff),
            "ffff" => Ok(Self::Ffff),
            "sfz" => Ok(Self::Sfz),
            "fp" => Ok(Self::Fp),
            "custom" => Ok(Self::Custom),
            _ => Err(DynamicsError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DynamicChangeKind {
    Crescendo,
    Decrescendo,
    Diminuendo,
    Subito,
    Gradual,
    Unknown,
}

impl DynamicChangeKind {
    pub const ALL: &'static [Self] = &[
        Self::Crescendo,
        Self::Decrescendo,
        Self::Diminuendo,
        Self::Subito,
        Self::Gradual,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Crescendo => "crescendo",
            Self::Decrescendo => "decrescendo",
            Self::Diminuendo => "diminuendo",
            Self::Subito => "subito",
            Self::Gradual => "gradual",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for DynamicChangeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DynamicChangeKind {
    type Err = DynamicsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "crescendo" => Ok(Self::Crescendo),
            "decrescendo" => Ok(Self::Decrescendo),
            "diminuendo" => Ok(Self::Diminuendo),
            "subito" => Ok(Self::Subito),
            "gradual" => Ok(Self::Gradual),
            "unknown" => Ok(Self::Unknown),
            _ => Err(DynamicsError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HairpinKind {
    Crescendo,
    Decrescendo,
}

impl HairpinKind {
    pub const ALL: &'static [Self] = &[Self::Crescendo, Self::Decrescendo];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Crescendo => "crescendo",
            Self::Decrescendo => "decrescendo",
        }
    }
}

impl fmt::Display for HairpinKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for HairpinKind {
    type Err = DynamicsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "crescendo" => Ok(Self::Crescendo),
            "decrescendo" => Ok(Self::Decrescendo),
            _ => Err(DynamicsError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AccentDynamicKind {
    Accent,
    Marcato,
    Sforzando,
    Rinforzando,
    Custom,
}

impl AccentDynamicKind {
    pub const ALL: &'static [Self] = &[
        Self::Accent,
        Self::Marcato,
        Self::Sforzando,
        Self::Rinforzando,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accent => "accent",
            Self::Marcato => "marcato",
            Self::Sforzando => "sforzando",
            Self::Rinforzando => "rinforzando",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for AccentDynamicKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AccentDynamicKind {
    type Err = DynamicsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "accent" => Ok(Self::Accent),
            "marcato" => Ok(Self::Marcato),
            "sforzando" => Ok(Self::Sforzando),
            "rinforzando" => Ok(Self::Rinforzando),
            "custom" => Ok(Self::Custom),
            _ => Err(DynamicsError::UnknownLabel),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicsError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for DynamicsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("dynamics metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("dynamics metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("dynamics metadata value is out of range"),
            Self::NonFinite => formatter.write_str("dynamics metadata value must be finite"),
            Self::NonPositive => formatter.write_str("dynamics metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown dynamics metadata label"),
        }
    }
}

impl Error for DynamicsError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, DynamicsError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(DynamicsError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, DynamicsError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(DynamicsError::Empty)
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
        AccentDynamicKind, DynamicChangeKind, DynamicLevel, DynamicMarking, DynamicsError,
        ExpressionMarking, HairpinKind,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), DynamicsError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = DynamicsError>,
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
    fn validates_text_newtypes() -> Result<(), DynamicsError> {
        let value = ExpressionMarking::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(
            <ExpressionMarking as TryFrom<&str>>::try_from("example-value")?,
            value
        );
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), DynamicsError> {
        let value = DynamicLevel::new(0)?;
        assert_eq!(value.value(), 0);
        assert_eq!("0".parse::<DynamicLevel>()?, value);
        assert_eq!(DynamicLevel::new(128), Err(DynamicsError::OutOfRange));
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), DynamicsError> {
        assert_enum_family(DynamicMarking::ALL)?;
        assert_enum_family(DynamicChangeKind::ALL)?;
        assert_enum_family(HairpinKind::ALL)?;
        assert_enum_family(AccentDynamicKind::ALL)?;
        Ok(())
    }
}
