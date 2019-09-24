// Problem statement:
// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often) of the list.

use std::io;
use std::collections::HashMap;

fn main() {
    println!("Provide list of integers separated by spaces:");

    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)
               .expect("Failed to read line");

    let mut numbers: Vec<i32> = buffer.split_whitespace()
                                      .filter_map(|w| w.parse().ok())
                                      .collect();

    numbers.sort();

    match mean(&numbers) {
        Some(avg) => println!("Mean: {}", avg),
        None => println!("Couldn't find mean for {:?}", numbers)
    }

    match median(&numbers) {
        Some(median) => println!("Median: {}", median),
        None => println!("Couldn't find median for {:?}", numbers)
    }

    match mode(&numbers) {
        Some(mode) => println!("Mode: {}", mode),
        None => println!("Couldn't find mode for {:?}", numbers)
    }
}

fn mean(numbers: &Vec<i32>) -> Option<f64> {
    let sum: i32 = numbers.iter().sum();
    let maybe_mean = sum as f64 / numbers.len() as f64;
    if maybe_mean.is_nan() {
        None
    } else {
        Some(maybe_mean)
    }
}

fn median(numbers: &Vec<i32>) -> Option<&i32> {
    numbers.get(numbers.len() / 2)
}

fn mode(numbers: &Vec<i32>) -> Option<&i32> {
    let mut map = HashMap::new();

    for num in numbers.into_iter() {
        let count = map.entry(num).or_insert(0);
        *count +=1;
    }

    map.into_iter()
       .max_by_key(|&(_, count)| count)
       .map(|(val, _)| val)
}
