#[path = "../reader.rs"]
mod reader;

fn main() {
    // Array to keep the scores in
    let mut scores = Vec::with_capacity(100);

    // Go over input line by line
    if let Ok(lines) = reader::read_lines("./input") {
        for line in lines {
            if let Ok(ip) = line {

                // Put our characters into a vector for easier iteration
                let split = ip.split("").collect::<Vec<&str>>();
                
                let mut stack = Vec::with_capacity(split.len() - 2);

                // Go over all brackets in a line
                for i in 1..split.len() - 1 {

                    // If it's an opening bracket, push it onto the stack
                    if split[i] == "(" || split[i] == "{" || split[i] == "[" || split[i] == "<" {
                        stack.push(split[i]);
                    }
                    // If it's a closing bracket, check if the opening bracket at the top of the stack matches it;
                    // if not, discard the whole line 
                    else {
                        let top = stack.pop().unwrap();
                        if get_match(top) != split[i] {
                            stack = Vec::new();
                            break;
                        }
                    }
                }

                // Reverse stack as brackets are in backwards order
                stack.reverse();

                // If there are extra items on the stack and there weren't any mismatched brackets,
                // go over the stack and get our score for the line
                let mut score:i64 = 0;
                for i in 0..stack.len() {
                    score = score * 5 + get_score(stack[i]);
                }

                // Push the score onto array
                if score > 0 {
                    scores.push(score);
                }
            }
        }
    }

    // Sort the array and print the middle value, i.e. the result
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}

// Helper function to get a matching closing brace for an opening one
fn get_match(brace: &str) -> &str {
    match brace {
        _ if brace == "(" => return ")",
        _ if brace == "[" => return "]",
        _ if brace == "{" => return "}",
        _  => return ">"
    }
}

// Helper function to get the score for the bracket
fn get_score(brace: &str) -> i64 {
    match brace {
        _ if brace == "(" => return 1,
        _ if brace == "[" => return 2,
        _ if brace == "{" => return 3,
        _  => return 4
    }
}