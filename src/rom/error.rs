use std::{error::Error, fmt};

#[macro_export]
macro_rules! parse_error {
    ($($arg:tt)*) => {
        ParseError::new(format!($($arg)*))
    }
}

#[derive(Debug)]
pub struct ParseError
{
    pub message: String,
}

impl Error for ParseError {}

impl ParseError
{
    pub fn new<S: AsRef<str>>(message: S) -> ParseError
    {
        ParseError { message: message.as_ref().to_string() }
    }
}

impl fmt::Display for ParseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Error Parsing: {}", self.message)
    }
}
