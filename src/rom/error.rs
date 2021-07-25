use std::num::ParseIntError;
extern crate thiserror;
use self::thiserror::Error;

#[macro_export]
macro_rules! parse_error {
    ($($arg:tt)*) => {
        super::error::ParseError(format!($($arg)*))
    }
}

#[derive(Error, Debug)]
pub enum FF6Error
{
    // Normal Errors:
    #[error("Compression Error: `{0}`")]
    CompressionError(String),
    #[error("Decompression Error: `{0}`")]
    DecompressionError(String),
    #[error("Error Parsing: `{0}`")]
    ParseError(String),
    #[error("Error Parsing: `invalid hex string '{0}'`")]
    HexError(String),
    #[error("Error Parsing: `invalid hex range '{0}'`")]
    HexRangeError(String),
    // Wrap Other Errors:
    #[error("Error Parsing: `{0} '{1}'`")]
    HexWrapError(ParseIntError, String),
    #[error("Error Parsing: `{0} '{1}'`")]
    HexRangeWrapError(ParseIntError, String),
    #[error("Error Parsing JSON: `{0}`")]
    JsonError(serde_json::Error),
}

#[allow(non_snake_case)]
pub fn CompressionError<S: Into<String>>(s: S) -> FF6Error
{
    FF6Error::CompressionError(s.into())
}
#[allow(non_snake_case)]
pub fn DecompressionError<S: Into<String>>(s: S) -> FF6Error
{
    FF6Error::DecompressionError(s.into())
}

#[allow(non_snake_case)]
pub fn ParseError<S: Into<String>>(s: S) -> FF6Error
{
    FF6Error::ParseError(s.into())
}

#[allow(non_snake_case)]
pub fn HexError<S: Into<String>>(s: S) -> FF6Error
{
    FF6Error::HexError(s.into())
}

#[allow(non_snake_case)]
pub fn HexRangeError<S: Into<String>>(s: S) -> FF6Error
{
    FF6Error::HexRangeError(s.into())
}

#[allow(non_snake_case)]
pub fn HexWrapError<S: Into<String>>(e: ParseIntError, s: S) -> FF6Error
{
    FF6Error::HexWrapError(e, s.into())
}

#[allow(non_snake_case)]
pub fn HexRangeWrapError<S: Into<String>>(e: ParseIntError, s: S) -> FF6Error
{
    FF6Error::HexRangeWrapError(e, s.into())
}

#[allow(non_snake_case)]
pub fn JsonError(e: serde_json::Error) -> FF6Error
{
    FF6Error::JsonError(e)
}
