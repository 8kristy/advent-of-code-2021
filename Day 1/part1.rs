#[path = "../reader.rs"]
mod reader;

fn main() {
    // Get the input
    let vec = reader::get_input("./input", 2000);
    // Counter for how many times the number has increased
    let mut increased:i32 = 0;

    // Go over the array and count how many times the current number
    // is bigger than the previous
    for n in 1..=vec.len()-1 {
        if vec[n] > vec[n-1] {
            increased = increased + 1;
        }
    }

    // Print the result
    println!("{}", increased);
}