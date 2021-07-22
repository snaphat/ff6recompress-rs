use std::{error::Error, ops::Range};

use super::error::ParseError;
pub trait HexStringTo
{
    fn hex_to<T>(&self) -> Result<T, Box<dyn Error>>
    where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>;
    fn hex_to_range<T>(&self) -> Result<Range<T>, Box<dyn Error>>
    where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>;
}

impl HexStringTo for &str
{
    fn hex_to<T>(&self) -> Result<T, Box<dyn Error>>
    where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>
    {
        // Slice 0x from number.
        let num = self.get(2..).ok_or(ParseError::new("invalid hex string"))?;

        // Convert to usize.
        let num = T::from_str_radix(num, 16)?;
        Ok(num)
    }

    fn hex_to_range<T>(&self) -> Result<Range<T>, Box<dyn Error>>
    where T: num_traits::Num<FromStrRadixErr = std::num::ParseIntError>
    {
        let range = self.split("-").collect::<Vec<&str>>();

        // Check that range consist of only two entries..
        if range.len() != 2
        {
            return Err(Box::new(ParseError::new("invalid hex range")));
        };
        // Slice 0x from start.
        let beg = &range[0].get(2..).ok_or(ParseError::new("invalid hex range"))?;
        let end = &range[1].get(2..).ok_or(ParseError::new("invalid hex range"))?;

        // Convert to usize.
        let beg = T::from_str_radix(beg, 16)?;
        let end = T::from_str_radix(end, 16)?;
        Ok(beg..end)
    }
}

#[test]
fn hex_to_test_usize()
{
    assert_eq!(0x1F331, "0x1F331".hex_to::<usize>().unwrap());
}

#[test]
fn hex_to_test_isize()
{
    assert_eq!(0x1F331, "0x1F331".hex_to::<isize>().unwrap());
}

#[test]
fn hex_to_test_u64()
{
    assert_eq!(0x1F331, "0x1F331".hex_to::<u64>().unwrap());
}

#[test]
fn hex_to_test_i64()
{
    assert_eq!(0x1F331, "0x1F331".hex_to::<i64>().unwrap());
}

#[test]
fn hex_to_test_u32()
{
    assert_eq!(0x1F331, "0x1F331".hex_to::<u32>().unwrap());
}

#[test]
fn hex_to_test_i32()
{
    assert_eq!(0x1F331, "0x1F331".hex_to::<i32>().unwrap());
}

#[test]
fn hex_to_test_u16()
{
    assert_eq!(0x1F33, "0x1F33".hex_to::<u16>().unwrap());
}

#[test]
fn hex_to_test_i16()
{
    assert_eq!(0x1F33, "0x1F33".hex_to::<i16>().unwrap());
}

#[test]
fn hex_to_test_u8()
{
    assert_eq!(0x1F, "0x1F".hex_to::<u8>().unwrap());
}

#[test]
fn hex_to_test_i8()
{
    assert_eq!(0x1F, "0x1F".hex_to::<i8>().unwrap());
}

#[test]
fn hex_to_range_test_usize()
{
    assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<usize>().unwrap());
}

#[test]
fn hex_to_range_test_isize()
{
    assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<isize>().unwrap());
}

#[test]
fn hex_to_range_test_u64()
{
    assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<u64>().unwrap());
}

#[test]
fn hex_to_range_test_i64()
{
    assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<i64>().unwrap());
}

#[test]
fn hex_to_range_test_u32()
{
    assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<u32>().unwrap());
}

#[test]
fn hex_to_range_test_i32()
{
    assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<i32>().unwrap());
}

#[test]
fn hex_to_range_test_u16()
{
    assert_eq!(0x1F33..0xEEBB, "0x1F33-0xEEBB".hex_to_range::<u16>().unwrap());
}

#[test]
fn hex_to_range_test_i16()
{
    assert_eq!(0x1F33..0x4EBB, "0x1F33-0x4EBB".hex_to_range::<i16>().unwrap());
}

#[test]
fn hex_to_range_test_u8()
{
    assert_eq!(0x1F..0x8E, "0x1F-0x8E".hex_to_range::<u8>().unwrap());
}

#[test]
fn hex_to_range_test_i8()
{
    assert_eq!(0x1F..0x7E, "0x1F-0x7E".hex_to_range::<i8>().unwrap());
}

#[test]
fn hex_to_test_error_overflow()
{
    let err = "0xFF".hex_to::<i8>().unwrap_err();
    assert_eq!("number too large to fit in target type", format!("{}", err));
}

#[test]
fn hex_to_test_error_invalid()
{
    let err = "sdsfds".hex_to::<u64>().unwrap_err();
    assert_eq!("invalid digit found in string", format!("{}", err));
}

#[test]
fn hex_to_test_error_invalid_hex()
{
    let err = "s".hex_to::<u64>().unwrap_err();
    assert_eq!("Error Parsing: invalid hex string", format!("{}", err));
}

#[test]
fn hex_to_range_test_error_overflow()
{
    let err = "0x0F-0xFF".hex_to_range::<i8>().unwrap_err();
    assert_eq!("number too large to fit in target type", format!("{}", err));
    let err = "0xFF-0x0F".hex_to_range::<i8>().unwrap_err();
    assert_eq!("number too large to fit in target type", format!("{}", err));
}

#[test]
fn hex_to_range_test_error_invalid()
{
    let err = "sdsfds-0xFFFFFF".hex_to_range::<u64>().unwrap_err();
    assert_eq!("invalid digit found in string", format!("{}", err));
    let err = "0xFFFFFF-sdsfds".hex_to_range::<u64>().unwrap_err();
    assert_eq!("invalid digit found in string", format!("{}", err));
}

#[test]
fn hex_to_range_test_error_invalid_hex()
{
    let err = "sss".hex_to_range::<u64>().unwrap_err();
    assert_eq!("Error Parsing: invalid hex range", format!("{}", err));
}

#[test]
fn hex_to_range_test_error_invalid_hex2()
{
    let err = "-".hex_to_range::<u64>().unwrap_err();
    assert_eq!("Error Parsing: invalid hex range", format!("{}", err));
}

#[test]
fn hex_to_range_test_error_invalid_hex3()
{
    let err = "0xFFF-a".hex_to_range::<u64>().unwrap_err();
    assert_eq!("Error Parsing: invalid hex range", format!("{}", err));
}
