#[path = "../reader.rs"]
mod reader;

fn main(){
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut aim:i32 = 0;

    // Read the file line by line
    if let Ok(lines) = reader::read_lines("./input") {
        for line in lines {
            if let Ok(ip) = line {
                // Get the components from the string
                let split = ip.split(" ");
                let vec: Vec<&str> = split.collect();
                let command = vec[0];
                let amount =  vec[1].parse::<i32>().unwrap();

                // Decide what to do based on the command
                match vec[0] {
                    _ if command == "forward" => {x = x + amount;
                                                  y = y + amount * aim},
                    _ if command == "down"    =>  aim = aim + amount,
                    _ if command == "up"      =>  aim = aim - amount,
                    _  =>  println!("Unknown keyword"),
                }
            }
        }
    }
    // Print product of x and y, i.e. our result
    println!("{}", x * y);
}