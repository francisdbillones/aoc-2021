from numpy import *

def part1_and_2(nums: ndarray, boards: ndarray, i=None, marks=None):
    if marks is None:
        marks = zeros_like(boards)
    marks += boards == nums[(i := i or 0)]

    if boards.shape[0] == 0:
        return

    wbi = any(all(marks, axis=1) | all(marks, axis=2), axis=1)
    if any(wbi):
        yield boards[wbi], sum(nums[i] * (1 ^ marks[wbi]) * boards[wbi])
    yield from part1_and_2(nums, delete(boards, wbi, axis=0), i=i+1, marks=delete(marks, wbi, axis=0))


def get_data(lines: list[str]):
    nums = array([*map(int, lines[0].split(","))])

    boards = get_boards(lines)

    return nums, boards


def get_boards(lines: list[str]):
    boards = []

    cur_board = []
    for line in lines[1:]:
        if len(cur_board) != 5:
            cur_board.append([*map(int, line.split())])
        else:
            boards.append(cur_board)
            cur_board = [[*map(int, line.split())]]
    boards.append(cur_board)

    return array(boards)


def get_data(lines: list[str]):
    nums = array([*map(int, lines[0].split(","))])

    boards = get_boards(lines)

    return nums, boards

if __name__ == "__main__":
    with open("input.txt", "r") as file:
        lines = [*filter(bool, file.read().splitlines())]

    nums, boards = get_data(lines)

    with open("output.txt", "w") as file:
        _, part1_out = next(part1_and_2(nums, boards))
        for _, part2_out in part1_and_2(nums, boards):
            pass
        print(part1_out)
        print(part2_out)
        file.write(f"Part 1: {part1_out}\n")
        file.write(f"Part 2: {part2_out}\n")
