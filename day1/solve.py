from pathlib import Path

Path("output.txt").write_text('')

lines = Path("input.txt").read_text().split('\n')
out = Path("output.txt")
og_print = print

def print(*args, **kwargs):
	og_print(*args, **kwargs, file=out.open('a'))

def solve():
	nums = [*map(int,lines)]
	print(sum(a<b for a, b in zip(nums, nums[3:])))

solve()
