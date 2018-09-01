use config;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn debug(string: &str) {
    if config::DEBUG {
        println!("{}", string);
    }
}

pub fn write_to_file(line: String, append: bool) {
    let path = Path::new(config::FILE_PATH);

    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn create_file() {
    let path = Path::new(config::FILE_PATH);
    let display = path.display();

    match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
}
