import numpy as np

with open("../input.txt", "r") as f:
    con = f.readlines()
data = [i.strip() for i in con]
data = [i.split(" ") for i in data]
data = [[int(i) for i in row] for row in data]


def check(x):
    x = np.array(x)
    _diff = np.diff(x)
    if np.all(_diff > 0) and np.all(_diff <= 3):
        return True
    elif np.all(_diff < 0) and np.all(_diff >= -3):
        return True
    else:
        return False


res = 0
fixes = 0
for row in data:
    # Problem 1 = if block
    if check(row):
        res += 1
    # Problem 2 = else block
    else:
        for i, v in enumerate(row):
            rowcp = row.copy()
            rowcp.pop(i)
            if check(rowcp):
                fixes += 1
                break
print(f"Part1 : \n  Total = {res}")
print(f"Part2 : \n  Fixes = {fixes} \n  Total = {res+fixes}")

