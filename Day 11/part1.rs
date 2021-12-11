mod common;
#[path = "../reader.rs"]
mod reader;

fn main() {
    // Get the input array with padding
    let mut octopi = reader::get_input_lines_of_ints_with_padding("./input", 10, 12, i32::MIN);
    // Marks which octopi flashed at each step
    let mut marks = vec![vec![0; octopi[0].len()]; octopi.len()];
    // Counts the total number of flashes
    let mut flashes = 0;

    // Performs 100 steps
    for _iter in 0..100 {
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
        // After each step, count the number of octopi that flashed, reset them and the mark
        flashes = flashes + common::count_marks(&mut octopi, &mut marks);
    }
    // Print the sum of flashes
    println!("{}", flashes);
}

