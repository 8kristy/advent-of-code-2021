use std::collections::HashMap;
use std::cmp;

#[path = "../reader.rs"]
mod reader;

fn main(){
    // Map to count intersections
    let mut vents = HashMap::new();

    // Read file
    if let Ok(lines) = reader::read_lines("./input") {
        for line in lines {
            if let Ok(mut ip) = line {
                // Remove whitespace for easier processing
                ip.retain(|c| !c.is_whitespace());

                // Split the line into 2 vectors containing the coordinates
                let pairs:Vec<&str> = ip.split("->").collect();
                let pair1:Vec<i32> = pairs[0].split(",").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
                let pair2:Vec<i32> = pairs[1].split(",").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

                // x1 = x2 so only get points going up and down
                if pair1[0] == pair2[0] {
                    for i in cmp::min(pair1[1], pair2[1])..=cmp::max(pair1[1], pair2[1]) {
                        let point = pair1[0].to_string() + "," + &i.to_string();
                        vents.insert(point.clone(), if vents.contains_key(&point.clone()) { vents[&point.clone()] + 1 } else { 1 });
                    }
                }
                // y1 = y2 so only get points going side to side
                else if pair1[1] == pair2[1]  {
                    for i in cmp::min(pair1[0], pair2[0])..=cmp::max(pair1[0], pair2[0]) {
                        let point = i.to_string() + "," + &pair1[1].to_string();
                        vents.insert(point.clone(), if vents.contains_key(&point.clone()) { vents[&point.clone()] + 1 } else { 1 });
                    }
                }
            }
        }
    }

    // Count the points where we have at least 2 intersections
    let mut sum = 0;
    for (point, num) in &vents {
        if num >= &2 {
            sum = sum + 1;
        }
    }

    // Output the result
    println!("{}", sum);
}