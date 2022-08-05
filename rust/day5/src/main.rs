mod models;
use models::line::{Line, Point};

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let lines: Vec<Line> = input
        .lines()
        .map(parse_line)
        .filter(|line| !line.is_diagonal())
        .collect();

    let grid_width = lines.iter().map(|line| line.stop.x).max().unwrap() + 1;

    let grid_height = lines.iter().map(|line| line.stop.y).max().unwrap() + 1;

    let mut grid = vec![vec![0 as u32; grid_width as usize]; grid_height as usize];

    for line in lines.iter() {
        line.draw(&mut grid, false);
    }

    let mut overlap = 0;

    for row in grid.iter() {
        for col in row.iter() {
            if *col >= 2 {
                overlap += 1;
            }
        }
    }

    return overlap;
}

fn part2(input: &str) -> u32 {
    let lines: Vec<Line> = input.lines().map(parse_line).collect();

    let grid_width = lines.iter().map(|line| line.stop.x).max().unwrap() + 1;

    let grid_height = lines.iter().map(|line| line.stop.y).max().unwrap() + 1;

    let mut grid = vec![vec![0 as u32; grid_width as usize]; grid_height as usize];

    for line in lines.iter() {
        line.draw(&mut grid, true);
    }

    let mut overlap = 0;

    for row in grid.iter() {
        for col in row.iter() {
            if *col >= 2 {
                overlap += 1;
            }
        }
    }

    return overlap;
}

fn parse_line(line: &str) -> Line {
    let (start, stop) = line.split_once(" -> ").unwrap();

    let start = start.split_once(',').unwrap();
    let stop = stop.split_once(',').unwrap();

    let mut start = Point {
        x: start.0.parse::<usize>().unwrap(),
        y: start.1.parse::<usize>().unwrap(),
    };

    let mut stop = Point {
        x: stop.0.parse::<usize>().unwrap(),
        y: stop.1.parse::<usize>().unwrap(),
    };

    if start.x == stop.x {
        let (min, max) = (
            std::cmp::min(start.y, stop.y),
            std::cmp::max(start.y, stop.y),
        );

        start.y = min;
        stop.y = max;
    } else if start.y == stop.y {
        let (min, max) = (
            std::cmp::min(start.x, stop.x),
            std::cmp::max(start.x, stop.x),
        );

        start.x = min;
        stop.x = max;
    }

    return Line {
        start: start,
        stop: stop,
    };
}
