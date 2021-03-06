//! Loading and playing of audio files.

pub use self::audio_context::AudioContext;
pub use self::dj::Dj;
pub use self::source::Source;

pub mod play;
pub mod output;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

mod audio_context;
mod dj;
mod source;

/// An error occurred while decoding the source.
#[derive(Debug)]
pub struct DecoderError;

impl Display for DecoderError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("DecoderError")
    }
}

impl Error for DecoderError {
    fn description(&self) -> &str {
        "An error occurred while decoding sound data."
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
