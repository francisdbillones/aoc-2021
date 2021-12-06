from numpy import *
from collections import *

mat = concatenate((eye(9)[1:], eye(9)[0].reshape(1, -1)), axis=0).astype(uint64)
mat[6, 0] = 1

print((linalg.matrix_power(mat, 100_000 * 365).sum(axis=0) * (unique(concatenate((arange(9), [*map(int, open("i").read().split(","))])), return_counts=True)[1] - 1)).sum())

