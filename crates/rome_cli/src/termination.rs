use std::{
    env::current_exe,
    ffi::OsString,
    fmt::{self, Debug, Formatter},
};
use thiserror::Error;

/// Error message returned by the CLI when it aborts with an error
#[derive(Error)]
pub enum Termination {
    /// Returned when it is called with a subcommand it doesn't know
    #[error("unknown command '{command}'")]
    UnknownCommand { command: String },

    /// Return by the help command when it is called with a subcommand it doesn't know
    #[error("cannot print help for unknown command '{command}'")]
    UnknownCommandHelp { command: String },

    /// Returned when the value of a command line argument could not be parsed
    #[error("failed to parse argument '{argument}': {source}")]
    ParseError {
        argument: &'static str,
        #[source]
        source: pico_args::Error,
    },

    /// Returned when the CLI  doesn't recognize a command line argument
    #[error(
        "unrecognized option {argument:?}. Type '{} format --help' for more information.",
        command_name()
    )]
    UnexpectedArgument { argument: OsString },

    /// Returned when a required argument is not present in the command line
    #[error(
        "missing argument '{argument}'. Type '{} format --help' for more information.",
        command_name()
    )]
    MissingArgument { argument: &'static str },

    /// Returned when a subcommand is called without any arguments
    #[error("empty arguments")]
    EmptyArguments,

    /// Returned when a subcommand is called with an unsupported combination of arguments
    #[error("incompatible arguments '{0}' and '{1}'")]
    IncompatibleArguments(&'static str, &'static str),

    /// Returned by the formatter when error diagnostics were emitted in CI mode
    #[error("errors where emitted while formatting")]
    FormattingError,
}

fn command_name() -> String {
    current_exe()
        .ok()
        .and_then(|path| Some(path.file_name()?.to_str()?.to_string()))
        .unwrap_or_else(|| String::from("rome"))
}

// Termination implements Debug by redirecting to Display instead of deriving
// a "canonical" debug implementation as it it is returned as a Result in the
// main function and gets printed by the standard library, which uses Debug but
// we want to show the actuall error message to the user in case of an error
impl Debug for Termination {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self)
    }
}
