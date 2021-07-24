use std::ops::Range;

use super::error::{FF6Error, *};

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
        let num = T::from_str_radix(num, 16).map_err(|e| HexWrapError(e, num))?;
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
        let beg = T::from_str_radix(beg, 16).map_err(|e| HexRangeWrapError(e, beg))?;
        let end = T::from_str_radix(end, 16).map_err(|e| HexRangeWrapError(e, end))?;
        Ok(beg..end)
    }
}

#[cfg(test)]
mod test
{
    use super::HexStringTo;

    #[test]
    fn test_hex_to_usize()
    {
        assert_eq!(0x1F331, "0x1F331".hex_to::<usize>().unwrap());
    }

    #[test]
    fn test_hex_to_isize()
    {
        assert_eq!(0x1F331, "0x1F331".hex_to::<isize>().unwrap());
    }

    #[test]
    fn test_hex_to_u64()
    {
        assert_eq!(0x1F331, "0x1F331".hex_to::<u64>().unwrap());
    }

    #[test]
    fn test_hex_to_i64()
    {
        assert_eq!(0x1F331, "0x1F331".hex_to::<i64>().unwrap());
    }

    #[test]
    fn test_hex_to_u32()
    {
        assert_eq!(0x1F331, "0x1F331".hex_to::<u32>().unwrap());
    }

    #[test]
    fn test_hex_to_i32()
    {
        assert_eq!(0x1F331, "0x1F331".hex_to::<i32>().unwrap());
    }

    #[test]
    fn test_hex_to_u16()
    {
        assert_eq!(0x1F33, "0x1F33".hex_to::<u16>().unwrap());
    }

    #[test]
    fn test_hex_to_i16()
    {
        assert_eq!(0x1F33, "0x1F33".hex_to::<i16>().unwrap());
    }

    #[test]
    fn test_hex_to_u8()
    {
        assert_eq!(0x1F, "0x1F".hex_to::<u8>().unwrap());
    }

    #[test]
    fn test_hex_to_i8()
    {
        assert_eq!(0x1F, "0x1F".hex_to::<i8>().unwrap());
    }

    #[test]
    fn test_hex_to_range_usize()
    {
        assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<usize>().unwrap());
    }

    #[test]
    fn test_hex_to_range_isize()
    {
        assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<isize>().unwrap());
    }

    #[test]
    fn test_hex_to_range_u64()
    {
        assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<u64>().unwrap());
    }

    #[test]
    fn test_hex_to_range_i64()
    {
        assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<i64>().unwrap());
    }

    #[test]
    fn test_hex_to_range_u32()
    {
        assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<u32>().unwrap());
    }

    #[test]
    fn test_hex_to_range_i32()
    {
        assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<i32>().unwrap());
    }

    #[test]
    fn test_hex_to_range_u16()
    {
        assert_eq!(0x1F33..0xEEBB, "0x1F33-0xEEBB".hex_to_range::<u16>().unwrap());
    }

    #[test]
    fn test_hex_to_range_i16()
    {
        assert_eq!(0x1F33..0x4EBB, "0x1F33-0x4EBB".hex_to_range::<i16>().unwrap());
    }

    #[test]
    fn test_hex_to_range_u8()
    {
        assert_eq!(0x1F..0x8E, "0x1F-0x8E".hex_to_range::<u8>().unwrap());
    }

    #[test]
    fn test_hex_to_range_i8()
    {
        assert_eq!(0x1F..0x7E, "0x1F-0x7E".hex_to_range::<i8>().unwrap());
    }

    #[test]
    fn test_hex_to_error()
    {
        let err = "0x".hex_to::<i8>().unwrap_err();
        assert_eq!("Error Parsing: `invalid hex string '0x'`", format!("{}", err));
    }

    #[test]
    fn test_hex_to_error_overflow()
    {
        let err = "0xFF".hex_to::<i8>().unwrap_err();
        assert_eq!(
            "Error Parsing: `number too large to fit in target type 'FF'`",
            format!("{}", err)
        );
    }

    #[test]
    fn test_hex_to_error_invalid()
    {
        let err = "sdsfds".hex_to::<u64>().unwrap_err();
        assert_eq!("Error Parsing: `invalid digit found in string 'sdsfds'`", format!("{}", err));
    }

    #[test]
    fn test_hex_to_error_invalid_hex()
    {
        let err = "s".hex_to::<u64>().unwrap_err();
        assert_eq!("Error Parsing: `invalid digit found in string 's'`", format!("{}", err));
    }

    #[test]
    fn test_hex_to_range_error_overflow()
    {
        let err = "0x0F-0xFF".hex_to_range::<i8>().unwrap_err();
        assert_eq!(
            "Error Parsing: `number too large to fit in target type 'FF'`",
            format!("{}", err)
        );
        let err = "0xFF-0x0F".hex_to_range::<i8>().unwrap_err();
        assert_eq!(
            "Error Parsing: `number too large to fit in target type 'FF'`",
            format!("{}", err)
        );
    }

    #[test]
    fn test_hex_to_range_error_invalid()
    {
        let err = "sdsfds-0xFFFFFF".hex_to_range::<u64>().unwrap_err();
        assert_eq!("Error Parsing: `invalid digit found in string 'sdsfds'`", format!("{}", err));
        let err = "0xFFFFFF-sdsfds".hex_to_range::<u64>().unwrap_err();
        assert_eq!("Error Parsing: `invalid digit found in string 'sdsfds'`", format!("{}", err));
    }

    #[test]
    fn test_hex_to_range_error_invalid_hex()
    {
        let err = "sss".hex_to_range::<u64>().unwrap_err();
        assert_eq!("Error Parsing: `invalid hex range 'sss'`", format!("{}", err));
    }

    #[test]
    fn test_hex_to_range_error_invalid_hex2()
    {
        let err = "-".hex_to_range::<u64>().unwrap_err();
        assert_eq!("Error Parsing: `invalid hex range '-'`", format!("{}", err));
    }
}
