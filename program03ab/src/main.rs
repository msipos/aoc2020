use ndarray::Array2;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn load_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        // println!("{}", line.unwrap());
        lines.push(line.unwrap());
    }
    return lines;
}

fn solution(array: &Array2<i8>, xslope: usize, yslope: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let num_rows = array.shape()[0];
    let num_cols = array.shape()[1];

    let mut count = 0;

    loop {
        if array[[y, x]] == 1 {
            count += 1;
        }
        x += xslope;
        y += yslope;
        if y >= num_rows {
            break;
        }
        if x >= num_cols {
            x -= num_cols;
        }

    }
    println!("Xslope is {}, Yslope is {}, Count is: {}", xslope, yslope, count);
    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
    	println!("ERROR: use like this: ./program03a input03.txt");

    	return;
    }

    let lines = load_file(&args[1]);

    let num_rows = lines.len();
    let num_cols = lines[0].len();

    println!("{}x{}", num_rows, num_cols);

    let mut map = Array2::<i8>::zeros((num_rows, num_cols));
    for i in 0..num_rows {
        let char_vec: Vec<char> = lines[i].chars().collect();
        for j in 0..num_cols {
            if char_vec[j] == '#' {
                map[[i, j]] = 1;
            }
        }
    }
    let a = solution(&map, 1, 1);
    let b = solution(&map, 3, 1);
    let c = solution(&map, 5, 1);
    let d = solution(&map, 7, 1);
    let e = solution(&map, 1, 2);
    println!("product is {}", a*b*c*d*e);
}
