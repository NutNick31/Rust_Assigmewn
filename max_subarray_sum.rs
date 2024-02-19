use std::io;

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = num.max(current_sum + num);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    println!("Enter integers separated by space:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let max_sum = max_subarray_sum(&numbers);
    println!("Maximum subarray sum: {}", max_sum);
}
