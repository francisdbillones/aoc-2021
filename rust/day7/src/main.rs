use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let nums: Vec<u32> = input
        .split_terminator(',')
        .map(|s| s.parse().expect("Couldn't parse input"))
        .collect();

    nums.iter()
        .map(|num| ((*num as i32) - (median(&nums) as i32)).abs())
        .sum::<i32>() as u32
}

fn part2(input: &str) -> u32 {
    let nums: Vec<u32> = input
        .split_terminator(',')
        .map(|s| s.parse().expect("Couldn't parse input"))
        .collect();

    let nums_mean = mean(&nums).floor() as u32;
    let answers = [nums_mean, nums_mean + 1];

    (answers.iter().map(|a| {
        nums.iter()
            .map(|n| {
                let u = (*n as i32) - (*a as i32);
                (u.pow(2) + u.abs()) / 2
            })
            .sum::<i32>()
    }))
    .min()
    .unwrap() as u32
}

fn median(nums: &[u32]) -> u32 {
    let n = nums.len();
    let sorted_nums = {
        let mut _nums = nums.to_owned();
        _nums.sort();
        _nums
    };

    sorted_nums[n / 2]
}

fn mean(nums: &[u32]) -> f32 {
    nums.iter().sum::<u32>() as f32 / (nums.len() as f32)
}
