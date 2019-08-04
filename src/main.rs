use rust_cnc::{process};
use std::process::exit;
use std::env::Args;
use std::fs::read_to_string;
use std::io::Error;

pub struct Config {
    pub dxf_file: String
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let dxf_file = match args.next() {
            Some(s) => s,
            None => return Err("No DXF file provided")
        };

        Ok(Config {dxf_file})
    }
}

fn read_dxf(file: &str) -> Result<String, Error> {
    read_to_string(file)
}

fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|e| {
        eprintln!("Argument error: {}", e);
        exit(1);
    });

    let dxf_contents = read_dxf(&config.dxf_file).unwrap_or_else(|e| {
        eprintln!("Processing failed: {}", e);
        exit(1);
    });

    let lines = process(&dxf_contents);

    println!("{}", lines);
}
