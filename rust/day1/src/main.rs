use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    return input
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .into_iter()
        .tuple_windows()
        .fold(0, |acc, (a, b)| acc + ((b > a) as u32));
}

fn part2(input: &str) -> u32 {
    return input
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .into_iter()
        .tuple_windows()
        .fold(0, |acc, (a, b, c, d)| {
            acc + ((b + c + d > a + b + c) as u32)
        });
}
