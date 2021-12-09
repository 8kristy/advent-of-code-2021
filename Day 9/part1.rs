#[path = "../reader.rs"]
mod reader;

fn main() {
    // Put input into a 2D array
    let lava = reader::get_input_lines_of_ints("./input", 100);

    let mut risk_sum = 0;

    // Go over array
    for i in 0..lava[0].len() {
        for j in 0..lava.len() {
            // Define all adjacent values as 9 in case it's an edge
            let (mut top, mut bottom, mut left, mut right) = (9, 9, 9, 9);

            // Check all adjacent values; if it's an edge (i.e. out of bounds), don't do
            // anything and count that part as 9
            if i > 0 {
                left = lava[i - 1][j];
            }
            if i < lava[0].len() - 1 {
                right = lava[i + 1][j];
            }
            if j > 0 {
                top = lava[i][j - 1];
            }
            if j < lava.len() - 1 {
                bottom = lava[i][j + 1];
            }

            // Check if the current cell is the minimum point between all adjacent ones
            let cur = lava[i][j];
            if cur < top && cur < bottom && cur < left && cur < right {
                // If yes, as risk + 1 to risk sum
                risk_sum = risk_sum + cur + 1;
            }
        }
    }

    // Print the result
    println!("{}", risk_sum);
}