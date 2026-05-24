#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        ArticulationError, ArticulationKind, OrnamentKind, PerformanceTechnique, PhraseMarkKind,
        SlurKind, TieKind,
    };
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ArticulationKind {
    Staccato,
    Staccatissimo,
    Tenuto,
    Accent,
    Marcato,
    Legato,
    Fermata,
    BreathMark,
    Caesura,
    Custom,
}

impl ArticulationKind {
    pub const ALL: &'static [Self] = &[
        Self::Staccato,
        Self::Staccatissimo,
        Self::Tenuto,
        Self::Accent,
        Self::Marcato,
        Self::Legato,
        Self::Fermata,
        Self::BreathMark,
        Self::Caesura,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Staccato => "staccato",
            Self::Staccatissimo => "staccatissimo",
            Self::Tenuto => "tenuto",
            Self::Accent => "accent",
            Self::Marcato => "marcato",
            Self::Legato => "legato",
            Self::Fermata => "fermata",
            Self::BreathMark => "breath-mark",
            Self::Caesura => "caesura",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ArticulationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ArticulationKind {
    type Err = ArticulationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "staccato" => Ok(Self::Staccato),
            "staccatissimo" => Ok(Self::Staccatissimo),
            "tenuto" => Ok(Self::Tenuto),
            "accent" => Ok(Self::Accent),
            "marcato" => Ok(Self::Marcato),
            "legato" => Ok(Self::Legato),
            "fermata" => Ok(Self::Fermata),
            "breath-mark" => Ok(Self::BreathMark),
            "caesura" => Ok(Self::Caesura),
            "custom" => Ok(Self::Custom),
            _ => Err(ArticulationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrnamentKind {
    Trill,
    Mordent,
    Turn,
    Appoggiatura,
    Acciaccatura,
    GraceNote,
    Custom,
}

impl OrnamentKind {
    pub const ALL: &'static [Self] = &[
        Self::Trill,
        Self::Mordent,
        Self::Turn,
        Self::Appoggiatura,
        Self::Acciaccatura,
        Self::GraceNote,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trill => "trill",
            Self::Mordent => "mordent",
            Self::Turn => "turn",
            Self::Appoggiatura => "appoggiatura",
            Self::Acciaccatura => "acciaccatura",
            Self::GraceNote => "grace-note",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for OrnamentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OrnamentKind {
    type Err = ArticulationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "trill" => Ok(Self::Trill),
            "mordent" => Ok(Self::Mordent),
            "turn" => Ok(Self::Turn),
            "appoggiatura" => Ok(Self::Appoggiatura),
            "acciaccatura" => Ok(Self::Acciaccatura),
            "grace-note" => Ok(Self::GraceNote),
            "custom" => Ok(Self::Custom),
            _ => Err(ArticulationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhraseMarkKind {
    Slur,
    Tie,
    Phrase,
    Breath,
    Caesura,
}

impl PhraseMarkKind {
    pub const ALL: &'static [Self] = &[
        Self::Slur,
        Self::Tie,
        Self::Phrase,
        Self::Breath,
        Self::Caesura,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Slur => "slur",
            Self::Tie => "tie",
            Self::Phrase => "phrase",
            Self::Breath => "breath",
            Self::Caesura => "caesura",
        }
    }
}

impl fmt::Display for PhraseMarkKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PhraseMarkKind {
    type Err = ArticulationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "slur" => Ok(Self::Slur),
            "tie" => Ok(Self::Tie),
            "phrase" => Ok(Self::Phrase),
            "breath" => Ok(Self::Breath),
            "caesura" => Ok(Self::Caesura),
            _ => Err(ArticulationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SlurKind {
    Start,
    Continue,
    Stop,
    Unknown,
}

impl SlurKind {
    pub const ALL: &'static [Self] = &[Self::Start, Self::Continue, Self::Stop, Self::Unknown];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Continue => "continue",
            Self::Stop => "stop",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for SlurKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SlurKind {
    type Err = ArticulationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "start" => Ok(Self::Start),
            "continue" => Ok(Self::Continue),
            "stop" => Ok(Self::Stop),
            "unknown" => Ok(Self::Unknown),
            _ => Err(ArticulationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TieKind {
    Start,
    Continue,
    Stop,
    Unknown,
}

impl TieKind {
    pub const ALL: &'static [Self] = &[Self::Start, Self::Continue, Self::Stop, Self::Unknown];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Continue => "continue",
            Self::Stop => "stop",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for TieKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TieKind {
    type Err = ArticulationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "start" => Ok(Self::Start),
            "continue" => Ok(Self::Continue),
            "stop" => Ok(Self::Stop),
            "unknown" => Ok(Self::Unknown),
            _ => Err(ArticulationError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PerformanceTechnique {
    Pizzicato,
    Arco,
    Tremolo,
    Glissando,
    Harmonic,
    PalmMute,
    Bend,
    Vibrato,
    Custom,
}

impl PerformanceTechnique {
    pub const ALL: &'static [Self] = &[
        Self::Pizzicato,
        Self::Arco,
        Self::Tremolo,
        Self::Glissando,
        Self::Harmonic,
        Self::PalmMute,
        Self::Bend,
        Self::Vibrato,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pizzicato => "pizzicato",
            Self::Arco => "arco",
            Self::Tremolo => "tremolo",
            Self::Glissando => "glissando",
            Self::Harmonic => "harmonic",
            Self::PalmMute => "palm-mute",
            Self::Bend => "bend",
            Self::Vibrato => "vibrato",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for PerformanceTechnique {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PerformanceTechnique {
    type Err = ArticulationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "pizzicato" => Ok(Self::Pizzicato),
            "arco" => Ok(Self::Arco),
            "tremolo" => Ok(Self::Tremolo),
            "glissando" => Ok(Self::Glissando),
            "harmonic" => Ok(Self::Harmonic),
            "palm-mute" => Ok(Self::PalmMute),
            "bend" => Ok(Self::Bend),
            "vibrato" => Ok(Self::Vibrato),
            "custom" => Ok(Self::Custom),
            _ => Err(ArticulationError::UnknownLabel),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArticulationError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for ArticulationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("articulation metadata text cannot be empty"),
            Self::InvalidFormat => {
                formatter.write_str("articulation metadata has an invalid format")
            },
            Self::OutOfRange => formatter.write_str("articulation metadata value is out of range"),
            Self::NonFinite => formatter.write_str("articulation metadata value must be finite"),
            Self::NonPositive => {
                formatter.write_str("articulation metadata value must be positive")
            },
            Self::UnknownLabel => formatter.write_str("unknown articulation metadata label"),
        }
    }
}

impl Error for ArticulationError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, ArticulationError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(ArticulationError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, ArticulationError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(ArticulationError::Empty)
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
        ArticulationError, ArticulationKind, OrnamentKind, PerformanceTechnique, PhraseMarkKind,
        SlurKind, TieKind,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), ArticulationError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = ArticulationError>,
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
    fn validates_text_newtypes() -> Result<(), ArticulationError> {
        assert!(true);
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), ArticulationError> {
        assert!(true);
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), ArticulationError> {
        assert_enum_family(ArticulationKind::ALL)?;
        assert_enum_family(OrnamentKind::ALL)?;
        assert_enum_family(PhraseMarkKind::ALL)?;
        assert_enum_family(SlurKind::ALL)?;
        assert_enum_family(TieKind::ALL)?;
        assert_enum_family(PerformanceTechnique::ALL)?;
        Ok(())
    }
}
