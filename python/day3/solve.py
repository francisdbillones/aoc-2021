from numpy import *


def part1(data: ndarray) -> int:
    gamma = mean(data, axis=0, keepdims=True) >= 0.5
    epsilon = 1 ^ gamma

    gamma = sum(2 ** arange(len(gamma[0]) - 1, -1, -1) * gamma)
    epsilon = sum(2 ** arange(len(epsilon[0]) - 1, -1, -1) * epsilon)

    return gamma * epsilon


def part2(data: ndarray) -> int:
    o2_ans = o2(data, 0)
    o2_ans = sum(2 ** arange(len(o2_ans) - 1, -1, -1) * o2_ans)
    co2_ans = co2(data, 0)
    co2_ans = sum(2 ** arange(len(co2_ans) - 1, -1, -1) * co2_ans)
    print(o2_ans, co2_ans)
    return o2_ans * co2_ans


def o2(bytes: ndarray, i: int) -> list[int]:
    most_common = mean(bytes.T[i]) >= 0.5

    filtered = bytes[argwhere(bytes.T[i] == most_common).flatten()]

    if len(filtered) == 1:
        return filtered[0]
    else:
        return o2(filtered, i + 1)


def co2(bytes: ndarray, i: int):
    least_common = mean(bytes.T[i]) < 0.5

    filtered = bytes[argwhere(bytes.T[i] == least_common).flatten()]

    if len(filtered) == 1:
        return filtered[0]
    else:
        return co2(filtered, i + 1)


def get_data(lines: list[str]):
    return array([[int(c) for c in line] for line in lines])


if __name__ == "__main__":
    with open("input.txt", "r") as file:
        lines = [*filter(bool, file.read().splitlines())]

    data = get_data(lines)

    with open("output.txt", "w") as file:
        file.write(f"Part 1: {part1(data)}\n")
        file.write(f"Part 2: {part2(data)}\n")
