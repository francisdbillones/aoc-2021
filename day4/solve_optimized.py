from numpy import *


def part1_and_2(nums: ndarray, boards: ndarray, i=0):
    costs = {cost(board, nums): board for board in boards}
    c1, num1 = min(costs, key=lambda cost: cost[0])
    c2, num2 = max(costs, key=lambda cost: cost[0])

    part1, part2 = costs[c1, num1], costs[c2, num2]

    return sum(num1 * (1 ^ (part1 >> 8)) * part1), sum(num2 * (1 ^ (part2 >> 8)) * part2)
    

def cost(board: ndarray, nums: ndarray):
    c = 0
    for num in nums:
        c += 1
        board += (1 << 8) * (board == num)
        if any(all(board >> 8, axis=0) | all(board >> 8, axis=1), axis=0):
            return c, num


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

    part1_out, part2_out = part1_and_2(nums, boards)
    print(part1_out, part2_out)
