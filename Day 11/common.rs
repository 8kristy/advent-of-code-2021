// Goes over the array and counts the number of 1s in marks
// As it goes it also resets all octopi energy levels and the marks array itself
pub fn count_marks(octopi: &mut Vec<Vec<i32>>, marks: &mut Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for i in 0..octopi.len() {
        for j in 0..octopi[0].len() {
            if marks[i][j] == 1 {
                count = count + 1;
                octopi[i][j] = 0;
                marks[i][j] = 0;
            }
        }
    }
    return count;
}

// Flashes the octopus and any octopi around it which reached the threshold energy level
pub fn flash (octopi: &mut Vec<Vec<i32>>, marks: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    // Mark both grids
    octopi[i][j] = 0;
    marks[i][j] = 1;
    // Go over all adjacent cells (awkward iteration since usize doesn't allow negative numbers)
    for a in 0..=2 {
        for b in 0..=2 {
            // If we're not on current cell and the cell we're checking isn't marked
            if !(a == 1 && b == 1) && marks[i + a - 1][j + b - 1] == 0 {      
                // Increase the energy level          
                octopi[i + a - 1][j + b - 1] = octopi[i + a - 1][j + b - 1] + 1;
                // If the energy level is above 9, flash the octopus
                if octopi[i + a - 1][j + b - 1] > 9 {
                    flash(octopi, marks, i + a - 1, j + b - 1);
                }
            }
        }
    }
}