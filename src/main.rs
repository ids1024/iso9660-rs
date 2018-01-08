extern crate iso9660;

use std::{env, process};

use iso9660::ISO9660;

fn main() {
    let args = env::args();

    if args.len() != 2 {
        eprintln!("Requires 1 argument.");
        process::exit(1);
    }

    let path = env::args().nth(1).unwrap();
    let fs = ISO9660::new(path).unwrap();

    println!("{:#?}", fs.root.contents().unwrap());
}
