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
// Reads input if it's one line and splits it into an int vector
pub fn read_and_split_line(filename: &str, split_sym: &str) -> Vec<i32> {
    if let Ok(mut lines) = read_lines(filename) {
        if let Ok(line) = lines.next().unwrap() {
            return line.split(split_sym).collect::<Vec<&str>>().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        }
    }
    return vec![0];
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

#[allow(dead_code)]
// Reads a file and puts each line as a string in an array which is then returned
pub fn get_input_lines_of_ints(filename: &str, length: usize) -> Vec<Vec<i32>> {
    let mut vec = Vec::with_capacity(length);
    
    // Go over lines and convert each to an integer and add it to our vector
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split("").collect::<Vec<&str>>();
                split.remove(0);
                split.remove(split.len() - 1);
                vec.push(split.into_iter().map(|x| x.parse::<i32>().unwrap()).collect());
            }
        }
    }
    return vec;
}

#[allow(dead_code)]
// Reads a file and puts each line as a string in an array which is then returned and also pads the 
// vector with a value around it
pub fn get_input_lines_of_ints_with_padding(filename: &str, length: usize, width: usize, pad: i32) -> Vec<Vec<i32>> {
    let mut vec = Vec::with_capacity(length);
    // Top padding
    vec.push(vec![pad; width]);
    // Go over lines and convert each to an integer and add it to our vector
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split("").collect::<Vec<&str>>();
                split.remove(0);
                split.remove(split.len() - 1);

                let mut split_int:Vec<i32> = split.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
                // Side padding
                split_int.insert(0, pad);
                split_int.push(pad);
                vec.push(split_int);
            }
        }
    }
    // Bottom padding
    vec.push(vec![pad; width]);
    return vec;
}


// Gets the lines
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}