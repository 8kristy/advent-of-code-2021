#[path = "../reader.rs"]
mod reader;

fn main() {
    let mut counter = 0;
    if let Ok(lines) = reader::read_lines("./input") {
        for line in lines {
            if let Ok(ip) = line {
                // Put the strings after the delimiter into a list
                let outputs:Vec<&str> = ip.split("| ").collect::<Vec<&str>>()[1].split(" ").collect();
                // Definite lengths we know
                let lengths = [2, 3, 4, 7];
                // Count how many of the outputs have the length in lengths and add that to a counter
                counter = counter + outputs.into_iter().filter(|x| lengths.contains(&x.len())).collect::<Vec<&str>>().len();
            }
        }
    }
    // Print the number of 1, 4, 7 or 8 in all our outputs
    println!("{}", counter);
}