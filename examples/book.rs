use std::fs::File;
use std::io::BufReader;

fn main() {
    // read book.bin
    let mut reader = BufReader::new(File::open("book.dat").unwrap());
    let book: Vec<[isize; 3]> = bincode::deserialize_from(&mut reader).unwrap();
    println!("{}", book.len());
}
