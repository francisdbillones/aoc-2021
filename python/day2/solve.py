from pathlib import Path

Path("output.txt").write_text("")

lines = Path("input.txt").read_text().split("\n")
out = Path("output.txt")
og_print = print


def print(*args, **kwargs):
    og_print(*args, **kwargs, file=out.open("a"))


def part1():
    hor = 0
    dep = 0

    for line in lines:
        com, mag = line.split()
        mag = int(mag)
        if com == "forward":
            hor += mag
        elif com == "up":
            dep -= mag
        elif com == "down":
            dep += mag
    print(hor * dep)


def part2():
    aim = 0
    hor = 0
    dep = 0

    for line in lines:
        com, mag = line.split()
        mag = int(mag)

        if com == "forward":
            hor += mag
            dep += aim * mag
        elif com == "up":
            aim -= mag
        elif com == "down":
            aim += mag
    print(hor * dep)
    og_print(hor, dep)

part1()
part2()
