use std::io::Write;
use std::path::Path;
use std::fs::{File, OpenOptions};
use chrono::offset::Local;
use libp2p::PeerId;

pub fn exist_and_create_log(file_name: &str) {
    if Path::new(file_name).exists() {
    }
    else {
        File::create(file_name)
            .expect("Can not create the log file");
    }
}

fn write_log(file: &str, text: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file)
        .expect("Can not open file");

    let text = format!("[{}]{text}\r\n", Local::now());

    file.write(text.as_bytes())
        .expect("Can not write when log file");
}

pub fn append_log(file: &str, target_id: Option<PeerId>, description: &str) {
    match target_id {
        Some(o) => write_log(file, format!("\r\n\tTarget: {o}\r\n\t- {description}").as_str()),
        None => write_log(file, format!("\r\n\t- {description}").as_str()),
    }
}