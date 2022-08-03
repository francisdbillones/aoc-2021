use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    fs::write(
        "output/part1.txt",
        (input
            .lines()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .into_iter()
            .tuple_windows()
            .fold(0, |acc, (a, b)| acc + ((b > a) as u32)))
        .to_string(),
    )
    .expect("Failed to write to output/part1.txt");
}

fn part2(input: &str) {
    fs::write(
        "output/part2.txt",
        (input
            .lines()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .into_iter()
            .tuple_windows()
            .fold(0, |acc, (a, b, c, d)| {
                acc + ((b + c + d > a + b + c) as u32)
            }))
        .to_string(),
    )
    .expect("Failed to write to output/part2.txt");
}
