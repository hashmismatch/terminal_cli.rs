//! Internationalization support for various command line strings.

use prelude::v1::*;
use terminal::CharacterTerminalWriter;

pub trait Strings {
    fn property_invalid_value(&self, f: &mut CharacterTerminalWriter, id: &str, input: &str) -> Result<(), FmtError> {
        write!(f, "{}: Unable to parse the value, input was '{}'.", id, input)
    }

    fn property_value_too_small(&self, f: &mut CharacterTerminalWriter, id: &str, val: &Display, min: &Display) -> Result<(), FmtError> {
        write!(f, "{}: Value {} is too small, the minimum value is {}.", id, val, min)
    }

    fn property_value_too_big(&self, f: &mut CharacterTerminalWriter, id: &str, val: &Display, max: &Display) -> Result<(), FmtError> {
        write!(f, "{}: Value {} is too large, the maximum value is {}.", id, val, max)
    }

    fn property_value_set(&self, f: &mut CharacterTerminalWriter, id: &str, val: &Display) -> Result<(), FmtError> {
        write!(f, "New value for {} is {}.", id, val)
    }

    fn cmd_not_recognized(&self, f: &mut CharacterTerminalWriter, cmd: &str) -> Result<(), FmtError> {
        write!(f, "Command not recognized.")
    }
}

#[derive(Copy, Default, Clone)]
pub struct English;
impl Strings for English {

}
