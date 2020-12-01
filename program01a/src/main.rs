use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
    	println!("ERROR: use like this: ./program01a input.txt");

    	return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut number_set = HashSet::new();
    
    for line in reader.lines() {
    	let number: i32 = line.unwrap().parse().unwrap();
        number_set.insert(number);
        let number2: i32 = 2020 - number;
        if number_set.contains(&number2) {
        	println!("Solution is: {}", number2*number);
        }
    }
}
