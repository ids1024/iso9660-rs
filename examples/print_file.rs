extern crate iso9660;

use std::io::{self, Write, Read};
use std::{env, process};

use iso9660::{ISO9660, ISODirectory, DirectoryEntry};

fn main() {
    let args = env::args();

    if args.len() != 3 {
        eprintln!("Requires 2 arguments.");
        process::exit(1);
    }

    let iso_path = env::args().nth(1).unwrap();
    let file_path = env::args().nth(2).unwrap().to_uppercase();

    let fs = ISO9660::new(iso_path).unwrap();

    let mut parent = fs.root;
    let mut segments = file_path.split('/');
    let file_name = segments.next_back().unwrap();
    for segment in segments {
        let entry = match find_entry(&parent, segment) {
            Some(entry) => entry,
            None => panic!("'{}' not found", segment)
        };
        if let DirectoryEntry::Directory(dir) = entry {
            parent = dir;
        } else {
            panic!("{} is not a directory.", segment);
        }
    }

    match find_entry(&parent, file_name) {
        Some(DirectoryEntry::File(mut file)) => {
            let mut stdout = io::stdout();
            let mut text = Vec::new();
            file.read_to_end(&mut text).unwrap();
            stdout.write(&text).unwrap();
        }
        Some(_) => panic!("{} is not a file.", file_name),
        None => panic!("'{}' not found", file_name)
    }
}

fn find_entry(dir: &ISODirectory, identifier: &str) -> Option<DirectoryEntry> {
    for entry in dir.contents() {
        match entry.unwrap() {
            DirectoryEntry::Directory(dir) => {
                if dir.identifier == identifier {
                    return Some(DirectoryEntry::Directory(dir));
                }
            }
            DirectoryEntry::File(file) => {
                if file.identifier == identifier {
                    return Some(DirectoryEntry::File(file));
                }
            }
        }
    }

    None
}
