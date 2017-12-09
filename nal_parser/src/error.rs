use nom::Err as NomError;
use convert::Error as ConvertError;

// TODO: Implement this!
pub struct Error;

impl From<NomError> for Error {
    fn from(_err: NomError) -> Self {
        Error
    }
}

impl From<Vec<ConvertError>> for Error {
    fn from(_err: Vec<ConvertError>) -> Self {
        Error
    }
}
