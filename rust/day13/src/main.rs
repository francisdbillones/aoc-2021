use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    X(usize),
    Y(usize),
}

fn part1(input: &str) -> u32 {
    let (mut marks, instructions) = parse(input);

    for ins in instructions.iter().take(1) {
        let (height, width) = (marks.len(), marks[0].len());

        marks = match ins {
            Instruction::X(n) => fold_x(&marks, *n, height, width),
            Instruction::Y(n) => fold_y(&marks, *n, height, width),
        };
    }

    marks.iter().flatten().map(|&n| n as usize).sum::<usize>() as u32
}

fn part2(input: &str) -> u32 {
    let (mut marks, instructions) = parse(input);

    for ins in instructions.iter() {
        let (height, width) = (marks.len(), marks[0].len());

        marks = match ins {
            Instruction::X(n) => fold_x(&marks, *n, height, width),
            Instruction::Y(n) => fold_y(&marks, *n, height, width),
        };
    }

    for line in marks.iter() {
        let chars: String = line.iter().map(|&x| if x { '#' } else { '.' }).collect();

        println!("{}", chars);
    }

    0
}

fn fold_x(marks: &[Vec<bool>], n: usize, height: usize, width: usize) -> Vec<Vec<bool>> {
    let points: Vec<(usize, usize)> = (0..height)
        .cartesian_product(0..n)
        .filter(|&(i, j)| marks[i][j] || marks[i][width - j - 1])
        .collect();

    make_marks(&points, height, n)
}

fn fold_y(marks: &[Vec<bool>], n: usize, height: usize, width: usize) -> Vec<Vec<bool>> {
    let points: Vec<(usize, usize)> = (0..n)
        .cartesian_product(0..width)
        .filter(|&(i, j)| marks[i][j] || marks[height - i - 1][j])
        .collect();

    make_marks(&points, n, width)
}

fn make_marks(coords: &Vec<(usize, usize)>, height: usize, width: usize) -> Vec<Vec<bool>> {
    let mut marks: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for &(j, i) in coords {
        marks[j][i] = true;
    }

    marks
}

fn parse(input: &str) -> (Vec<Vec<bool>>, Vec<Instruction>) {
    let lines = input.lines();

    let mut coords: Vec<(usize, usize)> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    lines.for_each(|line| {
        if line.starts_with('f') {
            let line = line.chars().skip("fold along ".len()).collect::<String>();

            let (axis, n) = line.split_once('=').expect("Couldn't parse input");

            let n = n.parse::<usize>().expect("Couldn't parse input");

            let ins = match axis {
                "x" => Instruction::X(n),
                "y" => Instruction::Y(n),
                _ => panic!("Couldn't parse input"),
            };

            instructions.push(ins);
        } else if !line.is_empty() {
            let (x, y) = line.split_once(',').expect("Couldn't parse input");
            let x = x.parse::<usize>().expect("Couldn't parse input");
            let y = y.parse::<usize>().expect("Couldn't parse input");

            coords.push((y, x));
        }
    });

    let mat_width = instructions
        .iter()
        .map(|ins| match ins {
            Instruction::X(x) => *x,
            _ => 0,
        })
        .max()
        .unwrap()
        * 2
        + 1;

    let mat_height = instructions
        .iter()
        .map(|ins| match ins {
            Instruction::Y(y) => *y,
            _ => 0,
        })
        .max()
        .unwrap()
        * 2
        + 1;

    let marks = make_marks(&coords, mat_height, mat_width);

    (marks, instructions)
}
