#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        BeatDivision, DottedDuration, DurationValue, NoteDuration, RestDuration, RhythmError,
        RhythmPatternName, RhythmicPosition, SyncopationKind, TupletRatio,
    };
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RhythmPatternName(String);

impl RhythmPatternName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, RhythmError> {
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

impl AsRef<str> for RhythmPatternName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RhythmPatternName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RhythmPatternName {
    type Err = RhythmError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for RhythmPatternName {
    type Error = RhythmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DurationValue {
    DoubleWhole,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
    OneTwentyEighth,
}

impl DurationValue {
    pub const ALL: &'static [Self] = &[
        Self::DoubleWhole,
        Self::Whole,
        Self::Half,
        Self::Quarter,
        Self::Eighth,
        Self::Sixteenth,
        Self::ThirtySecond,
        Self::SixtyFourth,
        Self::OneTwentyEighth,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoubleWhole => "double-whole",
            Self::Whole => "whole",
            Self::Half => "half",
            Self::Quarter => "quarter",
            Self::Eighth => "eighth",
            Self::Sixteenth => "sixteenth",
            Self::ThirtySecond => "thirty-second",
            Self::SixtyFourth => "sixty-fourth",
            Self::OneTwentyEighth => "one-twenty-eighth",
        }
    }
}

impl fmt::Display for DurationValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DurationValue {
    type Err = RhythmError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "double-whole" => Ok(Self::DoubleWhole),
            "whole" => Ok(Self::Whole),
            "half" => Ok(Self::Half),
            "quarter" => Ok(Self::Quarter),
            "eighth" => Ok(Self::Eighth),
            "sixteenth" => Ok(Self::Sixteenth),
            "thirty-second" => Ok(Self::ThirtySecond),
            "sixty-fourth" => Ok(Self::SixtyFourth),
            "one-twenty-eighth" => Ok(Self::OneTwentyEighth),
            _ => Err(RhythmError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BeatDivision {
    Duple,
    Triple,
    Quadruple,
    Quintuple,
    Septuple,
    Custom,
}

impl BeatDivision {
    pub const ALL: &'static [Self] = &[
        Self::Duple,
        Self::Triple,
        Self::Quadruple,
        Self::Quintuple,
        Self::Septuple,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Duple => "duple",
            Self::Triple => "triple",
            Self::Quadruple => "quadruple",
            Self::Quintuple => "quintuple",
            Self::Septuple => "septuple",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for BeatDivision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BeatDivision {
    type Err = RhythmError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "duple" => Ok(Self::Duple),
            "triple" => Ok(Self::Triple),
            "quadruple" => Ok(Self::Quadruple),
            "quintuple" => Ok(Self::Quintuple),
            "septuple" => Ok(Self::Septuple),
            "custom" => Ok(Self::Custom),
            _ => Err(RhythmError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SyncopationKind {
    None,
    WeakBeatAccent,
    OffBeat,
    Anticipation,
    Suspension,
    Unknown,
}

impl SyncopationKind {
    pub const ALL: &'static [Self] = &[
        Self::None,
        Self::WeakBeatAccent,
        Self::OffBeat,
        Self::Anticipation,
        Self::Suspension,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::WeakBeatAccent => "weak-beat-accent",
            Self::OffBeat => "off-beat",
            Self::Anticipation => "anticipation",
            Self::Suspension => "suspension",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for SyncopationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SyncopationKind {
    type Err = RhythmError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "none" => Ok(Self::None),
            "weak-beat-accent" => Ok(Self::WeakBeatAccent),
            "off-beat" => Ok(Self::OffBeat),
            "anticipation" => Ok(Self::Anticipation),
            "suspension" => Ok(Self::Suspension),
            "unknown" => Ok(Self::Unknown),
            _ => Err(RhythmError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NoteDuration {
    value: DurationValue,
}

impl NoteDuration {
    pub const fn new(value: DurationValue) -> Self {
        Self { value }
    }
    pub const fn value(self) -> DurationValue {
        self.value
    }
    pub const fn is_rest(self) -> bool {
        false
    }
    pub const fn is_shorter_than_quarter_like(self) -> bool {
        self.value.is_shorter_than_quarter_like()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RestDuration {
    value: DurationValue,
}

impl RestDuration {
    pub const fn new(value: DurationValue) -> Self {
        Self { value }
    }
    pub const fn value(self) -> DurationValue {
        self.value
    }
    pub const fn is_rest(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TupletRatio {
    actual: u8,
    normal: u8,
}

impl TupletRatio {
    pub fn new(actual: u8, normal: u8) -> Result<Self, RhythmError> {
        if actual == 0 || normal == 0 {
            return Err(RhythmError::OutOfRange);
        }
        Ok(Self { actual, normal })
    }
    pub const fn actual(self) -> u8 {
        self.actual
    }
    pub const fn normal(self) -> u8 {
        self.normal
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DottedDuration {
    value: DurationValue,
    dots: u8,
}

impl DottedDuration {
    pub const fn new(value: DurationValue, dots: u8) -> Self {
        Self { value, dots }
    }
    pub const fn value(self) -> DurationValue {
        self.value
    }
    pub const fn dot_count(self) -> u8 {
        self.dots
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RhythmicPosition {
    beat: u16,
    subdivision: u16,
}

impl RhythmicPosition {
    pub const fn new(beat: u16, subdivision: u16) -> Self {
        Self { beat, subdivision }
    }
    pub const fn beat(self) -> u16 {
        self.beat
    }
    pub const fn subdivision(self) -> u16 {
        self.subdivision
    }
}

impl DurationValue {
    pub const fn is_shorter_than_quarter_like(self) -> bool {
        matches!(
            self,
            Self::Eighth
                | Self::Sixteenth
                | Self::ThirtySecond
                | Self::SixtyFourth
                | Self::OneTwentyEighth
        )
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RhythmError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for RhythmError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("rhythm metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("rhythm metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("rhythm metadata value is out of range"),
            Self::NonFinite => formatter.write_str("rhythm metadata value must be finite"),
            Self::NonPositive => formatter.write_str("rhythm metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown rhythm metadata label"),
        }
    }
}

impl Error for RhythmError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, RhythmError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(RhythmError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, RhythmError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(RhythmError::Empty)
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
        BeatDivision, DottedDuration, DurationValue, NoteDuration, RestDuration, RhythmError,
        RhythmPatternName, RhythmicPosition, SyncopationKind, TupletRatio,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), RhythmError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = RhythmError>,
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
    fn validates_text_newtypes() -> Result<(), RhythmError> {
        let value = RhythmPatternName::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(
            <RhythmPatternName as TryFrom<&str>>::try_from("example-value")?,
            value
        );
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), RhythmError> {
        assert!(true);
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), RhythmError> {
        assert_enum_family(DurationValue::ALL)?;
        assert_enum_family(BeatDivision::ALL)?;
        assert_enum_family(SyncopationKind::ALL)?;
        Ok(())
    }

    #[test]
    fn models_symbolic_durations() -> Result<(), RhythmError> {
        let note = NoteDuration::new(DurationValue::Eighth);
        let rest = RestDuration::new(DurationValue::Quarter);
        let dotted = DottedDuration::new(DurationValue::Half, 2);
        let tuplet = TupletRatio::new(3, 2)?;
        assert!(note.is_shorter_than_quarter_like());
        assert!(!note.is_rest());
        assert!(rest.is_rest());
        assert_eq!(dotted.dot_count(), 2);
        assert_eq!(tuplet.actual(), 3);
        Ok(())
    }
}
