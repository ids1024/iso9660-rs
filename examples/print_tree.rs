extern crate iso9660;

use std::{env, process};

use iso9660::{ISO9660, ISODirectory, DirectoryEntry};

fn main() {
    let args = env::args();

    if args.len() != 2 {
        eprintln!("Requires 1 argument.");
        process::exit(1);
    }

    let path = env::args().nth(1).unwrap();
    let fs = ISO9660::new(path).unwrap();

    print_tree(&fs.root, 0);
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
