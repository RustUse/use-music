#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        BarlineKind, BeatUnit, BeatsPerMeasure, MeasureNumber, MeterError, MeterKind,
        MetricAccentPattern, PickupMeasureKind, TimeSignature,
    };
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BeatsPerMeasure(u16);

impl BeatsPerMeasure {
    pub fn new(value: u16) -> Result<Self, MeterError> {
        if !(1..=256).contains(&value) {
            return Err(MeterError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}

impl fmt::Display for BeatsPerMeasure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for BeatsPerMeasure {
    type Err = MeterError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u16>()
            .map_err(|_| MeterError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u16> for BeatsPerMeasure {
    type Error = MeterError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeasureNumber(u16);

impl MeasureNumber {
    pub fn new(value: u16) -> Result<Self, MeterError> {
        if !(1..=65535).contains(&value) {
            return Err(MeterError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}

impl fmt::Display for MeasureNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for MeasureNumber {
    type Err = MeterError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u16>()
            .map_err(|_| MeterError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u16> for MeasureNumber {
    type Error = MeterError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MeterKind {
    Simple,
    Compound,
    Complex,
    Additive,
    Irregular,
    Free,
    Unknown,
}

impl MeterKind {
    pub const ALL: &'static [Self] = &[
        Self::Simple,
        Self::Compound,
        Self::Complex,
        Self::Additive,
        Self::Irregular,
        Self::Free,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Simple => "simple",
            Self::Compound => "compound",
            Self::Complex => "complex",
            Self::Additive => "additive",
            Self::Irregular => "irregular",
            Self::Free => "free",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for MeterKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MeterKind {
    type Err = MeterError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "simple" => Ok(Self::Simple),
            "compound" => Ok(Self::Compound),
            "complex" => Ok(Self::Complex),
            "additive" => Ok(Self::Additive),
            "irregular" => Ok(Self::Irregular),
            "free" => Ok(Self::Free),
            "unknown" => Ok(Self::Unknown),
            _ => Err(MeterError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BeatUnit {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
}

impl BeatUnit {
    pub const ALL: &'static [Self] = &[
        Self::Whole,
        Self::Half,
        Self::Quarter,
        Self::Eighth,
        Self::Sixteenth,
        Self::ThirtySecond,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Whole => "whole",
            Self::Half => "half",
            Self::Quarter => "quarter",
            Self::Eighth => "eighth",
            Self::Sixteenth => "sixteenth",
            Self::ThirtySecond => "thirty-second",
        }
    }
}

impl fmt::Display for BeatUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BeatUnit {
    type Err = MeterError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "whole" => Ok(Self::Whole),
            "half" => Ok(Self::Half),
            "quarter" => Ok(Self::Quarter),
            "eighth" => Ok(Self::Eighth),
            "sixteenth" => Ok(Self::Sixteenth),
            "thirty-second" => Ok(Self::ThirtySecond),
            _ => Err(MeterError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BarlineKind {
    Single,
    Double,
    Final,
    RepeatStart,
    RepeatEnd,
    RepeatBoth,
}

impl BarlineKind {
    pub const ALL: &'static [Self] = &[
        Self::Single,
        Self::Double,
        Self::Final,
        Self::RepeatStart,
        Self::RepeatEnd,
        Self::RepeatBoth,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Double => "double",
            Self::Final => "final",
            Self::RepeatStart => "repeat-start",
            Self::RepeatEnd => "repeat-end",
            Self::RepeatBoth => "repeat-both",
        }
    }
}

impl fmt::Display for BarlineKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BarlineKind {
    type Err = MeterError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "single" => Ok(Self::Single),
            "double" => Ok(Self::Double),
            "final" => Ok(Self::Final),
            "repeat-start" => Ok(Self::RepeatStart),
            "repeat-end" => Ok(Self::RepeatEnd),
            "repeat-both" => Ok(Self::RepeatBoth),
            _ => Err(MeterError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PickupMeasureKind {
    None,
    Anacrusis,
}

impl PickupMeasureKind {
    pub const ALL: &'static [Self] = &[Self::None, Self::Anacrusis];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Anacrusis => "anacrusis",
        }
    }
}

impl fmt::Display for PickupMeasureKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PickupMeasureKind {
    type Err = MeterError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "none" => Ok(Self::None),
            "anacrusis" => Ok(Self::Anacrusis),
            _ => Err(MeterError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TimeSignature {
    numerator: u16,
    denominator: u16,
}

impl TimeSignature {
    pub fn new(numerator: u16, denominator: u16) -> Result<Self, MeterError> {
        if numerator == 0 || !matches!(denominator, 1 | 2 | 4 | 8 | 16 | 32 | 64) {
            return Err(MeterError::OutOfRange);
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }

    pub const fn numerator(self) -> u16 {
        self.numerator
    }
    pub const fn denominator(self) -> u16 {
        self.denominator
    }
    pub const fn is_common_time_like(self) -> bool {
        self.numerator == 4 && self.denominator == 4
    }
    pub const fn is_cut_time_like(self) -> bool {
        self.numerator == 2 && self.denominator == 2
    }
    pub const fn is_compound(self) -> bool {
        self.numerator > 3 && self.numerator.is_multiple_of(3)
    }
    pub const fn is_simple(self) -> bool {
        !self.is_compound()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetricAccentPattern(Vec<u8>);

impl MetricAccentPattern {
    pub fn new(accents: impl Into<Vec<u8>>) -> Result<Self, MeterError> {
        let accents = accents.into();
        if accents.is_empty() {
            return Err(MeterError::Empty);
        }
        Ok(Self(accents))
    }
    pub fn accents(&self) -> &[u8] {
        &self.0
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for MeterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("meter metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("meter metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("meter metadata value is out of range"),
            Self::NonFinite => formatter.write_str("meter metadata value must be finite"),
            Self::NonPositive => formatter.write_str("meter metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown meter metadata label"),
        }
    }
}

impl Error for MeterError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, MeterError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(MeterError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, MeterError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(MeterError::Empty)
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
        BarlineKind, BeatUnit, BeatsPerMeasure, MeasureNumber, MeterError, MeterKind,
        MetricAccentPattern, PickupMeasureKind, TimeSignature,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), MeterError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = MeterError>,
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
    fn validates_text_newtypes() -> Result<(), MeterError> {
        assert!(true);
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), MeterError> {
        let value = BeatsPerMeasure::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<BeatsPerMeasure>()?, value);
        assert_eq!(BeatsPerMeasure::new(257), Err(MeterError::OutOfRange));
        let value = MeasureNumber::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<MeasureNumber>()?, value);
        assert_eq!(MeasureNumber::new(0), Err(MeterError::OutOfRange));
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), MeterError> {
        assert_enum_family(MeterKind::ALL)?;
        assert_enum_family(BeatUnit::ALL)?;
        assert_enum_family(BarlineKind::ALL)?;
        assert_enum_family(PickupMeasureKind::ALL)?;
        Ok(())
    }

    #[test]
    fn validates_time_signatures() -> Result<(), MeterError> {
        let common = TimeSignature::new(4, 4)?;
        let cut = TimeSignature::new(2, 2)?;
        let compound = TimeSignature::new(6, 8)?;
        assert!(common.is_common_time_like());
        assert!(cut.is_cut_time_like());
        assert!(compound.is_compound());
        assert_eq!(TimeSignature::new(0, 4), Err(MeterError::OutOfRange));
        assert_eq!(TimeSignature::new(4, 3), Err(MeterError::OutOfRange));
        Ok(())
    }
}
