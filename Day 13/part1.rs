mod common;

fn main() {
    let (mut points, folds) = common::read_input();
    common::fold(&mut points, folds, 1);
    println!("{}", points.len());
}