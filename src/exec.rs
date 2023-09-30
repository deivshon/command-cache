use std::process::Command;
use std::string::FromUtf8Error;
use std::{convert, fmt, io};

pub enum CommandError {
    ExecError(io::Error),
    OutputError(FromUtf8Error),
}

impl convert::From<io::Error> for CommandError {
    fn from(err: io::Error) -> CommandError {
        return CommandError::ExecError(err);
    }
}

impl convert::From<FromUtf8Error> for CommandError {
    fn from(err: FromUtf8Error) -> CommandError {
        return CommandError::OutputError(err);
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::ExecError(e) => write!(f, "Error executing command: {}", e),
            CommandError::OutputError(e) => write!(f, "Error parsing command output: {}", e),
        }
    }
}

pub fn execute_command(command: &String, args: &[String]) -> Result<String, CommandError> {
    let mut command = Command::new(command);
    for command_arg in args {
        command.arg(command_arg);
    }

    return Ok(String::from_utf8(command.output()?.stdout)?);
}
