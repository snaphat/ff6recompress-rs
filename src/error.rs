use thiserror::Error;

pub type FnError<S> = fn(S) -> Error;

#[macro_export]
macro_rules! JsonError {
    ($($arg:tt)*) => {
        crate::error::JsonError(format!($($arg)*))
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

#[derive(Error, Debug)]
pub enum Error
{
    // Aplib Errors:
    #[error("Aplib {source}")] #[rustfmt::skip]
    AplibApultraError { #[from] source: apultra::Error },
    #[error("Aplib Decompression Error: Header too short (<2)")]
    AplibDecompressShortHeaderError(),
    #[error("Aplib Decompression Error: Invalid header")]
    AplibDecompressInvalidheaderError(),

    // LZSS Errors:
    #[error("LZSS Decompression Error: Invalid compression length of 0")]
    LZSSDecompressZeroError(),
    #[error("LZSS Decompression Error: Input data too short (<2)")]
    LZSSDecompressInputError(),
    #[error("LZSS Decompression Error: Iterated past end of input buffer (>{0})")]
    LZSSDecompressOOBError(usize),
    #[error("LZSS Decompression Error: Buffer length is less than decoded data size ({0}<{1})")]
    LZSSDecompressSizeError(usize, usize),

    // From Errors:
    #[error("Error Parsing JSON: `{source}`")]  #[rustfmt::skip]
    FromJsonError { #[from] source: serde_json::Error },
    #[error("Error Opening File: `{source}`")]  #[rustfmt::skip]
    FromIOError { #[from] source: std::io::Error },

    // CheckedGet Wrappers::
    #[error("Extract Pointer Error: `{0}`")]
    ExtractPtrError(get_checked::Error),
    #[error("Splice Pointer Error: `{0}`")]
    SplicePtrError(get_checked::Error),

    // Zero Parameter Errors:
    #[error("Error Parsing: empty hex string")]
    HexEmptyError(),

    // Single Parameter Errors:
    #[error("Error Parsing: failed to find JSON entry `{0}`")]
    JsonError(String),
    #[error("Error Parsing: invalid hex string `{0}`")]
    HexError(String),
    #[error("Error Parsing: invalid hex string range `{0}`")]
    HexRangeError(String),

    // Two Parameter Errors:
    #[error("Error Parsing: number `0x{0}` too small to fit in target type for hex string `{1}`")]
    HexNegOverflowError(String, String),
    #[error("Error Parsing: number `0x{0}` too large to fit in target type for hex string `{1}`")]
    HexPosOverflowError(String, String),
    #[error(
        "Error Parsing: number `0x{0}` number would be zero for non-zero type for hex string `{1}`"
    )]
    HexZeroError(String, String),
}
nil_param_fn!(HexEmptyError);
one_param_fn!(JsonError, HexError, HexRangeError);
two_param_fn!(HexNegOverflowError, HexPosOverflowError, HexZeroError);
