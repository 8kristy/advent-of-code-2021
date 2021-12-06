#[path = "../reader.rs"]
mod reader;

// A more sofisticated solution where we keep the number of fish with the same age
fn main() {
    let fish_vec = reader::read_and_split_line("./input", ",");
    // Counter for our fish
    let mut fish_map:Vec<i64> = vec![0; 9];

    // Set the initial state
    for fish in fish_vec{
        fish_map[fish as usize] = fish_map[fish as usize] + 1;
    }

    for _day in 1..=256 {
        // Keep the wrap around value to update things later as we don't want
        // to use this value while iterating
        let tmp = fish_map[0];
        // Push every capacity in the middle of the list down 
        for num in 0..8 {
            fish_map[num as usize] = fish_map[((num + 1) % 9) as usize]
        }
        // Add fish that wrap around to 6
        fish_map[6] = fish_map[6] + tmp;
        // Add "new born" fish
        fish_map[8] = tmp;
    }

    // Sum the number of fish and print the result
    println!("{}", fish_map.iter().sum::<i64>());
}