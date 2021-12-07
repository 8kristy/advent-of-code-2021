#[path = "../reader.rs"]
mod reader;

fn main(){
    let crabs = reader::read_and_split_line("./input", ",");

    // Get min and max values in list so we don't need to iterate over 
    // numbers out of range (would make all crabs move which probably means more fuel)
    let min = *(&crabs).into_iter().min().unwrap();
    let max = *(&crabs).into_iter().max().unwrap();

    // Counter for minimum fuel so far
    let mut min_fuel = i32::MAX;

    // Go over all positions
    for num in min..=max {
        // Calculate fuel for this position by summing the fuel for all crabs
        let mut fuel = 0;
        for i in 0..crabs.len() {
            fuel = fuel + (num - crabs[i]).abs();
        }

        // Update fuel if it's less than current minimum
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    // Print our result
    println!("{}", min_fuel);
}
