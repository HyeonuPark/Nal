use parse::ParseError;
use check::Error as CheckError;

/// TODO: implement this
#[derive(Debug)]
pub struct Report;

impl Report {
    pub fn to_string(&self) -> String {
        format!("!!!PARSE ERROR!!!")
    }
}

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
