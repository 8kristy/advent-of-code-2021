use std::collections::HashMap;
mod common;

// Naive solution to the problem
fn main() {

    // Get all our initial values
    let mut template = common::read_template("./input");
    let rules = common::read_rules("./input");
    let mut count = count_letters(&template);

    for _iter in 0..10 {
        // Create a new temporary vector so we don't mess up the original and add 
        // the first character in our template to it
        let mut new_vec = Vec::with_capacity(template.len() * 2);
        new_vec.push(template[0].clone());

        for i in 0..template.len() - 1 {
            // Create the pair string from ith and (i+1) items in the list
            let pair = format!("{}{}", template[i], template[i + 1]);
            // Get the result of this production
            let res = rules[&pair].clone();
            // Increase the production letter's counter
            count.insert(res.clone(), if count.contains_key(&res.clone()) { count[&res.clone()] + 1 } else { 1 });
            // Add the production and next letter to our list to insert the production between the letters
            new_vec.push(res);
            new_vec.push(template[i + 1].clone());
        }
        template = new_vec;
    }

    // Find smallest and largest values
    let values: Vec<&i64> = count.values().collect();
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();

    // Print the result
    println!("{}", **max - **min);
}

// Initial letter counter; counts the number of each letter in a list of letters and puts that information in a dictionary
fn count_letters(letters: &Vec<String>) -> HashMap<String, i64> {
    let mut count = HashMap::new();
    for i in 0..letters.len() {
        count.insert(letters[i].clone(), if count.contains_key(&letters[i].clone()) { count[&letters[i].clone()] + 1 } else { 1 });
    }
    return count;
}
