use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

struct Point {
    x: usize,
    y: usize,
}

struct Line {
    start: Point,
    stop: Point,
}

impl Line {
    fn draw(&self, grid: &mut Vec<Vec<u32>>, check_diagonal: bool) {
        if self.is_horizontal() {
            let y = self.start.y;
            for x in self.start.x..self.stop.x + 1 {
                grid[y][x] += 1;
            }
        }

        if self.is_vertical() {
            let x = self.start.x;
            for y in self.start.y..self.stop.y + 1 {
                grid[y][x] += 1;
            }
        }

        if check_diagonal && self.is_diagonal() {
            // if start x and stop y are the same, then so is start y and stop x
            let rev_x: bool = self.start.x > self.stop.x;
            let rev_y: bool = self.start.y > self.stop.y;

            for (x, y) in itertools::zip(
                {
                    if !rev_x {
                        itertools::Either::Left(self.start.x..=self.stop.x)
                    } else {
                        itertools::Either::Right((self.stop.x..=self.start.x).rev())
                    }
                },
                {
                    if !rev_y {
                        itertools::Either::Left(self.start.y..=self.stop.y)
                    } else {
                        itertools::Either::Right((self.stop.y..=self.start.y).rev())
                    }
                },
            ) {
                grid[y][x] += 1;
            }
        }
    }

    fn is_diagonal(&self) -> bool {
        return (!self.is_horizontal()) && (!self.is_vertical());
    }

    fn is_horizontal(&self) -> bool {
        return self.start.y == self.stop.y;
    }

    fn is_vertical(&self) -> bool {
        return self.start.x == self.stop.x;
    }
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
