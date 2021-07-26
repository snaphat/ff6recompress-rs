use std::ops::Range;
extern crate paste;

use super::error::{FF6Error, *};

pub trait ParseIntErrorMapper<T>
where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>
{
    fn map_parse_err<S: Into<String>>(self, num: S, input: S, range: bool) -> Result<T, FF6Error>;
}

impl<T> ParseIntErrorMapper<T> for Result<T, std::num::ParseIntError>
where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>
{
    fn map_parse_err<S: Into<String>>(self, num: S, input: S, range: bool) -> Result<T, FF6Error>
    {
        let num = num.into();
        let input = input.into();
        match self
        {
            | Ok(num) => Ok(num),
            | Err(err) => Err(match *err.kind()
            {
                | std::num::IntErrorKind::PosOverflow => HexPosOverflowError(num, input),
                | std::num::IntErrorKind::NegOverflow => HexNegOverflowError(num, input),
                | std::num::IntErrorKind::Zero => HexZeroError(num, input),
                | _ if range == false => HexError(input),
                | _ => HexRangeError(input),
            }),
        }
    }
}

pub trait HexStringTo
{
    fn hex_to<T>(self) -> Result<T, FF6Error>
    where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>;
    fn hex_to_range<T>(self) -> Result<Range<T>, FF6Error>
    where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>;
}

impl HexStringTo for &str
{
    fn hex_to<T>(self) -> Result<T, FF6Error>
    where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>
    {
        // Slice 0x from number.
        let num = self.trim_start_matches("0x");

        // Check that num isn't empty.
        if num.len() == 0
        {
            return Err(HexError(self));
        };

        // Convert to usize.
        let num = T::from_str_radix(num, 16).map_parse_err(num, self, false)?;
        Ok(num)
    }

    fn hex_to_range<T>(self) -> Result<Range<T>, FF6Error>
    where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>
    {
        let range = self.split("-").collect::<Vec<&str>>();

        // Check that range consist of only two entries..
        if range.len() != 2
        {
            return Err(HexRangeError(self));
        };

        // Check if begins with 0x. If so strip it.
        let beg = range[0].trim_start_matches("0x");
        let end = range[1].trim_start_matches("0x");

        // Check that range isn't empty.
        if beg.len() == 0 || end.len() == 0
        {
            return Err(HexRangeError(self));
        };

        // Convert to usize.
        let beg = T::from_str_radix(beg, 16).map_parse_err(beg, self, true)?;
        let end = T::from_str_radix(end, 16).map_parse_err(end, self, true)?;
        Ok(beg..end)
    }
}

#[cfg(test)] #[rustfmt::skip]
mod test
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
    hex_to!(isize, "0x1F331", 0x1F331);
    hex_to!(u64, "0x1F331", 0x1F331);
    hex_to!(i64, "0x1F331", 0x1F331);
    hex_to!(u32, "0x1F331", 0x1F331);
    hex_to!(i32, "0x1F331", 0x1F331);
    hex_to!(u16, "0x1F33", 0x1F33);
    hex_to!(i16, "0x1F33", 0x1F33);
    hex_to!(u8, "0x1F", 0x1F);
    hex_to!(i8, "0x1F", 0x1F);
    // Num::hex_to_range success tests:
    hex_to_range!(usize, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(isize, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(u64, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(i64, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(u32, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(i32, "0x1F331-0xEEBB1", 0x1F331..0xEEBB1);
    hex_to_range!(u16, "0x1F33-0xEEBB", 0x1F33..0xEEBB);
    hex_to_range!(i16, "0x1F33-0x4EBB", 0x1F33..0x4EBB);
    hex_to_range!(u8, "0x1F-0x8E", 0x1F..0x8E);
    hex_to_range!(i8, "0x1F-0x7E", 0x1F..0x7E);
    // Num::hex_to_range error tests:
    hex_to_err!(empty1, i8, "", "Error Parsing: invalid hex string ``");
    hex_to_err!(empty2, i8, "0x", "Error Parsing: invalid hex string `0x`");
    hex_to_err!(invalid1, u64, "s", "Error Parsing: invalid hex string `s`" );
    hex_to_err!(invalid2, u64, "sss", "Error Parsing: invalid hex string `sss`" );
    hex_to_err!(large, i8, "0xFF", "Error Parsing: number `0xFF` too large \
        to fit in target type for hex string `0xFF`" );
    // Num::hex_to_range error tests:
    hex_to_range_err!(empty1, i8, "", "Error Parsing: invalid hex string range ``");
    hex_to_range_err!(empty2, i8, "0x", "Error Parsing: invalid hex string range `0x`");
    hex_to_range_err!(empty3, i8, "-", "Error Parsing: invalid hex string range `-`");
    hex_to_range_err!(invalid1, u64, "0xFF", "Error Parsing: invalid hex string range `0xFF`" );
    hex_to_range_err!(invalid2, u64, "s", "Error Parsing: invalid hex string range `s`" );
    hex_to_range_err!(invalid3, u64, "sss", "Error Parsing: invalid hex string range `sss`" );
    hex_to_range_err!(invalid4, u64, "sss-0xFFFFFF", "Error Parsing: invalid hex string \
        range `sss-0xFFFFFF`" );
    hex_to_range_err!(invalid5, u64, "0xFFFFFF-sss", "Error Parsing: invalid hex string \
        range `0xFFFFFF-sss`" );
    hex_to_range_err!(invalid6, u64, "sss-qqq", "Error Parsing: invalid hex string range \
        `sss-qqq`" );
    hex_to_range_err!(large1, i8, "0x0F-0xFF", "Error Parsing: number `0xFF` too large to \
        fit in target type for hex string `0x0F-0xFF`");
    hex_to_range_err!(large2, i8, "0xFF-0x0F", "Error Parsing: number `0xFF` too large to \
        fit in target type for hex string `0xFF-0x0F`");
}
