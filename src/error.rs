use parser::Error as ParseError;
use atoi::Error as ConvertError;
use nali::Error as RuntimeError;

#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
    Convert(ConvertError),
    Runtime(RuntimeError),
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parse(e)
    }
}

impl From<ConvertError> for Error {
    fn from(e: ConvertError) -> Self {
        Error::Convert(e)
    }
}

impl From<RuntimeError> for Error {
    fn from(e: RuntimeError) -> Self {
        Error::Runtime(e)
    }
}
