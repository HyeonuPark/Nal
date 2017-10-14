use parse::ParseError;
use check::Error as CheckError;

/// TODO: implement this
#[derive(Debug)]
pub struct Report;

impl From<ParseError> for Report {
    fn from(_err: ParseError) -> Self {
        Report
    }
}

impl From<Vec<CheckError>> for Report {
    fn from(_err: Vec<CheckError>) -> Self {
        Report
    }
}
