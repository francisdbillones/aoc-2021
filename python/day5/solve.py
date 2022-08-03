from numpy import *
from itertools import *


def part1(data: list[tuple[tuple[int, int], tuple[int, int]]]) -> int:
    x1, y1, x2, y2 = zip(*((*p1, *p2) for p1, p2 in data))
    min_x, max_x = min(array((x1, x2))), max(array((x1, x2)))
    min_y, max_y = min(array((y1, y2))), max(array((y1, y2)))

    grid = zeros((max_y - min_y + 1, max_x - min_x + 1))

    for ((x1, y1), (x2, y2)) in data:
        xs, xl = sorted([x1, x2])
        ys, yl = sorted([y1, y2])
        if x1 == x2 or y1 == y2:
            grid[ys - min_y : yl - min_y + 1, xs - min_x : xl - min_x + 1] += 1
    return sum(grid >= 2)


def part2(data: list[list[int]]) -> int:
    x1, y1, x2, y2 = zip(*((*p1, *p2) for p1, p2 in data))
    min_x, max_x = min(array((x1, x2))), max(array((x1, x2)))
    min_y, max_y = min(array((y1, y2))), max(array((y1, y2)))

    grid = zeros((max_y - min_y + 1, max_x - min_x + 1))

    for ((x1, y1), (x2, y2)) in data:
        xs, xl = sorted([x1, x2])
        ys, yl = sorted([y1, y2])
        if x1 == x2 or y1 == y2:
            grid[ys - min_y : yl - min_y + 1, xs - min_x : xl - min_x + 1] += 1

        else:
            to_add = eye(yl - ys + 1, xl - xs + 1).astype(int)
            if x1 < x2 and y1 > y2:
                to_add = rot90(to_add)
            if x1 > x2 and y1 < y2:
                to_add = rot90(to_add)

            grid[ys - min_y : yl - min_y + 1, xs - min_x : xl - min_x + 1] += to_add
    return sum(grid >= 2)


def get_data(lines: list[str]):
    data = []
    for line in lines:
        a, b = line.split(" -> ")
        x1, y1 = map(int, a.split(","))
        x2, y2 = map(int, b.split(","))
        data.append(((x1, y1), (x2, y2)))
    return data


if __name__ == "__main__":
    with open("input.txt", "r") as file:
        lines = [*filter(bool, file.read().splitlines())]

    data = get_data(lines)
    with open("output.txt", "w") as file:
        part1_out = part1(data)
        part2_out = part2(data)
        print(part1_out)
        print(part2_out)
        file.write(f"Part 1: {part1_out}\n")
        file.write(f"Part 2: {part2_out}\n")
