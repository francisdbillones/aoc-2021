use std::fs;

const B_SIZE: usize = 5;

struct Board {
    cells: Vec<Vec<u32>>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split_terminator(',')
        .map(|s| s.parse::<u32>().expect("Couldn't parse number"))
        .collect::<Vec<u32>>();

    let mut boards = Vec::<Board>::new();

    while let Some(_) = lines.next() {
        let board_lines = (0..B_SIZE)
            .map(|_| lines.next().unwrap())
            .collect::<Vec<&str>>();

        let board = board_lines
            .iter()
            .map(|s| {
                s.split_whitespace()
                    .map(|ss| ss.parse::<u32>().expect("Couldn't parse cell"))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        boards.push(Board { cells: board });
    }

    for num in numbers {
        for board in boards.iter_mut() {
            if let Some((i, j)) = find_n(&board, &num) {
                set_mbit(&mut board.cells[i][j], 1);
            }

            if board_win(board) {
                return num * calculate_score(board);
            }
        }
    }

    panic!("No winners");
}

fn part2(input: &str) -> u32 {
    let mut lines = input.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split_terminator(',')
        .map(|s| s.parse::<u32>().expect("Couldn't parse number"))
        .collect::<Vec<u32>>();

    let mut boards = Vec::<Board>::new();

    while let Some(_) = lines.next() {
        let board_lines = (0..B_SIZE)
            .map(|_| lines.next().unwrap())
            .collect::<Vec<&str>>();

        let board = board_lines
            .iter()
            .map(|s| {
                s.split_whitespace()
                    .map(|ss| ss.parse::<u32>().expect("Couldn't parse cell"))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        boards.push(Board { cells: board });
    }

    let board_count = boards.len();
    let mut winning_board = vec![false; boards.len()];
    let mut win_count = 0;

    for num in numbers {
        for (board_i, board) in boards.iter_mut().enumerate() {
            if winning_board[board_i] {
                continue;
            }

            if let Some((i, j)) = find_n(&board, &num) {
                set_mbit(&mut board.cells[i][j], 1);
            }

            if board_win(board) {
                if win_count != board_count - 1 {
                    winning_board[board_i] = true;
                    win_count += 1;
                } else {
                    let last_losing_board = &boards[(0..board_count)
                        .filter(|i| !winning_board[*i])
                        .next()
                        .unwrap()];

                    return num * calculate_score(last_losing_board);
                }
            }
        }
    }

    panic!("No winners");
}

fn board_win(board: &Board) -> bool {
    return board_win_row(board) || board_win_col(board) /*  || board_win_diag(board)*/;
}

fn board_win_row(board: &Board) -> bool {
    for i in 0..B_SIZE {
        let row = &board.cells[i];

        if row.iter().map(get_mbit).all(|b| b == 1) {
            return true;
        }
    }

    return false;
}

fn board_win_col(board: &Board) -> bool {
    for j in 0..B_SIZE {
        let col = board.cells.iter().map(|row| row[j]).collect::<Vec<u32>>();

        if col.iter().map(get_mbit).all(|b| b == 1) {
            return true;
        }
    }

    return false;
}

fn get_mbit(n: &u32) -> u32 {
    return (*n & (1 << 31)) >> 31;
}

fn set_mbit(n: &mut u32, bit: u32) {
    *n += bit << 31;
}

fn get_n(n: &u32) -> u32 {
    return (n << 1) >> 1;
}

fn find_n(board: &Board, n: &u32) -> Option<(usize, usize)> {
    for i in 0..B_SIZE {
        for j in 0..B_SIZE {
            if get_n(&board.cells[i][j]) == *n {
                return Some((i, j));
            }
        }
    }
    return None;
}

fn calculate_score(board: &Board) -> u32 {
    let mut score = 0;
    for i in 0..B_SIZE {
        for j in 0..B_SIZE {
            if get_mbit(&board.cells[i][j]) == 0 {
                score += get_n(&board.cells[i][j]);
            }
        }
    }

    return score;
}
