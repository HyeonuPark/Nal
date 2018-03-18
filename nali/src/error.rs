
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error(pub String);

impl<T: Into<String>> From<T> for Error {
    fn from(v: T) -> Self {
        Error(v.into())
    }
}
