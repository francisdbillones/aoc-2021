use std::collections::BinaryHeap;
use std::{cmp::Ordering, fs};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: usize,
    i: usize,
    j: usize,
}

impl Node {
    fn get_adjacent_nodes(
        &self,
        mat: &[Vec<usize>],
        height: usize,
        width: usize,
    ) -> [Option<Node>; 4] {
        let (i, j) = (self.i, self.j);
        let (i_isize, j_isize) = (i as isize, j as isize);

        [
            {
                if in_bounds(&(i_isize - 1, j_isize), height, width) {
                    Some(Node {
                        cost: self.cost + mat[i - 1][j],
                        i: (i - 1),
                        j,
                    })
                } else {
                    None
                }
            },
            {
                if in_bounds(&(i_isize + 1, j_isize), height, width) {
                    Some(Node {
                        cost: self.cost + mat[i + 1][j],
                        i: i + 1,
                        j,
                    })
                } else {
                    None
                }
            },
            {
                if in_bounds(&(i_isize, j_isize - 1), height, width) {
                    Some(Node {
                        cost: self.cost + mat[i][j - 1],
                        i,
                        j: j - 1,
                    })
                } else {
                    None
                }
            },
            {
                if in_bounds(&(i_isize, j_isize + 1), height, width) {
                    Some(Node {
                        cost: self.cost + mat[i][j + 1],
                        i,
                        j: j + 1,
                    })
                } else {
                    None
                }
            },
        ]
    }

    fn is_goal(&self, mat: &[Vec<usize>]) -> bool {
        let (i, j) = (self.i, self.j);

        (i == mat.len() - 1) && (j == mat[0].len() - 1)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.i.cmp(&self.i))
            .then_with(|| other.j.cmp(&self.j))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mat = parse(input);

    shortest_path(&mat, mat.len(), mat[0].len()).unwrap()
}

fn part2(input: &str) -> usize {
    let mat = parse(input);

    let mat = expand(&mat);

    shortest_path(&mat, mat.len(), mat[0].len()).unwrap()
}

fn expand(mat: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let (height, width) = (mat.len(), mat[0].len());

    let mut expanded = vec![vec![0; width * 5]; height * 5];

    for t_i in 0..5 {
        for t_j in 0..5 {
            for i in 0..height {
                for j in 0..width {
                    expanded[t_i * height + i][t_j * width + j] = wrap(mat[i][j] + t_i + t_j);
                }
            }
        }
    }

    expanded
}

fn wrap(mut n: usize) -> usize {
    if n > 9 {
        n -= 9;
    }

    n
}

fn shortest_path(mat: &[Vec<usize>], height: usize, width: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = vec![vec![usize::MAX; width]; height];

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[0][0] = 0;
    heap.push(Node {
        cost: 0,
        i: 0,
        j: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(node) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if node.is_goal(mat) {
            return Some(node.cost);
        }

        // Important as we may have already found a better way
        if node.cost > dist[node.i][node.j] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for neighbour in node.get_adjacent_nodes(mat, height, width) {
            if let Some(neighbour) = neighbour {
                // If so, add it to the frontier and continue
                if neighbour.cost < dist[neighbour.i][neighbour.j] {
                    heap.push(neighbour);
                    // Relaxation, we have now found a better way
                    dist[neighbour.i][neighbour.j] = neighbour.cost;
                }
            }
        }
    }

    // Goal not reachable
    None
}

fn in_bounds(&(i, j): &(isize, isize), height: usize, width: usize) -> bool {
    let i_size = height as isize;
    let j_size = width as isize;

    (0 <= i && i < i_size) && (0 <= j && j < j_size)
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    let mat: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_string()
                        .parse::<usize>()
                        .expect("Couldn't parse input")
                })
                .collect()
        })
        .collect();

    mat
}
