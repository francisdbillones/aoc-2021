use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (instruction, mag) in input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(i, m)| (i, m.parse::<i32>().unwrap()))
    {
        match instruction {
            "forward" => x += mag,
            "up" => y += mag,
            "down" => y -= mag,
            _ => {}
        }
    }

    return (x * y).abs() as u32;
}

fn part2(input: &str) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;

    for (instruction, mag) in input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(i, m)| (i, m.parse::<i32>().unwrap()))
    {
        match instruction {
            "down" => aim += mag,
            "up" => aim -= mag,
            "forward" => {
                x += mag;
                y += aim * mag;
            }
            _ => {}
        }
    }

    return (x * y).abs() as u32;
}
