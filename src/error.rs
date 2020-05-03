pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    inner: Box<ErrorImpl>,
}

pub struct ErrorImpl {}
