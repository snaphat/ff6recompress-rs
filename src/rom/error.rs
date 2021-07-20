use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParseError;

impl Error for ParseError {}

impl fmt::Display for ParseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Error Parsing Range.")
    }
}
