use std::fs::File;
use std::io;
use std::io::Read;

pub fn pi() -> f64 {
    std::f64::consts::PI
}

pub fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}
