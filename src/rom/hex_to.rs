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
