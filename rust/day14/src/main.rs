#![feature(linked_list_cursors)]

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

type Rules = HashMap<(char, char), char>;

struct Polymer {
    pairs: HashMap<(char, char), usize>,
    counts: HashMap<char, usize>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (mut template, rules) = parse(input);

    (0..10).for_each(|_| apply_rules(&mut template, &rules));

    template.counts.values().max().unwrap() - template.counts.values().min().unwrap()
}

fn part2(input: &str) -> usize {
    let (mut template, rules) = parse(input);

    (0..40).for_each(|_| apply_rules(&mut template, &rules));

    template.counts.values().max().unwrap() - template.counts.values().min().unwrap()
}

fn apply_rules(template: &mut Polymer, rules: &Rules) {
    let mut updated_pairs: HashMap<(char, char), usize> = HashMap::new();

    for (pair, count) in template.pairs.iter() {
        let insert = rules.get(pair).unwrap().to_owned();

        let &(a, b) = pair;

        *updated_pairs.entry((a, insert)).or_default() += count;
        *updated_pairs.entry((insert, b)).or_default() += count;
        *template.counts.entry(insert).or_default() += count;
    }

    template.pairs = updated_pairs;
}

fn parse(input: &str) -> (Polymer, Rules) {
    let mut lines = input.lines();

    let template_string = lines.next().unwrap().to_owned();
    let mut template = Polymer {
        pairs: HashMap::new(),
        counts: HashMap::new(),
    };

    for (a, b) in template_string.chars().tuple_windows() {
        *template.pairs.entry((a, b)).or_default() += 1;
    }

    template.counts = template_string.chars().counts();

    lines.next();

    let rules: Rules = lines
        .map(|s| {
            let (pair, insert) = s.split_once(" -> ").expect("Couldn't parse input");

            let pair = pair.chars().take(2).collect::<Vec<char>>();

            let pair = (pair[0], pair[1]);
            let insert = insert.chars().next().unwrap();

            (pair, insert)
        })
        .collect();

    (template, rules)
}
