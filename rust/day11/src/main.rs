use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut mat = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    String::from(char)
                        .parse::<u32>()
                        .expect("Couldn't parse char")
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let flashes: u32 = (0..100).map(|_| update(&mut mat)).sum();

    flashes
}

fn part2(input: &str) -> u32 {
    let mut mat = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    String::from(char)
                        .parse::<u32>()
                        .expect("Couldn't parse char")
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut steps: u32 = 0;

    loop {
        if synchronized(&mat) {
            break;
        }

        update(&mut mat);
        steps += 1;
    }

    steps
}

fn update(mat: &mut Vec<Vec<u32>>) -> u32 {
    for cell in mat.iter_mut().flatten() {
        *cell += 1;
    }

    let i_size = mat.len();
    let j_size = mat[0].len();

    let mut flashed: Vec<Vec<bool>> = vec![vec![false; j_size]; i_size];
    let mut flashes: u32 = 0;

    loop {
        let mut some_flashed = false;

        for i in 0..i_size {
            for j in 0..j_size {
                if mat[i][j] <= 9 || flashed[i][j] {
                    continue;
                }
                some_flashed = true;
                flashes += 1;
                mat[i][j] = 0;
                flashed[i][j] = true;

                for &(n_i, n_j) in surrounding_points((i, j), &(i_size, j_size))
                    .iter()
                    .flatten()
                {
                    if !flashed[n_i][n_j] {
                        mat[n_i][n_j] += 1;
                    }
                }
            }
        }

        if !some_flashed {
            break;
        }
    }

    flashes
}

fn surrounding_points((i, j): (usize, usize), dim: &(usize, usize)) -> [Option<(usize, usize)>; 8] {
    let (i_i32, j_i32) = (i as i32, j as i32);

    [
        {
            if in_bounds(&(i_i32 - 1, j_i32 - 1), dim) {
                Some((i - 1, j - 1))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32 - 1, j_i32), dim) {
                Some((i - 1, j))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32 - 1, j_i32 + 1), dim) {
                Some((i - 1, j + 1))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32, j_i32 - 1), dim) {
                Some((i, j - 1))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32, j_i32 + 1), dim) {
                Some((i, j + 1))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32 + 1, j_i32 - 1), dim) {
                Some((i + 1, j - 1))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32 + 1, j_i32), dim) {
                Some((i + 1, j))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32 + 1, j_i32 + 1), dim) {
                Some((i + 1, j + 1))
            } else {
                None
            }
        },
    ]
}

fn synchronized(mat: &[Vec<u32>]) -> bool {
    let val = mat[0][0];

    (0..mat.len())
        .cartesian_product(0..mat[0].len())
        .all(|(i, j)| mat[i][j] == val)
}

fn in_bounds(&(i, j): &(i32, i32), &(i_size, j_size): &(usize, usize)) -> bool {
    (0 <= i && i < (i_size as i32)) && (0 <= j && j < (j_size as i32))
}
