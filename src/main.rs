use std::{fs, str::FromStr};

fn main() {
    let input: Vec<u8> = fs::read("assets/TestPdf.pdf").expect("couldnt read a file");

    // let input = fs::read_to_string("src/Sommer.pdf").expect("couldn't read a file");
    // let lines: Vec<&str> = input.trim().split("\n").collect();

    for symbol in input {
        print!("{}", symbol as char);
    }
}
