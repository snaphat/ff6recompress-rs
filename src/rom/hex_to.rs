use std::ops::Range;

use super::error::{FF6Error, *};

pub trait ParseIntErrorMapper<T>
where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>
{
    fn map_parse_err<S: Into<String>>(self, num: S, input: S) -> Result<T, FF6Error>;
}

impl<T> ParseIntErrorMapper<T> for Result<T, std::num::ParseIntError>
where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>
{
    fn map_parse_err<S: Into<String>>(self, num: S, input: S) -> Result<T, FF6Error>
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
                | _ => HexError(input),
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
        let num = T::from_str_radix(num, 16).map_parse_err(num, self)?;
        Ok(num)
    }

    fn hex_to_range<T>(self) -> Result<Range<T>, FF6Error>
    where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>
    {
        let range = self.split("-").collect::<Vec<&str>>();

        // Check that range consist of only two entries..
        if range.len() != 2
        {
            return Err(HexError(self));
        };

        // Check if begins with 0x. If so strip it.
        let beg = range[0].trim_start_matches("0x");
        let end = range[1].trim_start_matches("0x");

        // Check that range isn't empty.
        if beg.len() == 0 || end.len() == 0
        {
            return Err(HexError(self));
        };

        // Convert to usize.
        let beg = T::from_str_radix(beg, 16).map_parse_err(beg, self)?;
        let end = T::from_str_radix(end, 16).map_parse_err(end, self)?;
        Ok(beg..end)
    }
}

#[cfg(test)]
mod test
{
    use super::HexStringTo;

    #[test] #[rustfmt::skip] fn usize() { assert_eq!("0x1F331".hex_to::<usize>().unwrap(), 0x1F331); }
    #[test] #[rustfmt::skip] fn isize() { assert_eq!("0x1F331".hex_to::<isize>().unwrap(), 0x1F331); }
    #[test] #[rustfmt::skip] fn u64() { assert_eq!("0x1F331".hex_to::<u64>().unwrap(), 0x1F331); }
    #[test] #[rustfmt::skip] fn i64() { assert_eq!("0x1F331".hex_to::<i64>().unwrap(), 0x1F331); }
    #[test] #[rustfmt::skip] fn u32() { assert_eq!("0x1F331".hex_to::<u32>().unwrap(), 0x1F331); }
    #[test] #[rustfmt::skip] fn i32() { assert_eq!("0x1F331".hex_to::<i32>().unwrap(), 0x1F331); }
    #[test] #[rustfmt::skip] fn u16() { assert_eq!("0x1F33".hex_to::<u16>().unwrap(), 0x1F33); }
    #[test] #[rustfmt::skip] fn i16() { assert_eq!("0x1F33".hex_to::<i16>().unwrap(), 0x1F33); }
    #[test] #[rustfmt::skip] fn u8() { assert_eq!("0x1F".hex_to::<u8>().unwrap(), 0x1F); }
    #[test] #[rustfmt::skip] fn i8() { assert_eq!("0x1F".hex_to::<i8>().unwrap(), 0x1F); }
    #[test] #[rustfmt::skip] fn range_usize() { assert_eq!("0x1F331-0xEEBB1".hex_to_range::<usize>().unwrap(), 0x1F331..0xEEBB1); }
    #[test] #[rustfmt::skip] fn range_isize() { assert_eq!("0x1F331-0xEEBB1".hex_to_range::<isize>().unwrap(), 0x1F331..0xEEBB1); }
    #[test] #[rustfmt::skip] fn range_u64() { assert_eq!("0x1F331-0xEEBB1".hex_to_range::<u64>().unwrap(), 0x1F331..0xEEBB1); }
    #[test] #[rustfmt::skip] fn range_i64() { assert_eq!("0x1F331-0xEEBB1".hex_to_range::<i64>().unwrap(), 0x1F331..0xEEBB1); }
    #[test] #[rustfmt::skip] fn range_u32() { assert_eq!("0x1F331-0xEEBB1".hex_to_range::<u32>().unwrap(), 0x1F331..0xEEBB1); }
    #[test] #[rustfmt::skip] fn range_i32() { assert_eq!("0x1F331-0xEEBB1".hex_to_range::<i32>().unwrap(), 0x1F331..0xEEBB1); }
    #[test] #[rustfmt::skip] fn range_u16() { assert_eq!("0x1F33-0xEEBB".hex_to_range::<u16>().unwrap(), 0x1F33..0xEEBB); }
    #[test] #[rustfmt::skip] fn range_i16() { assert_eq!("0x1F33-0x4EBB".hex_to_range::<i16>().unwrap(), 0x1F33..0x4EBB); }
    #[test] #[rustfmt::skip] fn range_u8() { assert_eq!("0x1F-0x8E".hex_to_range::<u8>().unwrap(), 0x1F..0x8E); }
    #[test] #[rustfmt::skip] fn range_i8() { assert_eq!("0x1F-0x7E".hex_to_range::<i8>().unwrap(), 0x1F..0x7E); }
    #[test] #[rustfmt::skip] fn err_hex1() { assert_eq!("".hex_to::<i8>().unwrap_err().to_string(), "Error Parsing: invalid hex string ``"); }
    #[test] #[rustfmt::skip] fn err_hex2() { assert_eq!("0x".hex_to::<i8>().unwrap_err().to_string(), "Error Parsing: invalid hex string `0x`"); }
    #[test] #[rustfmt::skip] fn err_digit1() { assert_eq!("sss".hex_to::<u64>().unwrap_err().to_string(), "Error Parsing: invalid hex string `sss`"); }
    #[test] #[rustfmt::skip] fn err_digit2() { assert_eq!("s".hex_to::<u64>().unwrap_err().to_string(), "Error Parsing: invalid hex string `s`"); }
    #[test] #[rustfmt::skip] fn range_err_hex1() { assert_eq!("sss".hex_to_range::<u64>().unwrap_err().to_string(), "Error Parsing: invalid hex string `sss`"); }
    #[test] #[rustfmt::skip] fn range_err_hex2() { assert_eq!("-".hex_to_range::<u64>().unwrap_err().to_string(), "Error Parsing: invalid hex string `-`"); }
    #[test] #[rustfmt::skip] fn err_large() { assert_eq!("0xFF".hex_to::<i8>().unwrap_err().to_string(), "Error Parsing: number `0xFF` too large to fit in target type for hex string `0xFF`"); }
    #[test] #[rustfmt::skip] fn hex_to_range_error_overflow() {
        assert_eq!("0x0F-0xFF".hex_to_range::<i8>().unwrap_err().to_string(), "Error Parsing: number `0xFF` too large to fit in target type for hex string `0x0F-0xFF`");
        assert_eq!("0xFF-0x0F".hex_to_range::<i8>().unwrap_err().to_string(), "Error Parsing: number `0xFF` too large to fit in target type for hex string `0xFF-0x0F`");
    }
    #[test] #[rustfmt::skip] fn hex_to_range_error_invalid()
    {
        assert_eq!("sss-0xFFFFFF".hex_to_range::<u64>().unwrap_err().to_string(), "Error Parsing: invalid hex string `sss-0xFFFFFF`");
        assert_eq!("0xFFFFFF-sss".hex_to_range::<u64>().unwrap_err().to_string(), "Error Parsing: invalid hex string `0xFFFFFF-sss`");
    }
}
