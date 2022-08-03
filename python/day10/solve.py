with open("input.txt") as reader:
    lines = reader.read().splitlines()

import statistics

opening = "(", "[", "{", "<"
closing = ")", "]", "}", ">"

scores = [3, 57, 1197, 25137]


def part1(lines):
    ans = 0
    for line in lines:
        stack = []
        for char in line:
            if char in opening:
                stack.append(char)
            else:
                if closing.index(char) != opening.index(stack[-1]):
                    ans += scores[closing.index(char)]
                    break
                stack.pop()
    return ans


def not_corrupted(lines):
    res = []

    for line in lines:
        stack = []
        for char in line:
            if char in opening:
                stack.append(char)
            elif closing.index(char) != opening.index(stack[-1]):
                break
            else:
                stack.pop()
        else:
            res.append(line)
    return res


def part2(lines):
    scores = []
    lines = not_corrupted(lines)

    for line in lines:
        stack = []
        for char in line:
            if char in opening:
                stack.append(char)
            else:
                stack.pop()

        autocomplete = []
        for char in reversed(stack):
            autocomplete.append(closing[opening.index(char)])

        score = 0
        for char in autocomplete:
            score = score * 5 + (closing.index(char) + 1)
        scores.append(score)
    return statistics.median(scores)


print(part1(lines))
print(part2(lines))
