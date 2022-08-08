use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(illegal_line)
        .map(|illegal_char| match illegal_char {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Found invalid character returned as illegal character"),
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut scores = input
        .lines()
        .filter(|line| illegal_line(line).is_none())
        .map(|line| {
            autocomplete_line(line)
                .iter()
                .map(|c| match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("Found invalid character in line autocomplete"),
                })
                .fold(0, |acc, score| acc * 5 + score)
        })
        .collect::<Vec<u64>>();

    scores.sort();

    scores[scores.len() / 2] as u32
}

fn illegal_line(line: &str) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();

    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' | ']' | '}' | '>' => {
                if match char {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    _ => panic!("Invalid character"),
                } != stack[stack.len() - 1]
                {
                    return Some(char);
                } else {
                    stack.pop();
                }
            }
            _ => panic!("Invalid character"),
        }
    }
    None
}

fn autocomplete_line(line: &str) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();

    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => {
                stack.push(char);
            }
            ')' | ']' | '}' | '>' => {
                stack.pop();
            }
            _ => panic!("Invalid character"),
        }
    }

    stack
        .iter()
        .rev()
        .map(|char| match char {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Found invalid character in stack"),
        })
        .collect::<Vec<char>>()
}
