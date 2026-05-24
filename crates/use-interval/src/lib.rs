#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        CompoundInterval, DiatonicStepDistance, IntervalDirection, IntervalError, IntervalName,
        IntervalNumber, IntervalQuality, SemitoneDistance, SimpleInterval,
    };
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
    DoubleAugmented,
    DoubleDiminished,
}

impl IntervalQuality {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Perfect => "perfect",
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Augmented => "augmented",
            Self::Diminished => "diminished",
            Self::DoubleAugmented => "double-augmented",
            Self::DoubleDiminished => "double-diminished",
        }
    }
}

impl fmt::Display for IntervalQuality {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IntervalDirection {
    Ascending,
    Descending,
    Unison,
}

impl IntervalDirection {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
            Self::Unison => "unison",
        }
    }
}

impl fmt::Display for IntervalDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IntervalNumber(u8);

impl IntervalNumber {
    pub fn new(value: u8) -> Result<Self, IntervalError> {
        if value == 0 || value > 64 {
            return Err(IntervalError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SemitoneDistance(i16);

impl SemitoneDistance {
    pub const fn new(value: i16) -> Self {
        Self(value)
    }

    pub const fn value(self) -> i16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiatonicStepDistance(i16);

impl DiatonicStepDistance {
    pub const fn new(value: i16) -> Self {
        Self(value)
    }

    pub const fn value(self) -> i16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SimpleInterval {
    quality: IntervalQuality,
    number: IntervalNumber,
    semitones: SemitoneDistance,
}

impl SimpleInterval {
    pub fn new(
        quality: IntervalQuality,
        number: u8,
        semitones: i16,
    ) -> Result<Self, IntervalError> {
        Ok(Self {
            quality,
            number: IntervalNumber::new(number)?,
            semitones: SemitoneDistance::new(semitones),
        })
    }

    pub fn perfect_unison() -> Self {
        Self::new(IntervalQuality::Perfect, 1, 0).expect("valid interval")
    }
    pub fn minor_second() -> Self {
        Self::new(IntervalQuality::Minor, 2, 1).expect("valid interval")
    }
    pub fn major_second() -> Self {
        Self::new(IntervalQuality::Major, 2, 2).expect("valid interval")
    }
    pub fn minor_third() -> Self {
        Self::new(IntervalQuality::Minor, 3, 3).expect("valid interval")
    }
    pub fn major_third() -> Self {
        Self::new(IntervalQuality::Major, 3, 4).expect("valid interval")
    }
    pub fn perfect_fourth() -> Self {
        Self::new(IntervalQuality::Perfect, 4, 5).expect("valid interval")
    }
    pub fn tritone() -> Self {
        Self::new(IntervalQuality::Augmented, 4, 6).expect("valid interval")
    }
    pub fn perfect_fifth() -> Self {
        Self::new(IntervalQuality::Perfect, 5, 7).expect("valid interval")
    }
    pub fn minor_sixth() -> Self {
        Self::new(IntervalQuality::Minor, 6, 8).expect("valid interval")
    }
    pub fn major_sixth() -> Self {
        Self::new(IntervalQuality::Major, 6, 9).expect("valid interval")
    }
    pub fn minor_seventh() -> Self {
        Self::new(IntervalQuality::Minor, 7, 10).expect("valid interval")
    }
    pub fn major_seventh() -> Self {
        Self::new(IntervalQuality::Major, 7, 11).expect("valid interval")
    }
    pub fn octave() -> Self {
        Self::new(IntervalQuality::Perfect, 8, 12).expect("valid interval")
    }

    pub const fn quality(self) -> IntervalQuality {
        self.quality
    }
    pub const fn number(self) -> IntervalNumber {
        self.number
    }
    pub const fn semitones(self) -> SemitoneDistance {
        self.semitones
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CompoundInterval {
    simple: SimpleInterval,
    octaves: u8,
}

impl CompoundInterval {
    pub const fn new(simple: SimpleInterval, octaves: u8) -> Self {
        Self { simple, octaves }
    }

    pub const fn simple(self) -> SimpleInterval {
        self.simple
    }
    pub const fn octaves(self) -> u8 {
        self.octaves
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IntervalName(String);

impl IntervalName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, IntervalError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            return Err(IntervalError::Empty);
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn value(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for IntervalName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for IntervalName {
    type Err = IntervalError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for IntervalName {
    type Error = IntervalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntervalError {
    Empty,
    OutOfRange,
}

impl fmt::Display for IntervalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("interval metadata text cannot be empty"),
            Self::OutOfRange => formatter.write_str("interval number is out of range"),
        }
    }
}

impl Error for IntervalError {}

#[cfg(test)]
#[allow(
    unused_imports,
    clippy::unnecessary_wraps,
    clippy::assertions_on_constants
)]
mod tests {
    use super::{IntervalError, IntervalName, IntervalNumber, IntervalQuality, SimpleInterval};

    #[test]
    fn validates_interval_numbers_and_names() -> Result<(), IntervalError> {
        assert_eq!(IntervalNumber::new(1)?.value(), 1);
        assert_eq!(IntervalNumber::new(0), Err(IntervalError::OutOfRange));
        assert_eq!(IntervalName::new(" octave ")?.as_str(), "octave");
        Ok(())
    }

    #[test]
    fn provides_common_intervals() {
        assert_eq!(SimpleInterval::perfect_unison().semitones().value(), 0);
        assert_eq!(SimpleInterval::minor_second().semitones().value(), 1);
        assert_eq!(
            SimpleInterval::major_third().quality(),
            IntervalQuality::Major
        );
        assert_eq!(SimpleInterval::perfect_fifth().semitones().value(), 7);
        assert_eq!(SimpleInterval::octave().number().value(), 8);
    }
}
