pub mod cache;
pub mod exec;

use crate::cache::Cache;
use crate::exec::execute_command;

use clap::Parser;
use file_lock::{FileLock, FileOptions};
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;

use bincode;
use md5;

const DEFAULT_CACHE_DIR: &str = "/tmp/command-cache";

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

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = DEFAULT_CACHE_DIR)]
    dir_cache: String,

    #[arg(short, long)]
    period: u64,

    #[arg(
        short,
        long,
        use_value_delimiter = true,
        value_delimiter = ',',
        required = true
    )]
    command: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.command.is_empty() {
        failure("A command must be specified");
    }

    let command_id = format!("{:?}", command_hash(&args.command));
    let cache_file = format!("{}/{}", args.dir_cache, command_id);

    if !Path::new(&args.dir_cache).is_dir() {
        fs::create_dir_all(&args.dir_cache).expect(&format!(
            "Could not create cache directory {}",
            &args.dir_cache
        ));
    };

    let mut output = String::new();
    let mut cache: Cache;

    if Path::new(&cache_file).exists() {
        let lock_options = FileOptions::new().write(true).read(true);

        let mut filelock = match FileLock::lock(&cache_file, true, lock_options) {
            Ok(lock) => lock,
            Err(e) => failure(&format!(
                "Could not acquire lock on cache file {}: {}",
                cache_file, e
            )),
        };

        let mut cache_bytes: Vec<u8> = Vec::new();
        match filelock.file.read_to_end(&mut cache_bytes) {
            Ok(_) => (),
            Err(e) => failure(&format!("Could not read from {}: {}", cache_file, e)),
        }

        cache = match bincode::deserialize(&cache_bytes) {
            Ok(cache) => cache,
            Err(e) => failure(&format!("bincode error: {}", e.to_string())),
        };

        if current_timestamp() - cache.ts > args.period {
            filelock
                .file
                .rewind()
                .expect("Could not seek to start of cache");

            store_output(
                &args.command[0],
                &args.command[1..],
                &mut output,
                &mut filelock,
                &cache_file,
            );

            cache = Cache {
                ts: current_timestamp(),
                output: output,
            };

            cache.write(&mut filelock, &cache_file);
        }
    } else {
        let lock_options = FileOptions::new().write(true).read(true).create_new(true);

        let mut filelock = match FileLock::lock(&cache_file, true, lock_options) {
            Ok(lock) => lock,
            Err(e) => failure(&format!(
                "Could not acquire lock on cache file {}: {}",
                cache_file, e
            )),
        };

        store_output(
            &args.command[0],
            &args.command[1..],
            &mut output,
            &mut filelock,
            &cache_file,
        );

        cache = Cache {
            ts: current_timestamp(),
            output: output,
        };

        cache.write(&mut filelock, &cache_file);
    };

    print!("{}", cache.output);
}
