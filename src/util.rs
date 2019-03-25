use std::fs;
use std::path::Path;

pub fn dir_entries<P: AsRef<Path>>(path: P) -> usize {
    match fs::read_dir(path) {
        Err(_) => 0,
        Ok(d) => d.count(),
    }
}

pub fn read_link<P: AsRef<Path>>(path: P) -> String {
    fs::read_link(path)
        .unwrap_or_default()
        .into_os_string()
        .into_string()
        .unwrap_or_default()
}

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn read_file_null_separated<P: AsRef<Path>>(path: P) -> String {
    read_file(path)
        .chars()
        .map(|c| match c {
            '\0' => ' ',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}
