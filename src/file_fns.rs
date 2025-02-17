use crate::error_flg;
use crate::vcf_parser;
use crate::arg_and_help;

pub use std::path::MAIN_SEPARATOR;
pub use std::{env, path::Path};
pub use std::fs::{copy, read_dir, remove_file, rename};
pub use std::io::prelude::*;

use error_flg::*;
use vcf_parser::*;
use arg_and_help::*;

/// Return filename + ".tmp"
pub fn make_tmp_filename(filename: &str) -> String {
    format!("{}.tmp", &filename)
}

/// Delete file
pub fn delete_file(filename: &str) {
    if let Err(_) = remove_file(&filename) {()};
}

/// File backup with auto increment filename
pub fn file_backup(filename: &str) -> Result<(), i32> {
    let bup_filename = get_new_bup_filename(&filename);
    match copy(&filename, &bup_filename) {
        Ok(_) => Ok(()),
        Err(_) => Err(_ERR_FILE_BACKUP),
    }
}

/// Return file list on same file path
pub fn get_filelist_same_dir(filename: &str) -> Option<Vec<String>> {
    // Get Dirs
    let path = match Path::new(&filename).parent() {
        Some(p) =>  p.to_str().map_or("", |s| s),
        _ => { return None; },
    };
    let dirs = match read_dir(path) {
        Ok(d) => d,
        _ => { return None; },
    };
    // Detect files
    let mut files: Vec<String> = Vec::new();
    for entry in dirs {
        let one_entry = match entry {
            Ok(de) => de,
            _ => { continue; },
        };
        if !one_entry.path().is_dir() {
            if let Some(s) = one_entry.path().to_str() {
                files.push(s.to_string());
            };
        }
    }
    Some(files)
}

/// Return made new self backup file name
pub fn get_new_bup_filename(filename: &str) -> String {
    let mut last_number = 0;
    let files = match get_filelist_same_dir(&filename) {
        Some(v) =>  v,
        _ => { return format!("{}.bup0001", &filename); },
    };
    // Detect last number
    let escaped_filename = regex::escape(&filename);
    let pat = format!(r"{}.bup([0-9]{{4}})+", &escaped_filename);
    let re = Regex::new(&pat).unwrap();
    for one_filename in files {
        let cap = re.captures(&one_filename);
        if let Some(c) = cap {
            let one_number = c.get(1).map_or("", |m| m.as_str());
            let one_number: i32 = one_number.parse().unwrap();
            if last_number < one_number {
                last_number = one_number;
            };
        };
    }
    last_number += 1;
    format!("{}.bup{:04}", &filename, last_number)
}

/// Check required files exists
pub fn is_exists_io_files(args: &Args) -> bool {
    Path::new(args.load_file_name()).is_file() && Path::new(args.save_file_name()).is_file()
}
