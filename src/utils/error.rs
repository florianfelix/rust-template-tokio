pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Variant,
    SerdeJson(serde_json::Error),
    TokioJoin(tokio::task::JoinError),
}

// region:    --- Froms
impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::SerdeJson(val)
    }
}

impl From<tokio::task::JoinError> for Error {
   fn from(val: tokio::task::JoinError) -> Self {
       Self::TokioJoin(val)
   }
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
