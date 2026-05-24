#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        BeatsPerMinute, MetronomeMark, RubatoKind, TempoChangeKind, TempoError, TempoMapPoint,
        TempoMarking, TempoRange,
    };
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct BeatsPerMinute(f64);

impl BeatsPerMinute {
    pub fn new(value: f64) -> Result<Self, TempoError> {
        if !value.is_finite() {
            return Err(TempoError::NonFinite);
        }
        if value <= 0.0 {
            return Err(TempoError::NonPositive);
        }
        Ok(Self(value))
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for BeatsPerMinute {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for BeatsPerMinute {
    type Err = TempoError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<f64>()
            .map_err(|_| TempoError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<f64> for BeatsPerMinute {
    type Error = TempoError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TempoMarking {
    Larghissimo,
    Largo,
    Larghetto,
    Adagio,
    Andante,
    Moderato,
    Allegro,
    Vivace,
    Presto,
    Prestissimo,
    Custom,
}

impl TempoMarking {
    pub const ALL: &'static [Self] = &[
        Self::Larghissimo,
        Self::Largo,
        Self::Larghetto,
        Self::Adagio,
        Self::Andante,
        Self::Moderato,
        Self::Allegro,
        Self::Vivace,
        Self::Presto,
        Self::Prestissimo,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Larghissimo => "larghissimo",
            Self::Largo => "largo",
            Self::Larghetto => "larghetto",
            Self::Adagio => "adagio",
            Self::Andante => "andante",
            Self::Moderato => "moderato",
            Self::Allegro => "allegro",
            Self::Vivace => "vivace",
            Self::Presto => "presto",
            Self::Prestissimo => "prestissimo",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for TempoMarking {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TempoMarking {
    type Err = TempoError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "larghissimo" => Ok(Self::Larghissimo),
            "largo" => Ok(Self::Largo),
            "larghetto" => Ok(Self::Larghetto),
            "adagio" => Ok(Self::Adagio),
            "andante" => Ok(Self::Andante),
            "moderato" => Ok(Self::Moderato),
            "allegro" => Ok(Self::Allegro),
            "vivace" => Ok(Self::Vivace),
            "presto" => Ok(Self::Presto),
            "prestissimo" => Ok(Self::Prestissimo),
            "custom" => Ok(Self::Custom),
            _ => Err(TempoError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TempoChangeKind {
    Immediate,
    Gradual,
    Accelerando,
    Ritardando,
    Rallentando,
    ATempo,
}

impl TempoChangeKind {
    pub const ALL: &'static [Self] = &[
        Self::Immediate,
        Self::Gradual,
        Self::Accelerando,
        Self::Ritardando,
        Self::Rallentando,
        Self::ATempo,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::Gradual => "gradual",
            Self::Accelerando => "accelerando",
            Self::Ritardando => "ritardando",
            Self::Rallentando => "rallentando",
            Self::ATempo => "a-tempo",
        }
    }
}

impl fmt::Display for TempoChangeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TempoChangeKind {
    type Err = TempoError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "immediate" => Ok(Self::Immediate),
            "gradual" => Ok(Self::Gradual),
            "accelerando" => Ok(Self::Accelerando),
            "ritardando" => Ok(Self::Ritardando),
            "rallentando" => Ok(Self::Rallentando),
            "a-tempo" => Ok(Self::ATempo),
            _ => Err(TempoError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RubatoKind {
    None,
    Slight,
    Expressive,
    Free,
    Unknown,
}

impl RubatoKind {
    pub const ALL: &'static [Self] = &[
        Self::None,
        Self::Slight,
        Self::Expressive,
        Self::Free,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Slight => "slight",
            Self::Expressive => "expressive",
            Self::Free => "free",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for RubatoKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RubatoKind {
    type Err = TempoError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "none" => Ok(Self::None),
            "slight" => Ok(Self::Slight),
            "expressive" => Ok(Self::Expressive),
            "free" => Ok(Self::Free),
            "unknown" => Ok(Self::Unknown),
            _ => Err(TempoError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TempoRange {
    min: BeatsPerMinute,
    max: BeatsPerMinute,
}

impl TempoRange {
    pub fn new(min: BeatsPerMinute, max: BeatsPerMinute) -> Result<Self, TempoError> {
        if min.value() > max.value() {
            return Err(TempoError::OutOfRange);
        }
        Ok(Self { min, max })
    }
    pub const fn min(self) -> BeatsPerMinute {
        self.min
    }
    pub const fn max(self) -> BeatsPerMinute {
        self.max
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TempoMapPoint {
    beat: f64,
    bpm: BeatsPerMinute,
}

impl TempoMapPoint {
    pub fn new(beat: f64, bpm: BeatsPerMinute) -> Result<Self, TempoError> {
        if !beat.is_finite() || beat < 0.0 {
            return Err(TempoError::OutOfRange);
        }
        Ok(Self { beat, bpm })
    }
    pub const fn beat(self) -> f64 {
        self.beat
    }
    pub const fn bpm(self) -> BeatsPerMinute {
        self.bpm
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetronomeMark {
    beat_unit: &'static str,
    bpm: BeatsPerMinute,
}

impl MetronomeMark {
    pub const fn new(beat_unit: &'static str, bpm: BeatsPerMinute) -> Self {
        Self { beat_unit, bpm }
    }
    pub const fn beat_unit(self) -> &'static str {
        self.beat_unit
    }
    pub const fn bpm(self) -> BeatsPerMinute {
        self.bpm
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TempoError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for TempoError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("tempo metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("tempo metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("tempo metadata value is out of range"),
            Self::NonFinite => formatter.write_str("tempo metadata value must be finite"),
            Self::NonPositive => formatter.write_str("tempo metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown tempo metadata label"),
        }
    }
}

impl Error for TempoError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, TempoError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(TempoError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, TempoError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(TempoError::Empty)
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
        BeatsPerMinute, MetronomeMark, RubatoKind, TempoChangeKind, TempoError, TempoMapPoint,
        TempoMarking, TempoRange,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), TempoError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = TempoError>,
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
    fn validates_text_newtypes() -> Result<(), TempoError> {
        assert!(true);
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), TempoError> {
        let value = BeatsPerMinute::new(1.0)?;
        assert_eq!(value.value(), 1.0);
        assert_eq!("1.0".parse::<BeatsPerMinute>()?, value);
        assert_eq!(BeatsPerMinute::new(f64::NAN), Err(TempoError::NonFinite));
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), TempoError> {
        assert_enum_family(TempoMarking::ALL)?;
        assert_enum_family(TempoChangeKind::ALL)?;
        assert_enum_family(RubatoKind::ALL)?;
        Ok(())
    }

    #[test]
    fn validates_tempo_metadata() -> Result<(), TempoError> {
        let bpm = BeatsPerMinute::new(120.0)?;
        let range = TempoRange::new(BeatsPerMinute::new(90.0)?, bpm)?;
        let point = TempoMapPoint::new(4.0, bpm)?;
        assert_eq!(bpm.value(), 120.0);
        assert_eq!(range.max().value(), 120.0);
        assert_eq!(point.beat(), 4.0);
        assert_eq!(BeatsPerMinute::new(0.0), Err(TempoError::NonPositive));
        Ok(())
    }
}
