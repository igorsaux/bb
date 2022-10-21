use std::{fs, path::PathBuf};

use crate::log_file::LogFile;

#[derive(Debug)]
pub struct Logs {
    pub path: PathBuf,
}

impl Logs {
    pub fn new(folder: PathBuf) -> Self {
        Self { path: folder }
    }

    pub fn files(&self) -> Vec<LogFile> {
        let mut log_files = Vec::new();
        for dir in fs::read_dir(&self.path).unwrap() {
            let path = dir.unwrap().path();

            if path.is_dir() {
                continue;
            } else if path.extension().unwrap() == "log" {
                log_files.push(LogFile::new(path.to_owned()).unwrap())
            }
        }

        log_files
    }
}

unsafe impl Sync for Logs {}
unsafe impl Send for Logs {}
