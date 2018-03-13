extern crate iso9660;

use std::{env, process};

use iso9660::{ISO9660, ISODirectory, DirectoryEntry};

fn main() {
    let args = env::args();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Requires 1 or 2 arguments.");
        process::exit(1);
    }

    let mut args = env::args().skip(1);
    let path = args.next().unwrap();
    let dirpath = args.next();

    let fs = ISO9660::new(path).unwrap();

    if let Some(dirpath) = dirpath {
        match fs.open(&dirpath).unwrap() {
            Some(DirectoryEntry::Directory(dir)) => {
                print_tree(&dir, 0);
            }
            Some(DirectoryEntry::File(_)) => {
                eprintln!("'{}' is not a directory", dirpath);
                process::exit(1);
            }
            None => {
                eprintln!("'{}' does not exist", dirpath);
                process::exit(1);
            }
        }
    } else {
        print_tree(&fs.root, 0);
    }
}

fn print_tree(dir: &ISODirectory, level: u32) {
    for entry in dir.contents() {
        match entry.unwrap() {
            DirectoryEntry::Directory(dir) => {
                if dir.identifier == "\0" || dir.identifier == "\u{1}" {
                    continue;
                }
                for _i in 0..level {
                    print!("  ");
                }
                println!("- {}/", dir.identifier);
                print_tree(&dir, level+1);
            },
            DirectoryEntry::File(file) => {
                for _i in 0..level {
                    print!("  ");
                }
                println!("- {}", file.identifier);
            }
        }
    }
}
