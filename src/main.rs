pub mod cache;
pub mod command;

use crate::cache::Cache;
use crate::command::execute_command;

use file_lock::{FileLock, FileOptions};
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;

use bincode;
use md5;

const CACHE_DIR: &str = "/tmp/command-cache";

const COMMAND_CACHE_ARG: usize = 1;
const TIME_LIMIT: usize = 1;
const COMMAND: usize = 2;
const ARGS_START: usize = 3;

const PURGE: &str = "--purge";

fn failure(msg: &str) -> ! {
    eprintln!("command-cache: {}", msg);
    exit(1)
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
fn current_timestamp() -> u64 {
    SystemTime::UNIX_EPOCH
        .elapsed()
        .expect("Could not retrieve UNIX timestamp")
        .as_millis() as u64
}

fn store_output(
    command: &String,
    args: &[String],
    output: &mut String,
    filelock: &mut FileLock,
    cache_path: &String,
) {
    *output = match execute_command(command, args) {
        Ok(output) => output,
        Err(e) => {
            filelock
                .unlock()
                .expect("Command failed, could not unlock cache file to delete before exiting");

            fs::remove_file(cache_path)
                .expect("Command failed, could not delete cache file before exiting");

            failure(&format!("{}", e))
        }
    };
}

fn cache_write(filelock: &mut FileLock, cache: &Cache, cache_path: &String) {
    let cache_bytes = match bincode::serialize(&cache) {
        Ok(cache_bytes) => cache_bytes,
        Err(e) => failure(format!("Could not serialize cache: {}", e).as_str()),
    };

    match filelock.file.write_all(&cache_bytes) {
        Ok(_) => (),
        Err(e) => {
            filelock
                .unlock()
                .expect("Cache write failed, could not unlock cache file to delete before exiting");

            fs::remove_file(&cache_path)
                .expect("Cache write failed, could not delete cache file before exiting");

            failure(&format!("Could write to {}: {}", cache_path, e))
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[COMMAND_CACHE_ARG] == PURGE {
        if Path::new(CACHE_DIR).exists() {
            match fs::remove_dir_all(CACHE_DIR) {
                Ok(_) => exit(0),
                Err(e) => failure(&format!("Could not purge cache: {}", e)),
            }
        } else {
            failure("Cache is already empty")
        }
    }

    if args.len() < 3 {
        failure("Not enough arguments");
    }

    let Ok(period) = args[TIME_LIMIT].parse::<u64>() else {
        failure("Could not parse time limit");
    };

    let command_id = format!("{:?}", command_hash(&args[COMMAND..]));
    let cache_path = format!("{}/{}", CACHE_DIR, command_id);

    if !Path::new(CACHE_DIR).is_dir() {
        fs::create_dir_all(CACHE_DIR)
            .expect(&format!("Could not create cache directory {}", CACHE_DIR));
    };

    let mut output = String::new();
    let mut cache: Cache;

    if Path::new(&cache_path).exists() {
        let lock_options = FileOptions::new().write(true).read(true);

        let mut filelock = match FileLock::lock(&cache_path, true, lock_options) {
            Ok(lock) => lock,
            Err(e) => failure(&format!(
                "Could not acquire lock on cache file {}: {}",
                cache_path, e
            )),
        };

        let mut cache_bytes: Vec<u8> = vec![];
        match filelock.file.read_to_end(&mut cache_bytes) {
            Ok(_) => (),
            Err(e) => failure(&format!("Could not read from {}: {}", cache_path, e)),
        }

        cache = match bincode::deserialize(&cache_bytes) {
            Ok(cache) => cache,
            Err(e) => failure(&format!("{}", e)),
        };

        if current_timestamp() - cache.ts > period {
            filelock
                .file
                .rewind()
                .expect("Could not seek to start of cache");

            store_output(
                &args[COMMAND],
                &args[ARGS_START..],
                &mut output,
                &mut filelock,
                &cache_path,
            );

            cache = Cache {
                ts: current_timestamp(),
                output: output,
            };

            cache_write(&mut filelock, &cache, &cache_path);
        }
    } else {
        let lock_options = FileOptions::new().write(true).read(true).create_new(true);

        let mut filelock = match FileLock::lock(&cache_path, true, lock_options) {
            Ok(lock) => lock,
            Err(e) => failure(&format!(
                "Could not acquire lock on cache file {}: {}",
                cache_path, e
            )),
        };

        store_output(
            &args[COMMAND],
            &args[ARGS_START..],
            &mut output,
            &mut filelock,
            &cache_path,
        );

        cache = Cache {
            ts: current_timestamp(),
            output: output,
        };

        cache_write(&mut filelock, &cache, &cache_path);
    };

    print!("{}", cache.output);
}
