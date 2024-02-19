use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    println!("Enter integers separated by space:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("Enter the number to find:");
    let mut target_input = String::new();
    io::stdin().read_line(&mut target_input).expect("Failed to read line");

    let target: i32 = match target_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered");
            return;
        }
    };

    match find_first_occurrence(&numbers, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} is not found in the array", target),
    }
}
