use std::num::ParseIntError;
extern crate thiserror;
use self::thiserror::Error;
extern crate paste;

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

        paste::item! {
            #[test]
            #[allow(non_snake_case)]
            fn [<test_ $arg>]() {
                let a = FF6Error::$arg("".into());
                let b = $arg("");
                assert_eq!(a.to_string(), b.to_string());
            }
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

        paste::item! {
            #[test]
            #[allow(non_snake_case)]
            fn [<test_ $arg0>]() {
                // Undefined behavior: Oh well...
                let e = i32::from_str_radix("a12", 10).unwrap_err();
                let a = FF6Error::$arg0(e.clone(), "".into());
                let b = $arg0(e, "");
                assert_eq!(a.to_string(), b.to_string());
            }
        }
    };
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
}

error_func!(CompressionError);
error_func!(DecompressionError);
error_func!(ParseError);
error_func!(HexError);
error_func!(HexRangeError);
error_func_wrap!(HexWrapError, ParseIntError);
error_func_wrap!(HexRangeWrapError, ParseIntError);
