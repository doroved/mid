use crate::MIDError;
use std::{ffi::OsStr, process::Command};

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!($($arg)*);
        }
    };
}

pub(crate) fn run_shell_comand<S, I>(shell: S, args: I) -> Result<String, MIDError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(shell)
        .args(args)
        .output()
        .map_err(MIDError::ExecuteProcessError)
        .and_then(|output| String::from_utf8(output.stdout).map_err(MIDError::ParseError))
}
