#[path = "../reader.rs"]
mod reader;

fn main(){
    let mut ones:Vec<i32> = vec![0; 12];
    let mut zeroes:Vec<i32> = vec![0; 12];

    // Get the input
    let lines = reader::get_input_strings("./input", 1000);

    // Count the ones an zeroes in each column
    for line in lines {
        for i in 0..=line.len()-1 {
            if line.as_bytes()[i] == 49 {
                ones[i] = ones[i] + 1;
            }
            else {
                zeroes[i] = zeroes[i] + 1;
            }
        }                 
    }

    // Get the most common (gamma) and least common (epsilon) values for each column
    // and put them into a string
    let gamma:String = (&ones).into_iter()
        .zip((&zeroes).into_iter())
        .map(|(x, y)| ((x > &y) as i32).to_string())
        .collect::<Vec<String>>()
        .join("");

    let epsilon:String = ones.into_iter()
        .zip(zeroes.into_iter())
        .map(|(x, y)| ((x <= y) as i32).to_string())
        .collect::<Vec<String>>()
        .join("");

    // Convert binary string to decimal value
    let gamma_dec:isize = isize::from_str_radix(&*gamma, 2).unwrap();
    let epsilon_dec:isize = isize::from_str_radix(&*epsilon, 2).unwrap();

    // Print product of gamma and epsilon in decimal, i.e. our result
    println!("{}", gamma_dec * epsilon_dec);
}