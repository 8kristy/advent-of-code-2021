mod common;

fn main() {
    let (mut points, folds) = common::read_input();
    common::fold(&mut points, folds.clone(), folds.len());

    // Find the dimensions of our final picture
    let max_x = points.iter().map(|(x, _y)| x).max().unwrap();
    let max_y = points.iter().map(|(_x, y)| y).max().unwrap();

    // Draw the picture
    for i in 0..=*max_y {
        for j in 0..=*max_x {
            if points.contains(&(j, i)) {
                print!("#");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }
}