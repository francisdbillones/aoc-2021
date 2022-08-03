import numpy as np
from helper import infinite_range

with open("input.txt", "r") as f:
    data = f.read().splitlines()

nums = np.array([[int(c) for c in l] for l in data]).astype(float)

def part1(nums: np.ndarray, steps: int):
    nums = nums.copy()
    bordered_nums = np.pad(nums, (1, 1), "constant", constant_values=-np.inf)
    flashes = 0
    for _ in range(steps):
        nums += 1
        bordered_nums += 1
        while (mask := (
            (bordered_nums[:-2, 1:-1] > 9).astype(int)
            + (bordered_nums[1:-1, :-2] > 9).astype(int)
            + (bordered_nums[2:, 1:-1] > 9).astype(int)
            + (bordered_nums[1:-1, 2:] > 9).astype(int)
            + (bordered_nums[:-2, :-2] > 9).astype(int)
            + (bordered_nums[2:, 2:] > 9).astype(int)
            + (bordered_nums[:-2, 2:] > 9).astype(int)
            + (bordered_nums[2:, :-2] > 9).astype(int)
        )).any():
            nums[nums > 9] = -np.inf
            nums += mask
            bordered_nums[1:-1, 1:-1] = nums

        flashes += (nums == -np.inf).sum()
        nums[nums == -np.inf] = 0
    return flashes

def part2(nums: np.ndarray):
    nums = nums.copy()
    bordered_nums = np.pad(nums, (1, 1), "constant", constant_values=-np.inf)
    for i in infinite_range():
        nums += 1
        bordered_nums += 1
        while (mask := (
            (bordered_nums[:-2, 1:-1] > 9).astype(int)
            + (bordered_nums[1:-1, :-2] > 9).astype(int)
            + (bordered_nums[2:, 1:-1] > 9).astype(int)
            + (bordered_nums[1:-1, 2:] > 9).astype(int)
            + (bordered_nums[:-2, :-2] > 9).astype(int)
            + (bordered_nums[2:, 2:] > 9).astype(int)
            + (bordered_nums[:-2, 2:] > 9).astype(int)
            + (bordered_nums[2:, :-2] > 9).astype(int)
        )).any():
            nums[nums > 9] = -np.inf
            nums += mask
            bordered_nums[1:-1, 1:-1] = nums

        if (nums == -np.inf).all():
            break

        nums[nums == -np.inf] = 0
    return i

if __name__ == "__main__":
    print(f"Part 1: {part1(nums, 100)}")
    print(f"Part 2: {part2(nums)}")
