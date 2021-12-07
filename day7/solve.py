import statistics

with open("input.txt", "r") as file:
    nums = [int(x) for x in file.read().split(",")]


def part1(nums: list[int]) -> int:
    min_cost = min(
        sum(abs(num - pos) for num in nums) for pos in range(min(nums), max(nums) + 1)
    )
    return min_cost


def part2(nums: list[int]) -> int:
    min_cost = min(
        sum(abs(num - pos) * (abs(num - pos) + 1) // 2 for num in nums)
        for pos in range(min(nums), max(nums) + 1)
    )
    return min_cost


part1_out = part1(nums)
part2_out = part2(nums)

print(f"Part 1: {part1_out}")
print(f"Part 2: {part2_out}")

