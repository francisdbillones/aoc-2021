from pathlib import Path

Path("output.txt").write_text('')

lines = Path("input.txt").read_text().split('\n')
out = Path("output.txt")
og_print = print

def print(*args, **kwargs):
	og_print(*args, **kwargs, file=out.open('a'))

def solve():
	res = 0
	nums = list(map(int, lines))
	last = float("inf")

	for a, b, c in zip(nums, nums[1:], nums[2:]):
		if a + b + c > last:
			res += 1
		last = a + b + c
	print(res)

solve()
