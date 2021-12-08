with open("input.txt", "r") as file:
    lines = file.read().splitlines()

# parse this: (example of a line in lines) "cdbga acbde eacdfbg adbgf gdebcf bcg decabf cg ebdgac egca | geac ceag faedcb cg"
# into this: [["cdbga", "abcde", ...], ["geac", "ceag", ...]]
signals, queries = zip(*(line.split(" | ") for line in lines))
signals = [signal.split() for signal in signals]
queries = [query.split() for query in queries]


# def solve(signal: list[str], query: list[str]) -> str:
num_to_signal = {
    0: "abcefg",
    1: "cf",
    2: "acdeg",
    3: "acdfg",
    4: "bcdf",
    5: "abdfg",
    6: "abdefg",
    7: "acf",
    8: "abcdefg",
    9: "abcdfg",
}
signal_to_num = {frozenset(s): n for n, s in num_to_signal.items()}
#     for num, mapping in num_mapping.items():
#         num_mapping[num] = set(mapping)

#     uniques = {2: 1, 4: 4, 3: 7, 7: 8}
#     possibs = {c: set("abcdefg") for c in "ijklmnop"}

#     for segment in signal + query:
#         if len(segment) in uniques:
#             cor_num = uniques[len(segment)]
#             mapping = num_mapping[cor_num]

#             for c in segment:
#                 possibs[chr(ord(c) + 8)] = mapping

#     breakpoint()
#     # loop while each value in num_mapping is not a char
#     while sum(map(len, num_mapping.values())) != len(num_mapping):
#         print(possibs)

#         for c, possib in possibs.items():
#             for other_c, other_possib in possibs.items():
#                 if possib == other_possib:
#                     continue
#                 if possib < other_possib:
#                     possibs[other_c] -= other_possib
#                     break
#     return possibs


def part1(signals: list[str], queries: list[str]) -> int:
    ans = 0
    uniques = {2: 1, 4: 4, 3: 7, 7: 8}
    for signal, query in zip(signals, queries):
        ans += sum(len(s) in uniques for s in query)
    return ans


def part2(signals: list[str], queries: list[str]) -> int:
    ans = sum(
        sum(
            signal_to_num[frozenset(solve_system(signal)[c] for c in segment)]
            * (10 ** (3 - i))
            for i, segment in enumerate(query)
        )
        for signal, query in zip(signals, queries)
    )

    return ans


def solve_system(signal: list[str]) -> dict[str, str]:
    length_to_signal = {len(s): [] for s in signal}
    for segment in signal:
        length_to_signal[len(segment)].append(set(segment))

    (eight,), (one,), (four,), (seven,) = (
        length_to_signal[7],
        length_to_signal[2],
        length_to_signal[4],
        length_to_signal[3],
    )

    a, = (*(seven - one),)

    nine, = [segment for segment in length_to_signal[6] if four < segment]

    e, = (*(eight - nine),)
    g, = (*(eight - four - set((a, e))),)

    zero, = (
        segment
        for segment in length_to_signal[6]
        if not (segment == nine or one & (nine ^ segment))
    )

    d, = (*(eight ^ zero) - set(e),)
    b, = (*(four - one - set(d)),)

    six, = (
        segment
        for segment in length_to_signal[6]
        if segment != nine and segment != zero
    )

    f, = (*(six - set((a, e, g, d, b))),)

    c, = (*(one - set(f)),)

    return {a: "a", b: "b", c: "c", d: "d", e: "e", f: "f", g: "g"}


def main():
    print(part1(signals, queries))
    print(part2(signals, queries))

if __name__ == "__main__":
    main()
