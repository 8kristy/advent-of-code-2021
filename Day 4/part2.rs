use std::process;

mod common;

fn main(){
    // Get our input
    let numbers:Vec<i32> = common::read_numbers("./input");
    let mut boards:Vec<Vec<Vec<i32>>> = common::read_boards("./input", 100);

    // Mark the boards that won before
    let mut wins:Vec<i32> = vec![0; 100];

    // Mark the number in each board
    for number in numbers{
        for board in 0..100{
            for i in 0..5 {
                for j in 0..5 {
                    if boards[board][i][j] == number {
                        boards[board][i][j] = -1;
                    }
                }
            }
            // Check if the current board won and if yes, mark it in our wins list
            if common::check_board(&(boards[board].to_vec())) {
                wins[board] = 1;
                // If we don't have any more boards which didn't win, this is the last one 
                // so output the result and exit
                if !(&wins).into_iter().any(|x| *x == 0) {
                    println!("{}", number * common::sum_board(&(boards[board].to_vec())));
                    process::exit(1);
                }
            }
        }
    }
}