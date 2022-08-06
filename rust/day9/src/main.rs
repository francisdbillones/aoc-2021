use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mat: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    find_low_points(&mat).map(|(i, j)| mat[i][j] + 1).sum()
}

fn part2(input: &str) -> u32 {
    let mut mat: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let low_points: Vec<(usize, usize)> =
        find_low_points(&mat).map(|(i, j)| (i + 1, j + 1)).collect();
    dbg!(&low_points);

    pad(&mut mat, 9);

    // let basins = find_low_points(&mat).map(|low_point| find_basin(&mat, &low_point));
    let mut basin_sizes = low_points
        .iter()
        .map(|low_point| basin_size(&mat, low_point))
        .collect::<Vec<usize>>();
    basin_sizes.sort_unstable();

    basin_sizes.iter().rev().take(3).product::<usize>() as u32
}

fn find_low_points(mat: &Vec<Vec<u32>>) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..mat.len())
        .cartesian_product(0..mat[0].len())
        .filter(|(i, j)| {
            let i = *i;
            let j = *j;
            [(i as i32) - 1, i as i32, (i as i32) + 1]
                .iter()
                .filter(|n_i| 0 <= **n_i && **n_i < (mat.len() as i32))
                .map(|n_i| *n_i as usize)
                .all(|n_i| {
                    [(j as i32) - 1, j as i32, (j as i32) + 1]
                        .iter()
                        .filter(|n_j| 0 <= **n_j && **n_j < (mat[0].len() as i32))
                        .map(|n_j| *n_j as usize)
                        .all(|n_j| mat[i][j] <= mat[n_i][n_j])
                })
        })
}

// fn find_basin(mat: &Vec<Vec<u32>>, low_point: &(usize, usize)) -> Vec<(usize, usize)> {

// }

fn basin_size(mat: &[Vec<u32>], low_point: &(usize, usize)) -> usize {
    let mut basin: HashSet<(usize, usize)> = HashSet::new();

    flood_fill(mat, low_point, &mut basin);

    basin.len()
}

fn flood_fill(mat: &[Vec<u32>], point: &(usize, usize), basin: &mut HashSet<(usize, usize)>) {
    if !basin.insert(*point) {
        return;
    }

    surrounding_points(mat, point)
        .iter()
        .flatten()
        .filter(|(n_i, n_j)| mat[*n_i][*n_j] != 9)
        .for_each(|(n_i, n_j)| flood_fill(mat, &(*n_i, *n_j), basin))
}

fn surrounding_points(mat: &[Vec<u32>], point: &(usize, usize)) -> [Option<(usize, usize)>; 4] {
    let (i, j) = *point;
    let (i_i32, j_i32) = (i as i32, j as i32);

    [
        {
            if in_bounds(&(i_i32 - 1, j_i32), mat) && mat[i - 1][j] != 9 {
                Some((i - 1, j))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32 + 1, j_i32), mat) && mat[i + 1][j] != 9 {
                Some((i + 1, j))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32, j_i32 - 1), mat) && mat[i][j - 1] != 9 {
                Some((i, j - 1))
            } else {
                None
            }
        },
        {
            if in_bounds(&(i_i32, j_i32 + 1), mat) && mat[i][j + 1] != 9 {
                Some((i, j + 1))
            } else {
                None
            }
        },
    ]
}

fn in_bounds(point: &(i32, i32), mat: &[Vec<u32>]) -> bool {
    let size_i = mat.len() as i32;
    let size_j = mat[0].len() as i32;

    let (i, j) = *point;

    (0 <= i && i < size_i) && (0 <= j && j <= size_j)
}

// fn find_boundaries(mat: &Vec<Vec<u32>>, low_point: &(usize, usize)) -> Vec<Vec<u32>> {
//     // return boundaries as a 2D sparse vector, where
//     // the first axis represents i,
//     // while the u32 inside represents j
//     // i.e. if vec[1] returned [1, 2, 3], then there are
//     // boundary points at (1, 1), (1, 2), and (1, 3)

//     let boundaries: Vec<HashSet<u32>> = vec![HashSet::new(); mat.len()];

// }

fn walk_left(mat: &[Vec<u32>], point: &(usize, usize)) -> (usize, usize) {
    let (i, mut j) = *point;

    while mat[i][j - 1] != 9 {
        j -= 1;
    }

    (i, j)
}

fn walk_right(mat: &[Vec<u32>], point: &(usize, usize)) -> (usize, usize) {
    let (i, mut j) = *point;

    while mat[i][j + 1] != 9 {
        j += 1;
    }

    (i, j)
}

fn walk_up(mat: &[Vec<u32>], point: &(usize, usize)) -> (usize, usize) {
    let (mut i, j) = *point;

    while mat[i - 1][j] != 9 {
        i -= 1;
    }

    (i, j)
}

fn walk_down(mat: &[Vec<u32>], point: &(usize, usize)) -> (usize, usize) {
    let (mut i, j) = *point;

    while mat[i + 1][j] != 9 {
        i += 1;
    }

    (i, j)
}

fn find_basin_top(mat: &[Vec<u32>], point: &(usize, usize)) -> (usize, usize) {
    // assumes basin is convex

    let (mut i, mut j) = *point;

    loop {
        // walk left
        if mat[i][j - 1] != 9 {
            (i, j) = walk_left(mat, &(i, j));
        }

        // walk up
        if mat[i - 1][j] != 9 {
            (i, j) = walk_up(mat, &(i, j));
        } else {
            // step right until we can walk up
            loop {
                // step right if we can
                if mat[i][j + 1] != 9 {
                    j += 1;

                    // if we can now walk up, then walk up and break
                    if mat[i - 1][j] != 9 {
                        (i, j) = walk_up(mat, &(i, j));
                        break;
                    }
                }
                // if we can't step right then we've found the top
                else {
                    return walk_left(mat, &(i, j));
                };
            }
        }
    }
}

fn pad<T>(mat: &mut Vec<Vec<T>>, pad_value: T)
where
    T: Clone + Copy,
{
    let size_i = mat.len();
    let size_j = mat[0].len();

    let mut padded = vec![vec![pad_value; size_j + 2]; size_i + 2];

    for i in 0..size_i {
        for j in 0..size_j {
            padded[i + 1][j + 1] = mat[i][j];
        }
    }

    *mat = padded;
}
