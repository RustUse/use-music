#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        MidiChannel, MidiControllerNumber, MidiDeviceKind, MidiError, MidiEventKind,
        MidiMessageKind, MidiNoteNumber, MidiPortName, MidiProfileKind, MidiProgramNumber,
        MidiPropertyKind, MidiUmpMessageKind, MidiVelocity, MidiVersion,
    };
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MidiPortName(String);

impl MidiPortName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, MidiError> {
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

impl AsRef<str> for MidiPortName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for MidiPortName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MidiPortName {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for MidiPortName {
    type Error = MidiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MidiChannel(u8);

impl MidiChannel {
    pub fn new(value: u8) -> Result<Self, MidiError> {
        if !(1..=16).contains(&value) {
            return Err(MidiError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for MidiChannel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for MidiChannel {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| MidiError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for MidiChannel {
    type Error = MidiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MidiNoteNumber(u8);

impl MidiNoteNumber {
    pub fn new(value: u8) -> Result<Self, MidiError> {
        if !(0..=127).contains(&value) {
            return Err(MidiError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for MidiNoteNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for MidiNoteNumber {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| MidiError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for MidiNoteNumber {
    type Error = MidiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MidiVelocity(u8);

impl MidiVelocity {
    pub fn new(value: u8) -> Result<Self, MidiError> {
        if !(0..=127).contains(&value) {
            return Err(MidiError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for MidiVelocity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for MidiVelocity {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| MidiError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for MidiVelocity {
    type Error = MidiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MidiControllerNumber(u8);

impl MidiControllerNumber {
    pub fn new(value: u8) -> Result<Self, MidiError> {
        if !(0..=127).contains(&value) {
            return Err(MidiError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for MidiControllerNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for MidiControllerNumber {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| MidiError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for MidiControllerNumber {
    type Error = MidiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MidiProgramNumber(u8);

impl MidiProgramNumber {
    pub fn new(value: u8) -> Result<Self, MidiError> {
        if !(0..=127).contains(&value) {
            return Err(MidiError::OutOfRange);
        }

        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for MidiProgramNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for MidiProgramNumber {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<u8>()
            .map_err(|_| MidiError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl TryFrom<u8> for MidiProgramNumber {
    type Error = MidiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MidiVersion {
    Midi1,
    Midi2,
}

impl MidiVersion {
    pub const ALL: &'static [Self] = &[Self::Midi1, Self::Midi2];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Midi1 => "midi-1",
            Self::Midi2 => "midi-2",
        }
    }
}

impl fmt::Display for MidiVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MidiVersion {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "midi-1" => Ok(Self::Midi1),
            "midi-2" => Ok(Self::Midi2),
            _ => Err(MidiError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MidiMessageKind {
    NoteOff,
    NoteOn,
    PolyphonicKeyPressure,
    ControlChange,
    ProgramChange,
    ChannelPressure,
    PitchBend,
    SystemExclusive,
    SystemCommon,
    SystemRealTime,
    UniversalMidiPacket,
    Unknown,
}

impl MidiMessageKind {
    pub const ALL: &'static [Self] = &[
        Self::NoteOff,
        Self::NoteOn,
        Self::PolyphonicKeyPressure,
        Self::ControlChange,
        Self::ProgramChange,
        Self::ChannelPressure,
        Self::PitchBend,
        Self::SystemExclusive,
        Self::SystemCommon,
        Self::SystemRealTime,
        Self::UniversalMidiPacket,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoteOff => "note-off",
            Self::NoteOn => "note-on",
            Self::PolyphonicKeyPressure => "polyphonic-key-pressure",
            Self::ControlChange => "control-change",
            Self::ProgramChange => "program-change",
            Self::ChannelPressure => "channel-pressure",
            Self::PitchBend => "pitch-bend",
            Self::SystemExclusive => "system-exclusive",
            Self::SystemCommon => "system-common",
            Self::SystemRealTime => "system-real-time",
            Self::UniversalMidiPacket => "universal-midi-packet",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for MidiMessageKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MidiMessageKind {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "note-off" => Ok(Self::NoteOff),
            "note-on" => Ok(Self::NoteOn),
            "polyphonic-key-pressure" => Ok(Self::PolyphonicKeyPressure),
            "control-change" => Ok(Self::ControlChange),
            "program-change" => Ok(Self::ProgramChange),
            "channel-pressure" => Ok(Self::ChannelPressure),
            "pitch-bend" => Ok(Self::PitchBend),
            "system-exclusive" => Ok(Self::SystemExclusive),
            "system-common" => Ok(Self::SystemCommon),
            "system-real-time" => Ok(Self::SystemRealTime),
            "universal-midi-packet" => Ok(Self::UniversalMidiPacket),
            "unknown" => Ok(Self::Unknown),
            _ => Err(MidiError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MidiEventKind {
    ChannelVoice,
    System,
    Meta,
    Sysex,
    Ump,
    Unknown,
}

impl MidiEventKind {
    pub const ALL: &'static [Self] = &[
        Self::ChannelVoice,
        Self::System,
        Self::Meta,
        Self::Sysex,
        Self::Ump,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelVoice => "channel-voice",
            Self::System => "system",
            Self::Meta => "meta",
            Self::Sysex => "sysex",
            Self::Ump => "ump",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for MidiEventKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MidiEventKind {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "channel-voice" => Ok(Self::ChannelVoice),
            "system" => Ok(Self::System),
            "meta" => Ok(Self::Meta),
            "sysex" => Ok(Self::Sysex),
            "ump" => Ok(Self::Ump),
            "unknown" => Ok(Self::Unknown),
            _ => Err(MidiError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MidiDeviceKind {
    Keyboard,
    Controller,
    Synthesizer,
    DrumMachine,
    Sequencer,
    Daw,
    Interface,
    Unknown,
}

impl MidiDeviceKind {
    pub const ALL: &'static [Self] = &[
        Self::Keyboard,
        Self::Controller,
        Self::Synthesizer,
        Self::DrumMachine,
        Self::Sequencer,
        Self::Daw,
        Self::Interface,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keyboard => "keyboard",
            Self::Controller => "controller",
            Self::Synthesizer => "synthesizer",
            Self::DrumMachine => "drum-machine",
            Self::Sequencer => "sequencer",
            Self::Daw => "daw",
            Self::Interface => "interface",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for MidiDeviceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MidiDeviceKind {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "keyboard" => Ok(Self::Keyboard),
            "controller" => Ok(Self::Controller),
            "synthesizer" => Ok(Self::Synthesizer),
            "drum-machine" => Ok(Self::DrumMachine),
            "sequencer" => Ok(Self::Sequencer),
            "daw" => Ok(Self::Daw),
            "interface" => Ok(Self::Interface),
            "unknown" => Ok(Self::Unknown),
            _ => Err(MidiError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MidiProfileKind {
    DefaultControlChange,
    Mpe,
    DrawbarOrgan,
    OrchestralArticulation,
    Custom,
}

impl MidiProfileKind {
    pub const ALL: &'static [Self] = &[
        Self::DefaultControlChange,
        Self::Mpe,
        Self::DrawbarOrgan,
        Self::OrchestralArticulation,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultControlChange => "default-control-change",
            Self::Mpe => "mpe",
            Self::DrawbarOrgan => "drawbar-organ",
            Self::OrchestralArticulation => "orchestral-articulation",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for MidiProfileKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MidiProfileKind {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "default-control-change" => Ok(Self::DefaultControlChange),
            "mpe" => Ok(Self::Mpe),
            "drawbar-organ" => Ok(Self::DrawbarOrgan),
            "orchestral-articulation" => Ok(Self::OrchestralArticulation),
            "custom" => Ok(Self::Custom),
            _ => Err(MidiError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MidiPropertyKind {
    PerNoteExpression,
    PropertyExchange,
    ProfileConfiguration,
    EndpointInfo,
    Custom,
}

impl MidiPropertyKind {
    pub const ALL: &'static [Self] = &[
        Self::PerNoteExpression,
        Self::PropertyExchange,
        Self::ProfileConfiguration,
        Self::EndpointInfo,
        Self::Custom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerNoteExpression => "per-note-expression",
            Self::PropertyExchange => "property-exchange",
            Self::ProfileConfiguration => "profile-configuration",
            Self::EndpointInfo => "endpoint-info",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for MidiPropertyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MidiPropertyKind {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "per-note-expression" => Ok(Self::PerNoteExpression),
            "property-exchange" => Ok(Self::PropertyExchange),
            "profile-configuration" => Ok(Self::ProfileConfiguration),
            "endpoint-info" => Ok(Self::EndpointInfo),
            "custom" => Ok(Self::Custom),
            _ => Err(MidiError::UnknownLabel),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MidiUmpMessageKind {
    Utility,
    System,
    Midi1ChannelVoice,
    Data64,
    Midi2ChannelVoice,
    Data128,
    FlexData,
    Unknown,
}

impl MidiUmpMessageKind {
    pub const ALL: &'static [Self] = &[
        Self::Utility,
        Self::System,
        Self::Midi1ChannelVoice,
        Self::Data64,
        Self::Midi2ChannelVoice,
        Self::Data128,
        Self::FlexData,
        Self::Unknown,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Utility => "utility",
            Self::System => "system",
            Self::Midi1ChannelVoice => "midi-1-channel-voice",
            Self::Data64 => "data-64",
            Self::Midi2ChannelVoice => "midi-2-channel-voice",
            Self::Data128 => "data-128",
            Self::FlexData => "flex-data",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for MidiUmpMessageKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MidiUmpMessageKind {
    type Err = MidiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalized_label(value)?.as_str() {
            "utility" => Ok(Self::Utility),
            "system" => Ok(Self::System),
            "midi-1-channel-voice" => Ok(Self::Midi1ChannelVoice),
            "data-64" => Ok(Self::Data64),
            "midi-2-channel-voice" => Ok(Self::Midi2ChannelVoice),
            "data-128" => Ok(Self::Data128),
            "flex-data" => Ok(Self::FlexData),
            "unknown" => Ok(Self::Unknown),
            _ => Err(MidiError::UnknownLabel),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MidiError {
    Empty,
    InvalidFormat,
    OutOfRange,
    NonFinite,
    NonPositive,
    UnknownLabel,
}

impl fmt::Display for MidiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("MIDI metadata text cannot be empty"),
            Self::InvalidFormat => formatter.write_str("MIDI metadata has an invalid format"),
            Self::OutOfRange => formatter.write_str("MIDI metadata value is out of range"),
            Self::NonFinite => formatter.write_str("MIDI metadata value must be finite"),
            Self::NonPositive => formatter.write_str("MIDI metadata value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown MIDI metadata label"),
        }
    }
}

impl Error for MidiError {}

#[allow(dead_code)]
fn non_empty_text(value: impl AsRef<str>) -> Result<String, MidiError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(MidiError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, MidiError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(MidiError::Empty)
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
        MidiChannel, MidiControllerNumber, MidiDeviceKind, MidiError, MidiEventKind,
        MidiMessageKind, MidiNoteNumber, MidiPortName, MidiProfileKind, MidiProgramNumber,
        MidiPropertyKind, MidiUmpMessageKind, MidiVelocity, MidiVersion,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), MidiError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = MidiError>,
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
    fn validates_text_newtypes() -> Result<(), MidiError> {
        let value = MidiPortName::new(" example-value ")?;
        assert_eq!(value.as_str(), "example-value");
        assert_eq!(value.value(), "example-value");
        assert_eq!(value.to_string(), "example-value");
        assert_eq!(
            <MidiPortName as TryFrom<&str>>::try_from("example-value")?,
            value
        );
        Ok(())
    }

    #[test]
    fn validates_numeric_newtypes() -> Result<(), MidiError> {
        let value = MidiChannel::new(1)?;
        assert_eq!(value.value(), 1);
        assert_eq!("1".parse::<MidiChannel>()?, value);
        assert_eq!(MidiChannel::new(17), Err(MidiError::OutOfRange));
        let value = MidiNoteNumber::new(0)?;
        assert_eq!(value.value(), 0);
        assert_eq!("0".parse::<MidiNoteNumber>()?, value);
        assert_eq!(MidiNoteNumber::new(128), Err(MidiError::OutOfRange));
        let value = MidiVelocity::new(0)?;
        assert_eq!(value.value(), 0);
        assert_eq!("0".parse::<MidiVelocity>()?, value);
        assert_eq!(MidiVelocity::new(128), Err(MidiError::OutOfRange));
        let value = MidiControllerNumber::new(0)?;
        assert_eq!(value.value(), 0);
        assert_eq!("0".parse::<MidiControllerNumber>()?, value);
        assert_eq!(MidiControllerNumber::new(128), Err(MidiError::OutOfRange));
        let value = MidiProgramNumber::new(0)?;
        assert_eq!(value.value(), 0);
        assert_eq!("0".parse::<MidiProgramNumber>()?, value);
        assert_eq!(MidiProgramNumber::new(128), Err(MidiError::OutOfRange));
        Ok(())
    }

    #[test]
    fn displays_and_parses_enums() -> Result<(), MidiError> {
        assert_enum_family(MidiVersion::ALL)?;
        assert_enum_family(MidiMessageKind::ALL)?;
        assert_enum_family(MidiEventKind::ALL)?;
        assert_enum_family(MidiDeviceKind::ALL)?;
        assert_enum_family(MidiProfileKind::ALL)?;
        assert_enum_family(MidiPropertyKind::ALL)?;
        assert_enum_family(MidiUmpMessageKind::ALL)?;
        Ok(())
    }
}
