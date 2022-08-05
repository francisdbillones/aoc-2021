use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut state: [u64; 9] = [0; 9];

    for fish in input.split_terminator(',') {
        let fish_state = fish.parse::<u64>().expect("Couldn't parse initial state");
        state[fish_state as usize] += 1;
    }

    for _ in 0..80 {
        update_state(&mut state);
    }

    return state.iter().sum();
}

fn part2(input: &str) -> u64 {
    let mut state: [u64; 9] = [0; 9];

    for fish in input.split_terminator(',') {
        let fish_state = fish.parse::<u64>().expect("Couldn't parse initial state");
        state[fish_state as usize] += 1;
    }

    for _ in 0..256 {
        update_state(&mut state);
    }

    return state.iter().sum();
}

fn update_state(state: &mut [u64; 9]) {
    *state = [
        state[1],
        state[2],
        state[3],
        state[4],
        state[5],
        state[6],
        state[7] + state[0],
        state[8],
        state[0],
    ]
}
