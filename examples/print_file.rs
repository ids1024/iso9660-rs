extern crate iso9660;

use std::io::{self, Write, Read};
use std::{env, process};

use iso9660::{ISO9660, DirectoryEntry};

fn main() {
    let args = env::args();

    if args.len() != 3 {
        eprintln!("Requires 2 arguments.");
        process::exit(1);
    }

    let iso_path = env::args().nth(1).unwrap();
    let file_path = env::args().nth(2).unwrap();

    let fs = ISO9660::new(iso_path).unwrap();

    match fs.open(&file_path).unwrap() {
        Some(DirectoryEntry::File(mut file)) => {
            let mut stdout = io::stdout();
            let mut text = Vec::new();
            file.read_to_end(&mut text).unwrap();
            stdout.write(&text).unwrap();
        }
        Some(_) => panic!("{} is not a file.", file_path),
        None => panic!("'{}' not found", file_path)
    }
}
