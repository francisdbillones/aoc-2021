use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let nbits = input.lines().next().unwrap().len();

    let counter = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '1' => 1,
                    '0' => -1,
                    _ => panic!("Expected only 0 or 1"),
                })
                .collect::<Vec<i32>>()
        })
        .fold(vec![0; nbits], |acc, v| {
            (0..nbits).map(|i| acc[i] + v[i]).collect()
        });

    let gamma = counter
        .into_iter()
        .map(|n| (n >= 0) as bool)
        .collect::<Vec<bool>>();

    let epsilon = gamma
        .iter()
        .map(|bit| match bit {
            false => true,
            true => false,
        })
        .collect::<Vec<bool>>();

    return vec_to_n(gamma) * vec_to_n(epsilon);
}

fn part2(input: &str) -> u32 {
    let nbits = input.lines().next().unwrap().len();

    let bytes = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!("Expected only 0 or 1"),
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let o2_rating = vec_to_n(o2(&bytes, nbits));
    let co2_rating = vec_to_n(co2(&bytes, nbits));

    return o2_rating * co2_rating;
}

fn o2(bytes: &Vec<Vec<bool>>, nbits: usize) -> Vec<bool> {
    assert!(bytes.len() > 0);
    assert!(bytes[0].len() > 0);

    let mut sieve = (0..bytes.len()).collect::<Vec<usize>>();

    for bit_i in 0..nbits {
        let most_common_bit = sieve
            .iter()
            .map(|i| &bytes[*i])
            .map(|byte| match (*byte)[bit_i] {
                true => 1,
                false => -1,
            })
            .fold(0, |acc, s| acc + s)
            >= 0;

        sieve = sieve
            .iter()
            .copied()
            .filter(|i| bytes[*i][bit_i as usize] == most_common_bit)
            .collect::<Vec<usize>>();

        let filtered_bytes = sieve
            .iter()
            .map(|i| &(bytes[*i]))
            .collect::<Vec<&Vec<bool>>>();

        if filtered_bytes.len() == 1 {
            return filtered_bytes.iter().nth(0).unwrap().to_vec();
        }
    }

    panic!("Couldn't find o2 byte");
}

fn co2(bytes: &Vec<Vec<bool>>, nbits: usize) -> Vec<bool> {
    assert!(bytes.len() > 0);
    assert!(bytes[0].len() > 0);

    let mut sieve = (0..bytes.len()).collect::<Vec<usize>>();

    for bit_i in 0..nbits {
        let most_common_bit = sieve
            .iter()
            .map(|i| &bytes[*i])
            .map(|byte| match (*byte)[bit_i] {
                true => 1,
                false => -1,
            })
            .fold(0, |acc, s| acc + s)
            < 0;

        sieve = sieve
            .iter()
            .copied()
            .filter(|i| bytes[*i][bit_i as usize] == most_common_bit)
            .collect::<Vec<usize>>();

        let filtered_bytes = sieve
            .iter()
            .map(|i| &(bytes[*i]))
            .collect::<Vec<&Vec<bool>>>();

        if filtered_bytes.len() == 1 {
            return filtered_bytes.iter().nth(0).unwrap().to_vec();
        }
    }

    panic!("Couldn't find co2 byte");
}

fn vec_to_n(vec: Vec<bool>) -> u32 {
    return vec
        .iter()
        .fold(0, |acc: u32, bit| (acc << 1) + (*bit as u32));
}
