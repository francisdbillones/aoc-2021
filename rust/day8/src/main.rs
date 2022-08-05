use phf::phf_map;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

const SIGNAL_TO_SUM: phf::Map<&str, usize> = phf_map! {
    "abcefg" => 0,
    "cf" => 1,
    "acdeg" => 2,
    "acdfg" => 3,
    "bcdf" => 4,
    "abdfg" => 5,
    "abdefg" => 6,
    "acf" => 7,
    "abcdefg" => 8,
    "abcdfg" => 9,
};

fn part1(input: &str) -> u32 {
    let input: Vec<(Vec<String>, Vec<String>)> = input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(signals, queries)| {
            (
                signals
                    .split_terminator(' ')
                    .map(String::from)
                    .collect::<Vec<String>>(),
                queries
                    .split_terminator(' ')
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        })
        .collect();

    let queries: Vec<&String> = input.iter().flat_map(|(_, query)| query).collect();

    let uniques = [2, 4, 3, 7];

    queries
        .iter()
        .map(|query| query.len())
        .filter(|len| uniques.contains(len))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let input: Vec<(Vec<String>, Vec<String>)> = input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(signals, queries)| {
            (
                signals
                    .split_terminator(' ')
                    .map(String::from)
                    .collect::<Vec<String>>(),
                queries
                    .split_terminator(' ')
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        })
        .collect();

    let signals: Vec<&Vec<String>> = input.iter().map(|(signal, _)| signal).collect();
    let queries: Vec<&Vec<String>> = input.iter().map(|(_, query)| query).collect();

    let ans: usize = std::iter::zip(signals, queries)
        .map(|(signal, query)| {
            let solution = solve_system(signal);
            query
                .iter()
                .enumerate()
                .map(|(i, segment)| {
                    SIGNAL_TO_SUM
                        [&sort_string(&(segment.chars().map(|c| solution[&c]).collect::<String>()))]
                        * (10_usize.pow(3 - (i as u32)))
                })
                .sum::<usize>()
        })
        .sum();

    ans as u32
}

fn solve_system(signal: &[String]) -> HashMap<char, char> {
    let mut length_to_signal: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();

    for segment in signal {
        match length_to_signal.get_mut(&(segment.len())) {
            Some(vec) => {
                vec.push(HashSet::from_iter(segment.chars()));
            }
            None => {
                length_to_signal.insert(segment.len(), vec![HashSet::from_iter(segment.chars())]);
            }
        }
    }

    let (eight, one, four, seven) = (
        &length_to_signal[&7][0],
        &length_to_signal[&2][0],
        &length_to_signal[&4][0],
        &length_to_signal[&3][0],
    );

    let a = *seven.difference(one).next().unwrap();

    let nine = length_to_signal[&6]
        .iter()
        .find(|segment| four.is_subset(segment))
        .unwrap();

    let e = *eight.difference(nine).next().unwrap();

    let g = *HashSet::<char>::from_iter(eight.symmetric_difference(four).into_iter().copied())
        .difference(&HashSet::<char>::from_iter([a, e].into_iter()))
        .next()
        .unwrap();

    let zero = length_to_signal[&6]
        .iter()
        .find(|segment| {
            !(segment.symmetric_difference(nine).count() == 0
                || one
                    .intersection(&HashSet::from_iter(
                        nine.symmetric_difference(*segment).copied(),
                    ))
                    .count()
                    != 0)
        })
        .unwrap();

    let d = **(HashSet::from_iter(eight.symmetric_difference(zero))
        .difference(&HashSet::from([&e]))
        .next()
        .unwrap());

    let b = **HashSet::from_iter(four.difference(one))
        .difference(&HashSet::from([&d]))
        .next()
        .unwrap();

    let six = length_to_signal[&6]
        .iter()
        .find(|segment| {
            segment.symmetric_difference(nine).count() != 0
                && segment.symmetric_difference(zero).count() != 0
        })
        .unwrap();

    let f = *six
        .difference(&HashSet::from([a, e, g, d, b]))
        .next()
        .unwrap();

    let c = *one.difference(&HashSet::from([f])).next().unwrap();

    HashMap::from([
        (a, 'a'),
        (b, 'b'),
        (c, 'c'),
        (d, 'd'),
        (e, 'e'),
        (f, 'f'),
        (g, 'g'),
    ])
}

fn sort_string(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort();

    chars.iter().collect()
}
