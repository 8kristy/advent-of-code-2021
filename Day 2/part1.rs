use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    let mut x:i32 = 0;
    let mut y:i32 = 0;

    // Read the file line by line
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(ip) = line {
                // Get the components from the string
                let split = ip.split(" ");
                let vec: Vec<&str> = split.collect();
                let command = vec[0];
                let amount =  vec[1].parse::<i32>().unwrap();

                // Decide what to do based on the command
                match command {
                    _ if command == "forward" => x = x + amount,
                    _ if command == "down"    => y = y + amount,
                    _ if command == "up"      => y = y - amount,
                    _  =>  println!("Unknown keyword"),
                }
            }
        }
    }

    // Print product of x and y, i.e. our result
    println!("{}", x * y);
}

// Gets the lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}