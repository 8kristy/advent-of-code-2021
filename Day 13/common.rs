use std::collections::HashSet;

#[path = "../reader.rs"]
mod reader;

// Reads the specific input
pub fn read_input() -> (HashSet<(i32, i32)>, Vec<(String, i32)>) {
    // Points are stored in a hashset of coordinates so that we don't have 
    // overlapping points stored twice
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    let mut folds: Vec<(String, i32)> = Vec::with_capacity(12);

    if let Ok(lines) = reader::read_lines("./input") {
        for line in lines {
            if let Ok(ip) = line {
                // Coordinate line; split the line and store the two numbers in a tuple which
                // is then pushed onto points set
                if ip.contains(",") {
                    let point: Vec<&str> = ip.split(",").collect();
                    points.insert((point[0].parse::<i32>().unwrap(), point[1].parse::<i32>().unwrap()));
                }
                // Fold line; find the instruction, split it and store in a (String, i32) tuple
                // which is then pushed onto the list
                else if ip.contains("fold") {
                    let fold: Vec<&str> = ip[11..].split("=").collect();
                    folds.push((fold[0].to_string(), fold[1].parse::<i32>().unwrap()));
                }
            }
        }
    }
    return (points, folds);
}

// Folds points with the first n instructions from folds array
pub fn fold(points: &mut HashSet<(i32, i32)>, folds: Vec<(String, i32)>, n: usize) {
    // Go over the required folds
    for i in 0..n {
        // Instead of deleting and adding points, we build a new set and reassign at the end
        let mut new_points = HashSet::new();

        for point in points.clone().iter() {
            let (x, y) = point;
            // We're folding on the x axis and our point is behind the fold line
            if folds[i].0 == "x" && x > &folds[i].1 {
                let diff = x - folds[i].1;
                new_points.insert((x - 2 * diff, *y));
            }
            // We're folding on the y axis and our point is behind the fold line
            else if folds[i].0 == "y" && y > &folds[i].1 {
                let diff = y - folds[i].1 ;
                new_points.insert((*x, y - 2 * diff));
            }
            // The point is on the correct side of the line
            else {
                new_points.insert((*x, *y));
            }
        }
        *points = new_points;
    }
}