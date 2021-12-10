#[path = "../reader.rs"]
mod reader;

fn main() {
    let mut error_sum = 0;

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

                    // If not, pop the last item on the stack and check if the closing bracket
                    // in our input matches; if not, add to our error_sum and move onto the next error
                    else {
                        let top = stack.pop().unwrap();
                        if get_match(top) != split[i] {
                            match split[i] {
                                _ if split[i] == ")" => error_sum = error_sum + 3,
                                _ if split[i] == "]" => error_sum = error_sum + 57,
                                _ if split[i] == "}" => error_sum = error_sum + 1197,
                                _  =>  error_sum = error_sum + 25137,
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    // Print the result
    println!("{}", error_sum);
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