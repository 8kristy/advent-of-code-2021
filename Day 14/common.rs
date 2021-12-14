use std::collections::HashMap;

#[path = "../reader.rs"]
mod reader;

// Reads the first line and splits it into a vector of characters
pub fn read_template(filename: &str) -> Vec<String> {
    if let Ok(mut lines) = reader::read_lines(filename) {
        if let Ok(line) = lines.next().unwrap() {
            let mut split: Vec<&str> = line.split("").collect();
            split.remove(0);
            split.remove(line.len());
            return split.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        }
    }
    return Vec::new();
}

// Skips the first 2 lines and reads the rest of the output into a dictionary
pub fn read_rules(filename: &str) -> HashMap<String, String> {
    let mut rules = HashMap::new();
    if let Ok(mut lines) = reader::read_lines(filename) {
        lines.next();
        lines.next();
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(" -> ").collect();
                rules.insert(split[0].to_string(), split[1].to_string());
            }
        }
    }
    return rules;
}