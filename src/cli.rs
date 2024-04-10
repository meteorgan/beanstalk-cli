use std::io;
use std::io::Read;

pub fn read_line_from_cli() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

pub fn read_bytes_from_cli(size: usize) -> Vec<u8> {
    let mut buffer = vec![0; size];
    io::stdin().read_exact(&mut buffer).unwrap();
    buffer
}