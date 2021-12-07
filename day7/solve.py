import numpy as np

with open("input.txt", "r") as file:
    nums = [int(x) for x in file.read().split(",")]
nums = np.array(nums)


def part1(nums: np.ndarray) -> int:
    return np.abs((nums - np.median(nums)).astype(int)).sum()


def part2(nums: np.ndarray) -> int:
    u1, u2 = np.floor(np.mean(nums) + [-0.5, 0.5])
    cost = lambda u: ((((nums - u) ** 2 + np.abs(nums - u)) // 2).sum())
    ans = min(u1, u2, key=cost)

    return cost(ans)


part1_out = part1(nums)
part2_out = part2(nums)

print(f"Part 1: {part1_out}")
print(f"Part 2: {part2_out}")
