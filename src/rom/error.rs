use std::num::ParseIntError;
extern crate thiserror;
use self::thiserror::Error;

#[macro_export]
macro_rules! parse_error {
    ($($arg:tt)*) => {
        super::error::ParseError(format!($($arg)*))
    }
}

macro_rules! one_param_fn {
    ($($arg:tt),*) => {

        $(  #[allow(non_snake_case)]
            pub fn $arg<S: Into<String>>(s: S) -> FF6Error {
            FF6Error::$arg(s.into()) }
        )*
    }
}

macro_rules! two_param_fn {
    ($($arg:tt),*) => {

        $(  #[allow(non_snake_case)]
            pub fn $arg<S: Into<String>>(n: S, s: S) -> FF6Error {
            FF6Error::$arg(n.into(), s.into())
        } )*
    }
}

#[derive(Error, Debug)]
pub enum FF6Error
{
    // From Errors:
    #[error("Error Parsing JSON: `{source}`")]
    JsonError
    {
        #[from]
        source: serde_json::Error,
    },
    #[error("Error Opening File: `{source}`")]
    FileError
    {
        #[from]
        source: std::io::Error,
    },
    // Normal Errors:
    #[error("Compression Error: `{0}`")]
    CompressionError(String),
    #[error("Decompression Error: `{0}`")]
    DecompressionError(String),
    #[error("Error Parsing: `{0}`")]
    ParseError(String),
    #[error("Error Parsing: invalid hex string `{0}`")]
    HexError(String),
    #[error("Error Parsing: invalid hex string range `{0}`")]
    HexRangeError(String),
    // Wrap Other Errors:
    #[error("Error Parsing: number `0x{0}` too large to fit in target type for hex string `{1}`")]
    HexPosOverflowError(String, String),
    #[error("Error Parsing: number `0x{0}` too small to fit in target type for hex string `{1}`")]
    HexNegOverflowError(String, String),
    #[error(
        "Error Parsing: number `0x{0}` number would be zero for non-zero type for hex string `{1}`"
    )]
    HexZeroError(String, String),
}

one_param_fn!(CompressionError, DecompressionError, ParseError, HexError, HexRangeError);
two_param_fn!(HexPosOverflowError, HexNegOverflowError, HexZeroError);
