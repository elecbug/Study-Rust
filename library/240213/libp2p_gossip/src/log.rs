use std::io::Write;
use std::path::Path;
use std::fs::{File, OpenOptions};
use chrono::offset::Local;

pub fn exist_and_create_log(file_name: &str) {
    if Path::new(file_name).exists() {
    }
    else {
        File::create(file_name)
            .expect("Can not create the log file");
    }
}

pub fn write_log(file: &str, text: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file)
        .expect("Can not open file");

    let text = format!("[{}] {text}\r\n", Local::now());

    file.write(text.as_bytes())
        .expect("Can not write when log file");
}