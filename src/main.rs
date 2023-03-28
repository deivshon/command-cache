pub mod cache;

use crate::cache::Cache;

use std::process::{exit, Command};
use std::time::SystemTime;
use std::fs;
use std::path::Path;
use file_lock::{FileLock, FileOptions};
use std::io::prelude::*;

use md5;

const TIME_LIMIT: usize = 1;
const COMMAND: usize = 2;
const ARGS_START: usize = 3;

const CACHE_DIR: &str = "/tmp/command-cache";

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

fn command_hash(command: &[String]) -> md5::Digest {
    let mut command_representation: Vec<u8> = vec![];

    for i in 0..command.len() {
        command_representation.extend_from_slice(command[i].as_bytes());
        command_representation.push(i as u8)
    }

    return md5::compute(command_representation);
}

#[inline]
fn current_timestamp() -> u128 {
    SystemTime::UNIX_EPOCH.elapsed()
        .expect("Could not retrieve UNIX timestamp")
        .as_millis()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        failure("Not enough arguments");
    }

    let Ok(period) = args[TIME_LIMIT].parse::<u128>() else {
        failure("Could not parse time limit");
    };

    let command_id = format!("{:?}", command_hash(&args[COMMAND..]));
    let cache_path = format!("{}/{}", CACHE_DIR, command_id);

    if !Path::new(CACHE_DIR).is_dir() {
        fs::create_dir_all(CACHE_DIR)
        .expect(&format!("Could not create cache directory {}", CACHE_DIR));
    };

    let output;
    let mut cache: Cache;

    if Path::new(&cache_path).exists() {
        let lock_options = FileOptions::new().write(true).read(true);

        let mut filelock = match FileLock::lock(&cache_path, true, lock_options) {
            Ok(lock) => lock,
            Err(e) =>  failure(&format!("Could not acquire lock on cache file {}: {}", cache_path, e))
        };

        let mut cache_bytes: Vec<u8> = vec![];
        match filelock.file.read_to_end(&mut cache_bytes) {
            Ok(_) => (),
            Err(e) => failure(&format!("Could not read from {}: {}", cache_path, e))
        }

        cache = match Cache::try_from(cache_bytes) {
            Ok(cache) => cache,
            Err(e) => failure(&format!("{}", e))
        };

        let now = SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("Could not retrieve UNIX timestamp")
            .as_millis();

        if now - cache.ts > period {
            filelock.file.rewind().expect("Could not seek to start of cache");

            output = execute_command(&args[COMMAND], &args[ARGS_START..]);
            cache = Cache {
                ts: current_timestamp(),
                output: output
            };

            match filelock.file.write_all(&cache.as_bytes()) {
                Ok(_) => (),
                Err(e) => failure(&format!("Could write to {}: {}", cache_path, e))
            }
        }
    }
    else {
        let lock_options = FileOptions::new().write(true).read(true).create_new(true);

        let mut filelock = match FileLock::lock(&cache_path, true, lock_options) {
            Ok(lock) => lock,
            Err(e) =>  failure(&format!("Could not acquire lock on cache file {}: {}", cache_path, e))
        };

        output = execute_command(&args[COMMAND], &args[ARGS_START..]);
        cache = Cache {
            ts: current_timestamp(),
            output: output
        };

        match filelock.file.write_all(&cache.as_bytes()) {
            Ok(_) => (),
            Err(e) => failure(&format!("Could write to {}: {}", cache_path, e))
        }
    };

    print!("{}", cache.output);
}
