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
        let num = self.get(2..).ok_or(ParseError::new(""))?; // FIXME: Parser error.

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
            return Err(Box::new(ParseError::new(""))); // FIXME: PARSE_ERROR
        };
        // Slice 0x from start.
        let beg = &range[0].get(2..).ok_or(ParseError::new(""))?; // FIXME: PARSE ERROR
        let end = &range[1].get(2..).ok_or(ParseError::new(""))?; // FIXME: PARSER ERROR

        // Convert to usize.
        let beg = T::from_str_radix(beg, 16)?;
        let end = T::from_str_radix(end, 16)?;
        Ok(beg..end)
    }
}

#[test]
fn hex_to_test()
{
    assert_eq!(0x0, "0x0".hex_to::<usize>().unwrap());
    assert_eq!(0x1F331, "0x1F331".hex_to::<usize>().unwrap());
    assert_eq!(0x0, "0x0".hex_to::<isize>().unwrap());
    assert_eq!(0x1F331, "0x1F331".hex_to::<isize>().unwrap());
    assert_eq!(0x0, "0x0".hex_to::<u64>().unwrap());
    assert_eq!(0x1F331, "0x1F331".hex_to::<u64>().unwrap());
    assert_eq!(0x0, "0x0".hex_to::<u16>().unwrap());
    assert_eq!(0x1F1, "0x1F1".hex_to::<u16>().unwrap());
    assert_eq!(0x0, "0x0".hex_to::<i8>().unwrap());
    assert_eq!(0x1F, "0x1F".hex_to::<i8>().unwrap());
}

#[test]
fn hex_to_range_test()
{
    assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<usize>().unwrap());
    assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<isize>().unwrap());
    assert_eq!(0x1F331..0xEEBB1, "0x1F331-0xEEBB1".hex_to_range::<u64>().unwrap());
    assert_eq!(0x1F33..0xEEBB, "0x1F33-0xEEBB".hex_to_range::<u16>().unwrap());
    assert_eq!(0x1F..0x7E, "0x1F-0x7E".hex_to_range::<i8>().unwrap());
}
