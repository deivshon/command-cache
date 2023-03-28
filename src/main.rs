use std::process::{exit, Command};
use std::time::SystemTime;

const TIME_LIMIT: usize = 1;
const COMMAND: usize = 2;
const ARGS_START: usize = 3;

fn failure(msg: &str) -> ! {
    eprintln!("command-cache: {}", msg);
    exit(1)
}

fn execute_command(command: &String, args: &[String]) -> String {
    let mut command = Command::new(command);

    for command_arg in args {
        command.arg(command_arg);
    }

    let Ok(command_output) = command.output() else {
        failure("Could not get command output");
    };

    let Ok(command_output) = String::from_utf8(command_output.stdout) else {
        failure("Could not convert command output to UTF-8 encoded text");
    };

    return command_output;
}

fn main() {
    let now = SystemTime::UNIX_EPOCH.elapsed()
        .expect("Could not retrieve UNIX timestamp")
        .as_millis();
    
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        failure("Not enough arguments");
    }

    let Ok(period) = args[TIME_LIMIT].parse::<u128>() else {
        failure("Could not parse time limit");
    };
    
    let output = execute_command(&args[COMMAND], &args[ARGS_START..]);

    println!("{}", output);
}
