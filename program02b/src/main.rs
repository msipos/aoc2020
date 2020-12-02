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
    let num1: usize = captures.get(1).unwrap().as_str().parse().unwrap();
    let num2: usize = captures.get(2).unwrap().as_str().parse().unwrap();
    let ch = captures.get(3).unwrap().as_str();
    let ch: char = ch.chars().next().unwrap();
    let password = captures.get(4).unwrap().as_str();

    let ch1: char = password.chars().nth(num1-1).unwrap();
    let ch2: char = password.chars().nth(num2-1).unwrap();

    let mut count: i32 = 0;
    if ch == ch1 {
        count += 1;
    }
    if ch == ch2 {
        count += 1;
    }

    if count == 1 {
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
