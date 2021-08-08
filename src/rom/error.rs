extern crate thiserror;
use self::thiserror::Error;

pub type FnError<S> = fn(S) -> Error;

#[macro_export]
macro_rules! JsonError {
    ($($arg:tt)*) => {
        super::error::JsonError(format!($($arg)*))
    }
}

macro_rules! nil_param_fn {
    ($($arg:tt),*) => {

        $(  #[allow(non_snake_case)]
            pub fn $arg() -> Error {
            Error::$arg() }
        )*
    }
}

macro_rules! one_param_fn {
    ($($arg:tt),*) => {

        $(  #[allow(non_snake_case)]
            pub fn $arg<S: Into<String>>(s: S) -> Error {
            Error::$arg(s.into()) }
        )*
    }
}

macro_rules! two_param_fn {
    ($($arg:tt),*) => {

        $(  #[allow(non_snake_case)]
            pub fn $arg<S: Into<String>>(n: S, s: S) -> Error {
            Error::$arg(n.into(), s.into())
        } )*
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)] #[rustfmt::skip]
pub enum Error
{
    // From Errors:
    #[error("Error Parsing JSON: `{source}`")]
    FromJsonError { #[from] source: serde_json::Error },
    #[error("Error Opening File: `{source}`")]
    FromIOError { #[from] source: std::io::Error },

    // Zero Parameter Errors:
    #[error("Error Parsing: empty hex string")]
    HexEmptyError(),

    // Single Parameter Errors:
    #[error("Compression Error: `{0}`")]
    CompressionError(String),
    #[error("Decompression Error: `{0}`")]
    DecompressionError(String),
    #[error("Error Parsing: failed to find JSON entry `{0}`")]
    JsonError(String),
    #[error("Error Parsing: invalid hex string `{0}`")]
    HexError(String),
    #[error("Error Parsing: invalid hex string range `{0}`")]
    HexRangeError(String),
    #[error("Error Parsing: `{0}`")]
    ParseError(String),

    // Two Parameter Errors:
    #[error("Error Parsing: number `0x{0}` too small to fit in target type for hex string `{1}`")]
    HexNegOverflowError(String, String),
    #[error("Error Parsing: number `0x{0}` too large to fit in target type for hex string `{1}`")]
    HexPosOverflowError(String, String),
    #[error("Error Parsing: number `0x{0}` number would be zero for non-zero type for hex string `{1}`")]
    HexZeroError(String, String),
}
nil_param_fn!(HexEmptyError);
one_param_fn!(CompressionError, DecompressionError, JsonError, HexError, HexRangeError, ParseError);
two_param_fn!(HexNegOverflowError, HexPosOverflowError, HexZeroError);
