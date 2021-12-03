use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(dead_code)]
// Reads a file and puts each line as an integer in an array which is then returned
pub fn get_input(filename: &str, length: usize) -> Vec<i32> {
    let mut vec = Vec::with_capacity(length);
    
    // Go over lines and convert each to an integer and add it to our vector
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                vec.push(ip.parse::<i32>().unwrap());
            }
        }
    }
    return vec;
}

#[allow(dead_code)]
// Reads a file and puts each line as a string in an array which is then returned
pub fn get_input_strings(filename: &str, length: usize) -> Vec<String> {
    let mut vec = Vec::with_capacity(length);
    
    // Go over lines and convert each to an integer and add it to our vector
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                vec.push(ip);
            }
        }
    }
    return vec;
}

// Gets the lines
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}