use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    day1(&input);
    day2(&input);
}

fn day1(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .into_iter()
            .tuple_windows()
            .fold(0, |acc, (a, b)| acc + ((b > a) as u32))
    );
}

fn day2(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .into_iter()
            .tuple_windows()
            .fold(0, |acc, (a, b, c, d)| acc + ((b + c + d > a + b + c) as u32))
    );
}
