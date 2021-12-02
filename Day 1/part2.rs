#[path = "../reader.rs"]
mod reader;

fn main() {
    // Get the input
    let vec = reader::get_input("./input", 2000);
    // Counter for how many times the sliding window sum has increased
    let mut increased:i32 = 0;
    // Record previous sum for comparison
    let mut prev_sum:i32 = 0;

    // Go over the array, calculate the sliding window sum 
    // and compare it to the previous sum
    for n in 0..=vec.len()-3 {
        let sum:i32 = vec[n] + vec[n+1] + vec[n+2];
        // Check if previous sum is not 0 because we don't want to count 
        // the first increase from 0 to vec[n] + vec[n+1] + vec[n+2];
        // Assumes we don't have 3 consecutive zeros in our input
        if prev_sum != 0 && sum > prev_sum {
            increased = increased + 1;
        }
        // Record this sum
        prev_sum = sum;
    }

    // Print the result
    println!("{}", increased);
}