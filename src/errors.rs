use thiserror::Error;

#[cfg(target_os = "macos")]
#[derive(Error, Debug)]
pub enum MIDError {
    #[error("Command 'system_profiler' not found")]
    CommandError(#[from] std::io::Error),

    #[error("Error converting output to UTF-8")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Result length does not match target length")]
    ResultLengthMismatchError,
}

#[cfg(target_os = "linux")]
#[derive(Error, Debug)]
pub enum MIDError {
    #[error("Command 'cat /etc/machine-id' not found")]
    CommandError(#[from] std::io::Error),

    #[error("Error converting output to UTF-8")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Machine ID length does not match the required length")]
    InvalidMachineIDLengthError,
}

#[cfg(target_os = "windows")]
#[derive(Error, Debug)]
pub enum MIDError {
    #[error("Command 'wmic csproduct get UUID' not found")]
    CommandError(#[from] std::io::Error),

    #[error("Error converting output to UTF-8")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Machine ID length does not match the required length")]
    InvalidMachineIDLengthError,
}
