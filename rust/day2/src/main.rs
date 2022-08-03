use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    day1(&input);
    day2(&input);
}

fn day1(input: &str) {
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

    fs::write("output/part1.txt", (x * y).abs().to_string()).expect("Failed to write to part1.txt");
}

fn day2(input: &str) {
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

    fs::write("output/part2.txt", (x * y).abs().to_string()).expect("Failed to write to part2.txt");
}
