#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        Cents, ConcertPitchStandard, EqualTemperamentDivision, MicrotonalDivision, ReferenceNote,
        ReferencePitch, TemperamentKind, TuningError, TuningRatio, TuningSystem,
    };
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MicrotonalDivision(u16);

impl MicrotonalDivision {
    pub fn new(value: u16) -> Result<Self, TuningError> {
        if !(1..=4096).contains(&value) {
            return Err(TuningError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}

impl fmt::Display for MicrotonalDivision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for MicrotonalDivision {
    type Err = TuningError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u16>()
            .map_err(|_| TuningError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u16> for MicrotonalDivision {
    type Error = TuningError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EqualTemperamentDivision(u16);

impl EqualTemperamentDivision {
    pub fn new(value: u16) -> Result<Self, TuningError> {
        if !(1..=4096).contains(&value) {
            return Err(TuningError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}

impl fmt::Display for EqualTemperamentDivision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for EqualTemperamentDivision {
    type Err = TuningError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u16>()
            .map_err(|_| TuningError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u16> for EqualTemperamentDivision {
    type Error = TuningError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ReferencePitch(f64);

impl ReferencePitch {
    pub fn new(value: f64) -> Result<Self, TuningError> {
        if !value.is_finite() {
            return Err(TuningError::NonFinite);
        }
        if value <= 0.0 {
            return Err(TuningError::NonPositive);
        }
        Ok(Self(value))
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for ReferencePitch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ReferencePitch {
    type Err = TuningError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<f64>()
            .map_err(|_| TuningError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<f64> for ReferencePitch {
    type Error = TuningError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Cents(f64);

impl Cents {
    pub fn new(value: f64) -> Result<Self, TuningError> {
        if !value.is_finite() {
            return Err(TuningError::NonFinite);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Cents {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for Cents {
    type Err = TuningError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<f64>()
            .map_err(|_| TuningError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<f64> for Cents {
    type Error = TuningError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TuningSystem {
    EqualTemperament,
    JustIntonation,
    Pythagorean,
    Meantone,
    WellTemperament,
    Microtonal,
    Custom,
}

impl TuningSystem {
    pub const ALL: &'static [Self] = &[
        Self::EqualTemperament,
        Self::JustIntonation,
        Self::Pythagorean,
        Self::Meantone,
        Self::WellTemperament,
        Self::Microtonal,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EqualTemperament => "equal-temperament",
            Self::JustIntonation => "just-intonation",
            Self::Pythagorean => "pythagorean",
            Self::Meantone => "meantone",
            Self::WellTemperament => "well-temperament",
            Self::Microtonal => "microtonal",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for TuningSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TuningSystem {
    type Err = TuningError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "equal-temperament" => Ok(Self::EqualTemperament),
            "just-intonation" => Ok(Self::JustIntonation),
            "pythagorean" => Ok(Self::Pythagorean),
            "meantone" => Ok(Self::Meantone),
            "well-temperament" => Ok(Self::WellTemperament),
            "microtonal" => Ok(Self::Microtonal),
            "custom" => Ok(Self::Custom),
            _ => Err(TuningError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TemperamentKind {
    TwelveToneEqualTemperament,
    NineteenToneEqualTemperament,
    TwentyFourToneEqualTemperament,
    Just,
    Pythagorean,
    QuarterCommaMeantone,
    Werckmeister,
    Kirnberger,
    Custom,
}

impl TemperamentKind {
    pub const ALL: &'static [Self] = &[
        Self::TwelveToneEqualTemperament,
        Self::NineteenToneEqualTemperament,
        Self::TwentyFourToneEqualTemperament,
        Self::Just,
        Self::Pythagorean,
        Self::QuarterCommaMeantone,
        Self::Werckmeister,
        Self::Kirnberger,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TwelveToneEqualTemperament => "twelve-tone-equal-temperament",
            Self::NineteenToneEqualTemperament => "nineteen-tone-equal-temperament",
            Self::TwentyFourToneEqualTemperament => "twenty-four-tone-equal-temperament",
            Self::Just => "just",
            Self::Pythagorean => "pythagorean",
            Self::QuarterCommaMeantone => "quarter-comma-meantone",
            Self::Werckmeister => "werckmeister",
            Self::Kirnberger => "kirnberger",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for TemperamentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TemperamentKind {
    type Err = TuningError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "twelve-tone-equal-temperament" => Ok(Self::TwelveToneEqualTemperament),
            "nineteen-tone-equal-temperament" => Ok(Self::NineteenToneEqualTemperament),
            "twenty-four-tone-equal-temperament" => Ok(Self::TwentyFourToneEqualTemperament),
            "just" => Ok(Self::Just),
            "pythagorean" => Ok(Self::Pythagorean),
            "quarter-comma-meantone" => Ok(Self::QuarterCommaMeantone),
            "werckmeister" => Ok(Self::Werckmeister),
            "kirnberger" => Ok(Self::Kirnberger),
            "custom" => Ok(Self::Custom),
            _ => Err(TuningError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReferenceNote {
    A4,
    C4,
    Custom,
}

impl ReferenceNote {
    pub const ALL: &'static [Self] = &[Self::A4, Self::C4, Self::Custom];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A4 => "a4",
            Self::C4 => "c4",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ReferenceNote {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ReferenceNote {
    type Err = TuningError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "a4" => Ok(Self::A4),
            "c4" => Ok(Self::C4),
            "custom" => Ok(Self::Custom),
            _ => Err(TuningError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConcertPitchStandard {
    A440,
    A432,
    A415,
    Custom,
}

impl ConcertPitchStandard {
    pub const ALL: &'static [Self] = &[Self::A440, Self::A432, Self::A415, Self::Custom];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A440 => "a440",
            Self::A432 => "a432",
            Self::A415 => "a415",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ConcertPitchStandard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ConcertPitchStandard {
    type Err = TuningError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "a440" => Ok(Self::A440),
            "a432" => Ok(Self::A432),
            "a415" => Ok(Self::A415),
            "custom" => Ok(Self::Custom),
            _ => Err(TuningError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TuningRatio {
    numerator: f64,
    denominator: f64,
}

impl TuningRatio {
    pub fn new(numerator: f64, denominator: f64) -> Result<Self, TuningError> {
        if !numerator.is_finite() || !denominator.is_finite() {
            return Err(TuningError::NonFinite);
        }
        if numerator <= 0.0 || denominator <= 0.0 {
            return Err(TuningError::NonPositive);
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }
    pub const fn numerator(self) -> f64 {
        self.numerator
    }
    pub const fn denominator(self) -> f64 {
        self.denominator
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TuningError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for TuningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("tuning metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("tuning metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("tuning metadata value is out of range"),
            Self::NonFinite => formatter.write_str("tuning metadata value must be finite"),
            Self::NonPositive => formatter.write_str("tuning metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown tuning metadata label"),
        }
    }
}

impl Error for TuningError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, TuningError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(TuningError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, TuningError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(TuningError::Empty)
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
        Cents, ConcertPitchStandard, EqualTemperamentDivision, MicrotonalDivision, ReferenceNote,
        ReferencePitch, TemperamentKind, TuningError, TuningRatio, TuningSystem,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), TuningError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = TuningError>,
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
    fn validates_text_newtypes() -> Result<(), TuningError> {
        assert!(true);
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), TuningError> {
        let value = MicrotonalDivision::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<MicrotonalDivision>()?, value);
        assert_eq!(MicrotonalDivision::new(4097), Err(TuningError::OutOfRange));
        let value = EqualTemperamentDivision::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<EqualTemperamentDivision>()?, value);
        assert_eq!(
            EqualTemperamentDivision::new(4097),
            Err(TuningError::OutOfRange)
        );
        let value = ReferencePitch::new(1.0)?;
        assert_eq!(value.value(), 1.0);
        assert_eq!("1.0".parse::<ReferencePitch>()?, value);
        assert_eq!(ReferencePitch::new(f64::NAN), Err(TuningError::NonFinite));
        let value = Cents::new(1.0)?;
        assert_eq!(value.value(), 1.0);
        assert_eq!("1.0".parse::<Cents>()?, value);
        assert_eq!(Cents::new(f64::NAN), Err(TuningError::NonFinite));
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), TuningError> {
        assert_enum_family(TuningSystem::ALL)?;
        assert_enum_family(TemperamentKind::ALL)?;
        assert_enum_family(ReferenceNote::ALL)?;
        assert_enum_family(ConcertPitchStandard::ALL)?;
        Ok(())
    }

    #[test]
    fn validates_tuning_metadata() -> Result<(), TuningError> {
        let reference = ReferencePitch::new(440.0)?;
        let cents = Cents::new(-12.5)?;
        let ratio = TuningRatio::new(3.0, 2.0)?;
        assert_eq!(reference.value(), 440.0);
        assert_eq!(cents.value(), -12.5);
        assert_eq!(ratio.numerator(), 3.0);
        assert_eq!(
            EqualTemperamentDivision::new(0),
            Err(TuningError::OutOfRange)
        );
        Ok(())
    }
}
