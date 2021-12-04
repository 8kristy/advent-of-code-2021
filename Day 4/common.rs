#[path = "../reader.rs"]
mod reader;

// Goes over a board and sums all unmarked values on it
pub fn sum_board(board: &Vec<Vec<i32>>) -> i32 {
    let mut sum:i32 = 0;
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j] != -1 {
                sum = sum + board[i][j];
            }
        }
    }
    return sum;
}

// Checks if the board won
pub fn check_board(board: &Vec<Vec<i32>>) -> bool {
    // Check rows
    for row in board {
        if row.into_iter().all(|x| *x == -1) {
            return true;
        }
    }

    // Transpose the board
    let mut columns = Vec::with_capacity(5);
    for i in 0..5{
        let mut column = Vec::with_capacity(5);
        for j in 0..5 {
            column.push(board[j][i])
        }
        columns.push(column);
    }

    // Check columns
    for col in columns {
        if col.into_iter().all(|x| x == -1) {
            return true;
        }
    }

    return false;
}

// Reads the first line of our input into an int array
pub fn read_numbers(filename: &str) -> Vec<i32> {
    if let Ok(mut lines) = reader::read_lines(filename) {
        if let Ok(line) = lines.next().unwrap() {
            return line.split(",").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        }
    }
    return vec![0];
}

// Reads the rest of the input into an array of 2D boards of size 5x5
pub fn read_boards(filename: &str, length: usize) -> Vec<Vec<Vec<i32>>> {
    let mut vec = Vec::with_capacity(length);

    if let Ok(mut lines) = reader::read_lines(filename) {
        lines.next(); // Skip the first line
        let mut count:i32 = 0; // Counter to keep track which line of the input are we on
        let mut rows = Vec::with_capacity(5); // Temporary holder for one board
        // Go over the input
        for line in lines {
            // Blank line, do nothing
            if count == 0 { }
            // Any part of the actual board
            else {
                // Turn the line into an int array
                if let Ok(ip) = line {
                    rows.push(ip.split(" ").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<i32>().unwrap()).collect());
                }
            }
            // Last row
            if count == 5 {
                // Push our board onto boards array
                vec.push(rows);
                // Reinitialise board
                rows = Vec::with_capacity(5);
                // Wrap around the counter
                count = -1;
            }
            // Increment counter to move onto next line
            count = count + 1;
        }
    }
    return vec;
}