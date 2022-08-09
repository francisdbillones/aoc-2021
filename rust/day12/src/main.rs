use std::{collections::HashMap, collections::HashSet, fs};

#[derive(Debug)]
struct Graph<'a> {
    nodes: HashMap<&'a str, bool>,
    edges: HashMap<&'a str, HashSet<&'a str>>,
}

impl Graph<'_> {
    fn get_adjacent_nodes(&self, node: &str) -> Option<Vec<&str>> {
        self.edges
            .get(node)
            .map(|node_edges| node_edges.iter().copied().collect())
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let graph = parse(input);

    paths(&graph, 0) as u32
}

fn part2(input: &str) -> u32 {
    let graph = parse(input);

    paths(&graph, 1) as u32
}

fn paths(graph: &Graph, small_counter: usize) -> usize {
    search("start", graph, &HashSet::new(), small_counter)
}

fn search<'a>(
    node: &'a str,
    graph: &Graph<'a>,
    seen: &HashSet<&'a str>,
    small_counter: usize,
) -> usize {
    if node == "end" {
        return 1;
    }

    if node == "start" && seen.len() > 1 {
        return 0;
    }

    let mut small_counter = small_counter;
    if !graph.nodes[node] && seen.contains(node) {
        if small_counter == 0 {
            return 0;
        } else {
            small_counter -= 1;
        }
    }

    let mut this_seen = seen.to_owned();
    this_seen.insert(node);

    graph
        .get_adjacent_nodes(node)
        .expect("Couldn't find node in graph")
        .iter()
        .map(|neighbour| search(neighbour, graph, &this_seen, small_counter))
        .sum()
}

fn parse(input: &str) -> Graph {
    let mut graph = Graph {
        nodes: HashMap::new(),
        edges: HashMap::new(),
    };

    for line in input.lines() {
        let (source, dest) = line.split_once('-').expect("Couldn't parse input");

        if !graph.nodes.contains_key(source) {
            graph.nodes.insert(
                source,
                source
                    .chars()
                    .next()
                    .expect("Couldn't parse input")
                    .is_uppercase(),
            );
        }

        if !graph.nodes.contains_key(dest) {
            graph.nodes.insert(
                dest,
                dest.chars()
                    .next()
                    .expect("Couldn't parse input")
                    .is_uppercase(),
            );
        }

        match graph.edges.get_mut(source) {
            Some(dest_paths) => {
                dest_paths.insert(dest);
            }
            None => {
                graph.edges.insert(source, HashSet::from([dest]));
            }
        }

        match graph.edges.get_mut(dest) {
            Some(dest_paths) => {
                dest_paths.insert(source);
            }
            None => {
                graph.edges.insert(dest, HashSet::from([source]));
            }
        }
    }

    graph
}
