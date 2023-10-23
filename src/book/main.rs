use std::io::{BufRead, BufReader, BufWriter};
use std::str::FromStr;
use std::{fs, process::exit};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        eprintln!("missing input file, usage: cargo run --bin book `book file`");
        exit(1)
    }
    let input = fs::File::open(&args[0]).unwrap();
    let mut book: Vec<[isize; 3]> = Vec::new();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let mut tmp: Vec<isize> = Vec::new();
        for i in line.unwrap().split(",").collect::<Vec<&str>>() {
            tmp.push(FromStr::from_str(i).unwrap());
        }
        if tmp.len() == 3 {
            book.push([tmp[0], tmp[1], tmp[2]])
        }
    };
    let mut writer = BufWriter::new(fs::File::create("book.dat").unwrap());
    bincode::serialize_into(&mut writer, &book).unwrap();
    println!("success, write {} pieces of `book.bin`", book.len());
}
