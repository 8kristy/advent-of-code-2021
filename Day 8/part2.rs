use std::collections::HashSet;
use std::iter::FromIterator;

#[path = "../reader.rs"]
mod reader;

fn main() {
    let mut counter = 0;
    if let Ok(lines) = reader::read_lines("./input") {
        for line in lines {
            if let Ok(ip) = line {
                // Get both sides of the line and put them into lists
                let mut patterns:Vec<&str> = ip.split(" |").collect::<Vec<&str>>()[0].split(" ").collect();
                let outputs:Vec<&str> = ip.split("| ").collect::<Vec<&str>>()[1].split(" ").collect();

                // We will keep a set of fragments for each number, e.g. fragments[1] = {"e", "a"}
                let mut fragments:Vec<HashSet<&str>> = Vec::with_capacity(10);

                // Initialise the fragments to avoid out of bounds errors
                for _i in 0..10 {
                    fragments.push(HashSet::new());
                }

                // The approach is to keep finding the numbers we know by checking the set length and whether it
                // overlaps any of the already known numbers' sets, and if yes, put that set in our fragments list
                // and delete it from patterns to avoid interfering with further numbers

                // 1, 4, 7, 8 known
                fragments[1] = HashSet::from_iter((&patterns).into_iter().filter(|x| x.len() == 2).collect::<Vec<&&str>>()[0].split(""));
                patterns.retain(|x| x.len() != 2);
                fragments[4] = HashSet::from_iter((&patterns).into_iter().filter(|x| x.len() == 4).collect::<Vec<&&str>>()[0].split(""));
                patterns.retain(|x| x.len() != 4);
                fragments[7] = HashSet::from_iter((&patterns).into_iter().filter(|x| x.len() == 3).collect::<Vec<&&str>>()[0].split(""));
                patterns.retain(|x| x.len() != 3);
                fragments[8] = HashSet::from_iter((&patterns).into_iter().filter(|x| x.len() == 7).collect::<Vec<&&str>>()[0].split(""));
                patterns.retain(|x| x.len() != 7);

                // len=6 and overlaps 4 = 9
                fragments[9] = HashSet::from_iter((&patterns).into_iter()
                                .filter(|x| x.len() == 6 && fragments[4].is_subset(&HashSet::from_iter(x.split(""))))
                                .collect::<Vec<&&str>>()[0].split(""));
                patterns.retain(|x| !(x.len() == 6 && fragments[4].is_subset(&HashSet::from_iter(x.split("")))));

                // len=6 and overlaps 1 = 0
                fragments[0] = HashSet::from_iter((&patterns).into_iter()
                                .filter(|x| x.len() == 6 && fragments[1].is_subset(&HashSet::from_iter(x.split(""))))
                                .collect::<Vec<&&str>>()[0].split(""));
                patterns.retain(|x| !(x.len() == 6 && fragments[1].is_subset(&HashSet::from_iter(x.split("")))));

                // len=6 else = 6
                fragments[6] = HashSet::from_iter((&patterns).into_iter().filter(|x| x.len() == 6).collect::<Vec<&&str>>()[0].split(""));
                patterns.retain(|x| !(x.len() == 6));

                // len=5 and overlaps 1 = 3
                fragments[3] = HashSet::from_iter((&patterns).into_iter()
                                .filter(|x| x.len() == 5 && fragments[1].is_subset(&HashSet::from_iter(x.split(""))))
                                .collect::<Vec<&&str>>()[0].split(""));
                patterns.retain(|x| !(x.len() == 5 && fragments[1].is_subset(&HashSet::from_iter(x.split("")))));

                // len=5 and overlaps 6 = 5
                fragments[5] = HashSet::from_iter((&patterns).into_iter()
                                .filter(|x| x.len() == 5 && HashSet::from_iter(x.split("")).is_subset(&fragments[6]))
                                .collect::<Vec<&&str>>()[0].split(""));
                patterns.retain(|x| !(x.len() == 5 && HashSet::from_iter(x.split("")).is_subset(&fragments[6])));

                // len=5 else = 2
                fragments[2] = HashSet::from_iter((&patterns).into_iter().filter(|x| x.len() == 5).collect::<Vec<&&str>>()[0].split(""));

                // Decoded output
                let mut num_str:String = "".to_string();

                // Go over output
                for i in 0..4 {
                    let set:HashSet<&str> = HashSet::from_iter(outputs[i].split(""));
                    // Compare to each fragment set of a number
                    for j in 0..10 {
                        if set.eq(&fragments[j]) {
                            // If the same, we use the number
                            num_str = num_str + &j.to_string();
                        }
                    }
                }
                // Convert our output to an integer and add to our sum
                counter = counter + num_str.parse::<i32>().unwrap();                
            }
        }
    }
    // Print the sum of all outputs
    println!("{}", counter);
}