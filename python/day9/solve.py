import numpy as np
import scipy.ndimage

with open("input.txt") as f:
    lines = f.read().splitlines()

heightmap = np.array([[int(x) for x in line] for line in lines])


def part1(heightmap: np.ndarray):
    heightmap_bordered = np.pad(heightmap, 1, "constant", constant_values=9)
    low_point_mask = (
        (heightmap < heightmap_bordered[2:, 1:-1])
        & (heightmap < heightmap_bordered[:-2, 1:-1])
        & (heightmap < heightmap_bordered[1:-1, 2:])
        & (heightmap < heightmap_bordered[1:-1, :-2])
    )

    return (heightmap[low_point_mask] + 1).sum()


def part2(heightmap: np.ndarray):
    labels, bins = scipy.ndimage.label(heightmap != 9)

    basins = sorted([(labels == bin).sum() for bin in range(1, bins + 1)])

    return basins[-1] * basins[-2] * basins[-3]


print(part1(heightmap))
print(part2(heightmap))
