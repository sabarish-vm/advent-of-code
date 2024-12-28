import numpy as np

data = np.loadtxt("../input.txt")

## Problem 1
col1 = np.sort(data[:, 0])
col2 = np.sort(data[:, 1])

print(f"Part1 = {np.sum(np.abs(col2-col1))}")

## Problem 2
res = 0


for v in data[:, 0]:
    res += np.sum(np.where(col2 == v, v, 0))
print(f"Part2 = {res}")

# Alternate way of doing it. This avoids an explicit for loop, might be
# advantageous if the for loop is longer
# def find(v):
#     return np.sum(np.where(col2 == v, v, 0))


# find_np = np.frompyfunc(find, 1, 1)
# res = find_np(col1)
