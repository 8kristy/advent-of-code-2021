use std::process;

mod common;

fn main(){
    // Get our input
    let numbers:Vec<i32> = common::read_numbers("./input");
    let mut boards:Vec<Vec<Vec<i32>>> = common::read_boards("./input", 100);
    // Mark the number in each board
    for number in numbers{
        for board in 0..99{
            for i in 0..5 {
                for j in 0..5 {
                    if boards[board][i][j] == number {
                        boards[board][i][j] = -1;
                    }
                }
            }
            // Check each board after marking it to see if it wins; if it does, output 
            // the result of first winning board and exit
            if common::check_board(&(boards[board].to_vec())) {
                println!("{}", number * common::sum_board(&(boards[board].to_vec())));
                process::exit(1);
            }
        }
    }
}