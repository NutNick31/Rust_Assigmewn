use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0];
    let mut longest_prefix = String::new();

    'outer: for (i, c) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(sc) = string.chars().nth(i) {
                if sc != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        longest_prefix.push(c);
    }
    longest_prefix
}

fn main() {
    println!("Enter strings separated by space:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let strings: Vec<String> = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let prefix = longest_common_prefix(&strings);

    if prefix.is_empty() {
        println!("No common prefix found.");
    } else {
        println!("Longest common prefix: {}", prefix);
    }
}
