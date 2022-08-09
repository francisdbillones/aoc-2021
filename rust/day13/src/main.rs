use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[derive(Debug, Copy, Clone)]
enum Axis {
    X,
    Y,
}

fn part1(input: &str) -> u32 {
    let (mut marks, instructions) = parse(input);

    for &(axis, n) in instructions.iter().take(1) {
        marks = match axis {
            Axis::X => fold_x(&marks, n),
            Axis::Y => fold_y(&marks, n),
        };
    }

    marks.iter().flatten().map(|&n| n as usize).sum::<usize>() as u32
}

fn part2(input: &str) -> u32 {
    let (mut marks, instructions) = parse(input);

    for &(axis, n) in &instructions {
        marks = match axis {
            Axis::X => fold_x(&marks, n),
            Axis::Y => fold_y(&marks, n),
        };
    }

    for line in marks.iter() {
        let chars: String = line.iter().map(|&x| if x { '#' } else { '.' }).collect();

        println!("{}", chars);
    }

    marks.iter().flatten().map(|&n| n as usize).sum::<usize>() as u32
}

fn fold_x(marks: &[Vec<bool>], n: usize) -> Vec<Vec<bool>> {
    let (height, width) = (marks.len(), marks[0].len());

    let (new_height, new_width) = (height, (width - 1) / 2);

    let mut folded_marks = vec![vec![false; new_width]; new_height];

    for i in 0..new_height {
        for j in 0..new_width {
            folded_marks[i][j] = marks[i][j] || marks[i][width - j - 1];
        }
    }

    folded_marks
}

fn fold_y(marks: &[Vec<bool>], n: usize) -> Vec<Vec<bool>> {
    let (height, width) = (marks.len(), marks[0].len());

    let (new_height, new_width) = ((height - 1) / 2, width);

    let mut folded_marks = vec![vec![false; new_width]; new_height];

    for i in 0..new_height {
        for j in 0..new_width {
            folded_marks[i][j] = marks[i][j] || marks[height - i - 1][j];
        }
    }

    folded_marks
}

fn parse(input: &str) -> (Vec<Vec<bool>>, Vec<(Axis, usize)>) {
    let lines = input.lines();

    let coords: Vec<(usize, usize)> = lines
        .to_owned()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let (x, y) = s.split_once(',').expect("Couldn't parse input");

            let x = x.parse::<usize>().expect("Couldn't parse input");
            let y = y.parse::<usize>().expect("Couldn't parse input");

            (x, y)
        })
        .collect();

    let instructions: Vec<(Axis, usize)> = lines
        .skip(coords.len() + 1)
        .map(|s| {
            let s = s.chars().skip("fold along ".len()).collect::<String>();

            let (axis, n) = s.split_once('=').expect("Couldn't parse input");

            let axis = match axis {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("Couldn't parse input"),
            };

            let n = n.parse::<usize>().expect("Couldn't parse input");

            (axis, n)
        })
        .collect();

    let mat_width = coords.iter().map(|&(x, _)| x).max().unwrap() + 1;
    let mat_height = coords.iter().map(|&(_, y)| y).max().unwrap() + 1;

    let mut marks = vec![vec![false; mat_width]; mat_height];

    for (j, i) in coords {
        marks[i][j] = true;
    }

    (marks, instructions)
}
