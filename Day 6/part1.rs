#[path = "../reader.rs"]
mod reader;

fn main() {
    let mut fish_vec = reader::read_and_split_line("./input", ",");

    // Naive solution where we go over the list and decrease the timer of every fish,
    // or if it's 0, set it to 6 and add a new fish with timer 8 to the list
    for _day in 1..=80 {
        for fish in 0..fish_vec.len() {
            // Wrap around and add a new fish
            if fish_vec[fish] == 0 {
                fish_vec[fish] = 6;
                fish_vec.push(8);
            }
            // Simply decrease the timer
            else {
                fish_vec[fish] = fish_vec[fish] - 1;
            }
        }
    }

    // Output how many fish we have in the end
    println!("{}", fish_vec.len())
}