use std::io;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

fn main() {
    println!("Enter integers separated by space:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");

    let k: usize = match k_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid value of k entered");
            return;
        }
    };

    match kth_smallest(&numbers, k) {
        Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
        None => println!("Invalid value of k or empty array."),
    }
}
