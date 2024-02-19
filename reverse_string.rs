use std::io;

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let reversed = reverse_string(input.trim());

    println!("Reversed string: {}", reversed);
}
