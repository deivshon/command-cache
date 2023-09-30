use std::{fs, io::Write};

use file_lock::FileLock;
use serde::{Deserialize, Serialize};

use crate::failure;

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub ts: u64,
    pub output: String,
}

impl Cache {
    pub fn write(&self, filelock: &mut FileLock, cache_path: &String) {
        let cache_bytes = match bincode::serialize(self) {
            Ok(cache_bytes) => cache_bytes,
            Err(e) => failure(format!("Could not serialize cache: {}", e).as_str()),
        };

        match filelock.file.write_all(&cache_bytes) {
            Ok(_) => (),
            Err(e) => {
                filelock.unlock().expect(
                    "Cache write failed, could not unlock cache file to delete before exiting",
                );

                fs::remove_file(&cache_path)
                    .expect("Cache write failed, could not delete cache file before exiting");

                failure(&format!("Could write to {}: {}", cache_path, e))
            }
        }
    }
}
