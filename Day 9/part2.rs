#[path = "../reader.rs"]
mod reader;

fn main() {
    // Put input into a 2D array
    let lava = reader::get_input_lines_of_ints("./input", 100);

    // Array to mark the squares we already visited when filling a basin
    let mut marks = vec![vec![0; lava[0].len()]; lava.len()];
    
    // Basin sizes
    let mut sizes = Vec::with_capacity(4);

    // Go over array
    for i in 0..lava.len() {
        for j in 0..lava[0].len() {
            // Find low points (similar to part 1)
            let (mut top, mut bottom, mut left, mut right) = (9, 9, 9, 9);
            if i > 0 {
                left = lava[i - 1][j];
            }
            if i < lava.len() - 2 {
                right = lava[i + 1][j];
            }
            if j > 0 {
                top = lava[i][j - 1];
            }
            if j < lava[0].len() - 2 {
                bottom = lava[i][j + 1];
            }

            // If the point is a low point, get the basin size for the 
            // surrounding area and put the size into our sizes array
            let cur = lava[i][j];
            if cur < top && cur < bottom && cur < left && cur < right {
                sizes.push(visit_cell(&lava, &mut marks, i, j, 1)); 
            }
        }
    }

    // Sort the sizes by descending order
    sizes.sort_by(|a, b| b.cmp(a));

    // Multipy 3 largest basin sizes and print the result
    println!("{}", sizes[0] * sizes[1] * sizes[2]);
}

// Performs a depth first search on a given cell and calculates how many
// cells we visited (return value)
fn visit_cell(grid: &Vec<Vec<i32>>, marks: &mut Vec<Vec<i32>>, i: usize, j: usize, basin_size: i32) -> i32 {
    // Check if we're at the edge, if the cell is marked or the cell is 9, all of which return 0
    if i > grid.len() - 1 || j > grid[0].len() - 1 || marks[i][j] == 1 || grid[i][j] == 9 {
        return 0;
    } 
    // Mark the cell as visited
    marks[i][j] = 1;

    // Perform depth first search on all adjacent squares

    // usize can't go below 0 so we need to check if our top and left values are
    // 0 before subtracting from them
    let mut left = 0;
    let mut top = 0;
    if i > 0 {
        left = visit_cell(&grid, marks, i - 1, j, basin_size + 1);
    }
    if j > 0 {
        top = visit_cell(&grid, marks, i, j - 1, basin_size + 1);
    }
    let right = visit_cell(&grid, marks, i + 1, j, basin_size + 1);
    let bottom = visit_cell(&grid, marks, i, j + 1, basin_size + 1);

    // Return the sum of all sizes near us + 1 (current cell)
    return left + right + top + bottom + 1;

}