use std::num::ParseIntError;
extern crate thiserror;
use self::thiserror::Error;

#[macro_export]
macro_rules! parse_error {
    ($($arg:tt)*) => {
        super::error::ParseError(format!($($arg)*))
    }
}

macro_rules! error_func {
    ($arg:tt) => {
        #[allow(non_snake_case)]
        pub fn $arg<S: Into<String>>(s: S) -> FF6Error
        {
            return FF6Error::$arg(s.into());
        }
    };
}

macro_rules! error_func_wrap {
    ($arg0:tt, $arg1:tt) => {
        #[allow(non_snake_case)]
        pub fn $arg0<S: Into<String>>(e: $arg1, s: S) -> FF6Error
        {
            return FF6Error::$arg0(e, s.into());
        }
    };
}

#[derive(Error, Debug)]
pub enum FF6Error
{
    // Normal Errors:
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
}

error_func!(ParseError);
error_func!(HexError);
error_func!(HexRangeError);
error_func_wrap!(HexWrapError, ParseIntError);
error_func_wrap!(HexRangeWrapError, ParseIntError);
