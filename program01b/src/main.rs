use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_numbers(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut result: Vec<i32> = Vec::new();
    
    for line in reader.lines() {
    	let number: i32 = line?.parse()?;
    	result.push(number);
    }

    return Ok(result);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
    	println!("ERROR: use like this: ./program01a input.txt");

    	return;
    }

    let numbers = load_numbers(&args[1]).unwrap();
    
    for i in 0..numbers.len() {
    	for j in i+1..numbers.len() {
    		for k in j+1..numbers.len() {
    			let n1 = numbers[i];
    			let n2 = numbers[j];
    			let n3 = numbers[k];
    			if n1 + n2 + n3 == 2020 {
    				println!("Solution is: {}", n1*n2*n3);
    			}
    		}
    	}
    }
}
