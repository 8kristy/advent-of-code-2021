use std::collections::HashMap;
mod common;

fn main() {
    // Get our initial values
    let template = common::read_template("./input");
    let rules = common::read_rules("./input");
    let mut count = HashMap::new();
    let mut pairs = count_pairs(&template);

    for _iter in 0..40 {

        // Copy the dictionary as we're updating it in the loop which
        // would mess up the results
        let pairs_values: Vec<String> = pairs.keys().cloned().collect();
        let pairs_copy = pairs.clone();

        for i in 0..pairs_values.len(){
            // Get current pair
            let pair = pairs_values[i].clone();

            // Extract first and second letter of the rule
            let mut c = pair.chars();
            let first = c.next().unwrap().to_string();
            let second = c.next().unwrap().to_string();

            // Get the production
            let res = rules[&pair].clone();

            // Get the number of pairs so we can add the correct number
            let pair_count = pairs_copy[&pair];

            // Form 2 new pairs (e.g. AB->C forms AC and CB)
            let new_pair1 = format!("{}{}", first, res).to_string();
            let new_pair2 = format!("{}{}", res, second).to_string();

            // Add the pairs to our counter based on how many rule pairs we had
            pairs.insert(new_pair1.to_string(), if pairs.contains_key(&new_pair1) { pairs[&new_pair1] + pair_count} else { pair_count });
            pairs.insert(new_pair2.to_string(), if pairs.contains_key(&new_pair2) { pairs[&new_pair2] + pair_count} else { pair_count });

            // Remove one of the rule pairs as it is split by the production and no longer exits
            pairs.insert(pair.clone(), pairs[&pair] - pair_count);
            if pairs[&pair] <= 0 {
                pairs.remove(&pair);
            }
        }
    }

    // Get all the pairs and count the letters from them
    let pairs_keys: Vec<String> = pairs.keys().cloned().collect();

    // Only count the first letter of the pair to avoid doubles
    for i in 0..pairs_keys.len() {
        let mut c = pairs_keys[i].chars();
        let first = c.next().unwrap().to_string();
        count.insert(first.clone(), if count.contains_key(&first.clone()) { count[&first.clone()] + pairs[&pairs_keys[i]]} else { pairs[&pairs_keys[i]]});
    }

    // Add the last letter to the counters as it wasn't counted in our previous loop
    let last = template[template.len() - 1].clone();
    count.insert(last.clone(), if count.contains_key(&last.clone()) { count[&last.clone()] + 1} else { 1 });

    // Count min and max values
    let values: Vec<&i64> = count.values().collect();
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();

    // Output the result
    println!("{}", **max - **min);
}

// Initial pair count where we simply go over the template and add each pair to the dictionary
pub fn count_pairs(letters: &Vec<String>) -> HashMap<String, i64> {
    let mut count = HashMap::new();
    for i in 0..letters.len() - 1 {
        let pair = format!("{}{}", letters[i], letters[i + 1]);
        count.insert(pair.clone(), if count.contains_key(&pair.clone()) { count[&pair.clone()] + 1 } else { 1 });
    }
    return count;
}