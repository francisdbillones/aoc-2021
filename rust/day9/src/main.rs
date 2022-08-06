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

    pad(&mut mat, 9);

    // let basins = find_low_points(&mat).map(|low_point| find_basin(&mat, &low_point));
    let mut basins = low_points
        .iter()
        .map(|low_point| find_basin(&mat, low_point))
        .collect::<Vec<HashSet<(usize, usize)>>>();
    basins.sort_unstable_by_key(|basin| basin.len());

    basins
        .iter()
        .rev()
        .take(3)
        .map(|basin| basin.len())
        .product::<usize>() as u32
}

fn find_low_points(mat: &Vec<Vec<u32>>) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..mat.len())
        .cartesian_product(0..mat[0].len())
        .filter(|(i, j)| {
            surrounding_points(mat, &(*i, *j))
                .iter()
                .flatten()
                .map(|(n_i, n_j)| (*n_i, *n_j))
                .all(|(n_i, n_j)| mat[*i][*j] <= mat[n_i][n_j])
        })
}

fn find_basin(mat: &[Vec<u32>], low_point: &(usize, usize)) -> HashSet<(usize, usize)> {
    let mut basin: HashSet<(usize, usize)> = HashSet::new();

    flood_fill(mat, low_point, &mut basin);

    basin
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
