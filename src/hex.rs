use std::{
    num::{IntErrorKind, ParseIntError},
    ops::Range,
};

use crate::{
    error::{
        FnError, HexEmptyError, HexError, HexNegOverflowError, HexPosOverflowError, HexRangeError,
        HexZeroError,
    },
    result::Result,
};

type ParseIntErrorResult<T> = std::result::Result<T, ParseIntError>;
pub trait Num = num_traits::Num<FromStrRadixErr = ParseIntError>;

pub trait ParseIntErrorMapper<T: Num>
{
    fn map_parse_err<S: Into<String>>(self, num: S, input: S, default: FnError<S>) -> Result<T>;
}

impl<T: Num> ParseIntErrorMapper<T> for ParseIntErrorResult<T>
{
    fn map_parse_err<S: Into<String>>(self, num: S, input: S, default: FnError<S>) -> Result<T>
    {
        match self
        {
            | Ok(num) => Ok(num),
            | Err(err) => Err(match *err.kind()
            {
                | IntErrorKind::PosOverflow => HexPosOverflowError(num, input),
                | IntErrorKind::NegOverflow => HexNegOverflowError(num, input),
                | IntErrorKind::Zero => HexZeroError(num, input),
                | IntErrorKind::Empty => HexError(input),
                | _ => default(input),
            }),
        }
    }
}

pub trait HexStringTo
{
    fn hex_to<T: Num>(self) -> Result<T>;
    fn hex_to_range<T: Num>(self) -> Result<Range<T>>;
}

impl HexStringTo for &str
{
    fn hex_to<T: Num>(self) -> Result<T>
    {
        // Check that string isn't empty.
        if self.len() == 0
        {
            return Err(HexEmptyError());
        };

        // Check that number begins with 0x.
        if self.starts_with("0x") == false
        {
            return Err(HexError(self));
        }

        // Remove 0x.
        let num = self.trim_start_matches("0x");

        // Convert to usize.
        let num = T::from_str_radix(num, 16).map_parse_err(num, self, HexError)?;
        Ok(num)
    }

    fn hex_to_range<T: Num>(self) -> Result<Range<T>>
    {
        // Check that string isn't empty.
        if self.len() == 0
        {
            return Err(HexEmptyError());
        };

        let range = self.split("-").collect::<Vec<&str>>();

        // Get range entries.
        let range0 = unsafe { range.get_unchecked(0) };
        let range1 = unsafe { range.get_unchecked(1) };

        // Check that range consist of only two entries prefixed with 0x + >2 in length.
        #[rustfmt::skip]
        if range.len() != 2
            || !range0.starts_with("0x") || !range1.starts_with("0x")
            ||  range0.len() < 3         ||  range1.len() < 3
        {
            return Err(HexRangeError(self));
        };

        // Remove 0x.
        let beg = range0.trim_start_matches("0x");
        let end = range1.trim_start_matches("0x");

        // Convert to usize.
        let beg = T::from_str_radix(beg, 16).map_parse_err(beg, self, HexRangeError)?;
        let end = T::from_str_radix(end, 16).map_parse_err(end, self, HexRangeError)?;
        Ok(beg..end)
    }
}

#[cfg(test)] #[rustfmt::skip]
mod tests
{
    use super::HexStringTo;

    macro_rules! hex_to {
        ($type:tt, $arg0:expr, $arg1:expr) => {
            paste::item!{ #[test]
            fn [<to_ $type _ok>]() {
                assert_eq!($arg0.hex_to::<$type>().unwrap(), $arg1);
            } }
        }
    }

    macro_rules! hex_to_err {
        ($name:tt, $type:tt, $arg0:tt, $arg1:tt) => {
            paste::item!{ #[test]
            fn [<to_ $type _ $name _err>]() {
                assert_eq!($arg0.hex_to::<$type>().unwrap_err().to_string(), $arg1);
            } }
        }
    }

    macro_rules! hex_to_range {
        ($type:ty, $arg0:expr, $arg1:expr) => {
            paste::item!{ #[test]
                fn [<range_ $type _ok>]() { assert_eq!($arg0.hex_to_range::<$type>().unwrap(), $arg1);
            } }
        }
    }

    macro_rules! hex_to_range_err {
        ($name:tt, $type:tt, $arg0:tt, $arg1:tt) => {
            paste::item!{ #[test]
            fn [<range_ $type _ $name _err>]() {
                assert_eq!($arg0.hex_to_range::<$type>().unwrap_err().to_string(), $arg1);
            } }
        }
    }

    // Num::hex_to success tests:
    hex_to!(usize, "0x1F331", 0x1F331);
    hex_to!(isize, "0x1f331", 0x1F331);
    hex_to!(u64, "0x1F331", 0x1F331);
    hex_to!(i64, "0x1f331", 0x1F331);
    hex_to!(u32, "0x1F331", 0x1F331);
    hex_to!(i32, "0x1f331", 0x1F331);
    hex_to!(u16, "0xF33", 0xF33);
    hex_to!(i16, "0xF33", 0xF33);
    hex_to!(u8, "0x1F", 0x1F);
    hex_to!(i8, "0x1F", 0x1F);
    // Num::hex_to_range success tests:
    hex_to_range!(usize, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(isize, "0x1f331-0xEEbB1", 0x1F331..0xEEBB1);
    hex_to_range!(u64, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(i64, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(u32, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(i32, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(u16, "0xF33-0xEBB", 0xF33..0xEBB);
    hex_to_range!(i16, "0xF33-0xEBB", 0xF33..0xEBB);
    hex_to_range!(u8, "0x1F-0x8E", 0x1F..0x8E);
    hex_to_range!(i8, "0x1F-0x7E", 0x1F..0x7E);
    // Num::hex_to_range error tests:
    hex_to_err!(empty1, i8, "", "Error Parsing: empty hex string");
    hex_to_err!(empty2, i8, "0x", "Error Parsing: invalid hex string `0x`");
    hex_to_err!(invalid1, u64, "s", "Error Parsing: invalid hex string `s`" );
    hex_to_err!(invalid2, u64, "sss", "Error Parsing: invalid hex string `sss`" );
    hex_to_err!(invalid3, u64, "FFFF", "Error Parsing: invalid hex string `FFFF`");
    hex_to_err!(large, i8, "0xFF", "Error Parsing: number `0xFF` too large \
        to fit in target type for hex string `0xFF`" );
    // Num::hex_to_range error tests:
    hex_to_range_err!(empty1, i8, "", "Error Parsing: empty hex string");
    hex_to_range_err!(empty2, i8, "0x", "Error Parsing: invalid hex string range `0x`");
    hex_to_range_err!(empty3, i8, "-", "Error Parsing: invalid hex string range `-`");
    hex_to_range_err!(empty4, u64, "0x-", "Error Parsing: invalid hex string range `0x-`" );
    hex_to_range_err!(empty5, u64, "-0x", "Error Parsing: invalid hex string range `-0x`" );
    hex_to_range_err!(invalid1, u64, "q", "Error Parsing: invalid hex string range `q`" );
    hex_to_range_err!(invalid2, u64, "0xFF", "Error Parsing: invalid hex string range `0xFF`" );
    hex_to_range_err!(invalid3, u64, "ssss", "Error Parsing: invalid hex string range `ssss`" );
    hex_to_range_err!(invalid4, u64, "ssss-", "Error Parsing: invalid hex string range `ssss-`" );
    hex_to_range_err!(invalid5, u64, "-ssss", "Error Parsing: invalid hex string range `-ssss`" );
    hex_to_range_err!(invalid6, u64, "0xssss", "Error Parsing: invalid hex string range `0xssss`" );
    hex_to_range_err!(invalid7, u64, "0xssss-", "Error Parsing: invalid hex string range `0xssss-`" );
    hex_to_range_err!(invalid8, u64, "-0xssss", "Error Parsing: invalid hex string range `-0xssss`" );
    hex_to_range_err!(invalid9, u64, "FFFF-0xFFFF", "Error Parsing: invalid hex string \
    range `FFFF-0xFFFF`" );
    hex_to_range_err!(invalid10, u64, "0xFFFF-FFFF", "Error Parsing: invalid hex string \
    range `0xFFFF-FFFF`" );

    hex_to_range_err!(invalid11, u64, "0xssss-0xFFFFFF", "Error Parsing: invalid hex string \
    range `0xssss-0xFFFFFF`" );
    hex_to_range_err!(invalid12, u64, "0xFFFFFF-0xssss", "Error Parsing: invalid hex string \
        range `0xFFFFFF-0xssss`" );
    hex_to_range_err!(invalid13, u64, "0xssss-0xqqqq", "Error Parsing: invalid hex string range \
        `0xssss-0xqqqq`" );
    hex_to_range_err!(large1, i8, "0x0F-0xFF", "Error Parsing: number `0xFF` too large to \
        fit in target type for hex string `0x0F-0xFF`");
    hex_to_range_err!(large2, i8, "0xFF-0x0F", "Error Parsing: number `0xFF` too large to \
        fit in target type for hex string `0xFF-0x0F`");
}
