use regex::bytes::Regex;
use std::{fs, str::FromStr};

mod lexer;

fn main() {
    // let re = Regex::new(r"\x6f\x62\x6a").unwrap();
    let input: Vec<u8> = fs::read("assets/TestPdf.pdf").expect("couldnt read a file");
    // let captures = re.find_iter(&input);

    // for capture in captures {
    //     println!("{:?}", capture.start());
    // }

    // println!("{:?}", captures);

    // let input = fs::read_to_string("src/Sommer.pdf").expect("couldn't read a file");
    // let lines: Vec<&str> = input.trim().split("\n").collect();

    // let mut string: String = "".to_string();
    // for symbol in input {
    //     string.push(symbol as char);
    // }
    lexer::tokenize(input);
    // let mut i: [u8; 3] = [111, 98, 106];
    // let ii = i.as_mut_ptr();
    // let iii = i.len();
    // let iiii = 0;
    // let o = 'o';
    // let b = 'b';
    // let j = 'j';
    // let e = 'e';
    // let n = 'n';
    // let d = 'd';
    // println!(
    //     "{:x} {:x} {:x} {:x} {:x} {:x}",
    //     o as u8, b as u8, j as u8, e as u8, n as u8, d as u8
    // );
    // unsafe {
    //     println!("{}", String::from_raw_parts(ii, iii, iiii));
    // }
    // fs::write("test.txt", string).expect("OH NO");
}
