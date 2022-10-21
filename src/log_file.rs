use std::{fs, path::PathBuf, time::Duration};

use lazy_static::lazy_static;
use regex::Regex;

use crate::stopwatch;

#[derive(Debug)]
pub struct LogFile {
    pub path: PathBuf,
}

fn fmt_duration(duration: &Duration) -> String {
    let suffixes = ["n", "µ", "m"];
    let mut val = duration.as_nanos();
    let mut suffix_num = 0;

    while val > 1000 {
        suffix_num += 1;
        val /= 1000;
    }

    if suffix_num >= suffixes.len() {
        return format!("{val}s");
    }

    format!("{val}{}s", suffixes[suffix_num])
}

impl LogFile {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        Ok(Self { path })
    }

    pub fn keys(&mut self) -> Vec<String> {
        lazy_static! {
            static ref KEY_REGEX: Regex = Regex::new(r"(?m)^.*(?P<key>.+).*$").unwrap();
        }

        let mut keys = Vec::new();
        let (content, sw_read) = stopwatch(|| fs::read_to_string(&self.path));

        if let Err(e) = content {
            println!("'{}': {e:#?}", self.path.display());
            return Vec::new();
        }

        let content = content.unwrap();

        let (_, sw_regex) = stopwatch(|| {
            for key in KEY_REGEX.captures_iter(&content) {
                if let Some(key) = key.get(1) {
                    let key = key.as_str().to_lowercase();
                    keys.push(key);
                }
            }
        });

        println!("- Timings:");
        println!("- read_to_string: {}", fmt_duration(&sw_read));
        println!("- captures_iter: {}", fmt_duration(&sw_regex));

        keys
    }
}

unsafe impl Sync for LogFile {}
unsafe impl Send for LogFile {}
