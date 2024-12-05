/// A MIDI channel.
#[derive(Debug)]
pub enum Channel {
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
    Channel9,
    Channel10,
    Channel11,
    Channel12,
    Channel13,
    Channel14,
    Channel15,
    Channel16,
}

impl From<u8> for Channel {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Channel1,
            1 => Self::Channel2,
            2 => Self::Channel3,
            3 => Self::Channel4,
            4 => Self::Channel5,
            5 => Self::Channel6,
            6 => Self::Channel7,
            7 => Self::Channel8,
            8 => Self::Channel9,
            9 => Self::Channel10,
            10 => Self::Channel11,
            11 => Self::Channel12,
            12 => Self::Channel13,
            13 => Self::Channel14,
            14 => Self::Channel15,
            _ => Self::Channel16,
        }
    }
}

/// A MIDI message.  
/// Refer to [this table](https://midi.org/expanded-midi-1-0-messages-list)
/// for more info.
#[derive(Debug)]
pub enum Message<'a> {
    // TODO: Implement a struct for 7-bit values
    // TODO: Implement an enum for notes
    NoteOff(Channel, u8, u8),
    NoteOn(Channel, u8, u8),
    PolyphonicAftertouch(Channel, u8, u8),
    /* TODO: Implement MIDI spec table 3
    https://midi.org/midi-1-0-control-change-messages */
    ControlOrModeChange(Channel, u8, u8),
    ProgramChange(Channel, u8),
    Aftertouch(Channel, u8),
    PitchBendChange(Channel, u8, u8),
    SystemExclusive(&'a [u8]),
    // TODO: Determine whether to implement the quarter frame (242)
    SongPositionPointer(u8, u8),
    SongSelect(u8),
    TuneRequest,
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    SystemReset,
}

impl<'a> From<&'a [u8]> for Message<'a> {
    fn from(value: &'a [u8]) -> Self {
        let first_byte = if value.len() >= 1 {
            value[0]
        } else {
            panic!("A MIDI message can't be zero bytes long!");
        };

        let get_second_byte = || {
            if value.len() >= 2 {
                value[1]
            } else {
                // TODO: Add an error message
                panic!("No second byte present!")
            }
        };

        let get_third_byte = || {
            if value.len() >= 3 {
                value[2]
            } else {
                panic!("No third byte present!")
            }
        };

        match first_byte {
            128..=143 => Self::NoteOff(
                Channel::from(first_byte - 128),
                get_second_byte(),
                get_third_byte(),
            ),
            144..=159 => Self::NoteOn(
                Channel::from(first_byte - 144),
                get_second_byte(),
                get_third_byte(),
            ),
            160..=175 => Self::PolyphonicAftertouch(
                Channel::from(first_byte - 145),
                get_second_byte(),
                get_third_byte(),
            ),
            176..=191 => Self::ControlOrModeChange(
                Channel::from(first_byte - 176),
                get_second_byte(),
                get_third_byte(),
            ),
            192..=207 => Self::ProgramChange(Channel::from(first_byte - 192), get_second_byte()),
            208..=223 => Self::Aftertouch(Channel::from(first_byte - 208), get_second_byte()),
            224..=239 => Self::PitchBendChange(
                Channel::from(first_byte),
                get_second_byte(),
                get_third_byte(),
            ),
            240 => Self::SystemExclusive(&value[1..value.len() - 1]),
            242 => Self::SongPositionPointer(get_second_byte(), get_third_byte()),
            243 => Self::SongSelect(get_second_byte()),
            246 => Self::TuneRequest,
            248 => Self::TimingClock,
            250 => Self::Start,
            251 => Self::Continue,
            252 => Self::Stop,
            254 => Self::ActiveSensing,
            255 => Self::SystemReset,
            // TODO: Add error handling
            _ => panic!("Invalid MIDI message!"),
        }
    }
}
