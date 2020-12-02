#[macro_use] extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;

fn check_line(line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let num1: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
    let num2: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
    let ch = captures.get(3).unwrap().as_str();
    let ch: char = ch.chars().next().unwrap();
    let password = captures.get(4).unwrap().as_str();

    let mut count: i32 = 0;
    for pwd_ch in password.chars() {
        if ch == pwd_ch { count += 1; }
    }
    if count >= num1 && count <= num2 {
        return true;
    }

    return false;
}

fn process_file(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let mut count = 0;

    for line in reader.lines() {
        if check_line(&line.unwrap()) {
            count += 1;
        }
    }

    println!("{}", count);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
    	println!("ERROR: use like this: ./program02a input02.txt");

    	return;
    }

    process_file(&args[1]);
}
