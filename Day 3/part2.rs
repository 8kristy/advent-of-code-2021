#[path = "../reader.rs"]
mod reader;

fn main(){

    // Get two separate lists as we will need to take elements out of them
    let mut oxygen_gen_rating_vec = reader::get_input_strings("./input", 1000);
    let mut co2_scrub_rating_vec = reader::get_input_strings("./input", 1000);

    // Repeat for each column until the vector size is 1
    // - recalculate the numbers of 1s and 0s in our new vector
    // - keep only the numbers with most common number (oxygen) or least common number (epsilon)
    //   in each corresponding vector
    for i in 0..12 {
        if oxygen_gen_rating_vec.len() > 1{
            let (ones, zeroes) = recalculate_occurance(&oxygen_gen_rating_vec);
            oxygen_gen_rating_vec = oxygen_gen_rating_vec.into_iter()
                .filter(|x| (ones[i] >= zeroes[i] && x.as_bytes()[i] == 49) // 1 is most common
                         || (ones[i] < zeroes[i] && x.as_bytes()[i] == 48)) // 0 is most common
                .collect();
        }
        if co2_scrub_rating_vec.len() > 1{
            let (ones, zeroes) = recalculate_occurance(&co2_scrub_rating_vec);
            co2_scrub_rating_vec = co2_scrub_rating_vec.into_iter()
                .filter(|x| (ones[i] < zeroes[i] && x.as_bytes()[i] == 49)   // 1 is least common
                         || (ones[i] >= zeroes[i] && x.as_bytes()[i] == 48)) // 0 is least common
                .collect();
        }
    }

    // Convert binary string to decimal
    let oxygen_gen_rating:isize = isize::from_str_radix(&*oxygen_gen_rating_vec[0], 2).unwrap();
    let co2_scrub_rating:isize = isize::from_str_radix(&*co2_scrub_rating_vec[0], 2).unwrap();

    // Print product of oxygen and co2 in decimal, i.e. our result
    println!("{}", oxygen_gen_rating * co2_scrub_rating);

}

// Helper function for recalculating ones and zeroes counters each time
fn recalculate_occurance(vec:&Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut ones:Vec<i32> = vec![0; 12];
    let mut zeroes:Vec<i32> = vec![0; 12];
    
     // Count the ones an zeroes in each column
     for line in vec {
        for i in 0..=line.len()-1 {
            if line.as_bytes()[i] == 49 {
                ones[i] = ones[i] + 1;
            }
            else {
                zeroes[i] = zeroes[i] + 1;
            }
        }                 
    }
    return (ones, zeroes);
}