use std::fmt;
use std::process::Command;

pub enum CommandError {
    ExecError(std::io::Error),
    OutputError(std::string::FromUtf8Error),
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

    let command_output = match command.output() {
        Ok(output) => output,
        Err(e) => return Err(CommandError::ExecError(e)),
    };

    let command_output = match String::from_utf8(command_output.stdout) {
        Ok(output) => output,
        Err(e) => return Err(CommandError::OutputError(e)),
    };

    return Ok(command_output);
}
