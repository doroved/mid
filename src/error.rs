pub type Result<T> = core::result::Result<T, MIDError>;

#[derive(Debug)]
pub enum MIDError {
    // MID.DEV -> MID.get[error]: ExecuteProcessError(Os { code: 2, kind: NotFound, message: "No such file or directory" })
    ExecuteProcessError(std::io::Error),
    ParseError(std::string::FromUtf8Error),
    EmptyMidResult,
    EmptyMidKey,
}

impl core::fmt::Display for MIDError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for MIDError {}
