use derive_more::From;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Temporary error type, to be replaced later
    Temporary(String),

    // -- External error types below
    #[from]
    Io(std::io::Error),
}

impl Error {
    // For converting any error into Error::Temporary
    // Usage ->
    //     .map_err(|err| Error::temp_from_err(err))?
    pub fn temp_from_err(err: impl std::error::Error) -> Self {
        Self::Temporary(err.to_string())
    }

    // For explicitly constructing our temporary error
    // Usage ->
    //     return Err(Error::temp("Folder is empty!"));
    pub fn temp(val: impl Into<String>) -> Self {
        Self::Temporary(val.into())
    }
}

// For promoting a simple &str into our temporary error for
// ergonomic error handling!
// Usage ->
//     return Err("Folder is Empty!")?;
//     return Err("Folder is Empty!".into());
//
//     NOTE: ? or .into() are required here!
impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Temporary(val.to_string())
    }
}

// region: Error Boilerplate

// Promotes our custom error enum to core::error::Error error
impl core::error::Error for Error {}

// Every error implementing core::error::Error must also
// implement core::fmt::Display
impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
// endregion
