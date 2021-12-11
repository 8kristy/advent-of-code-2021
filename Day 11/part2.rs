use std::process;
mod common;

#[path = "../reader.rs"]
mod reader;

fn main() {
    // Get the input array with padding
    let mut octopi = reader::get_input_lines_of_ints_with_padding("./input", 10, 12, i32::MIN);
    // Marks which octopi flashed at each step
    let mut marks = vec![vec![0; octopi[0].len()]; octopi.len()];
    // Keep track of iterations
    let mut iter = 0;

    // Loop indefinitely
    loop {
        // Go over the array
        for i in 1..octopi.len() - 1 {
            for j in 1..octopi[0].len() - 1 {
                // Increment every element
                octopi[i][j] = octopi[i][j] + 1;
                // If the octopus has an energy level higher than 9, mark it as flashed and call
                // subsequent flashing method
                if octopi[i][j] > 9 {
                    marks[i][j] = 1;
                    common::flash(&mut octopi, &mut marks, i, j);
                }
            }
        }
        iter = iter + 1;
        // After we find the first instance when the octopi synchronise, output the iteration 
        // number and exit
        if common::count_marks(&mut octopi, &mut marks) == 100 {
            println!("{}", iter);
            process::exit(1);
        }
    }
}
