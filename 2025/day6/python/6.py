import numpy as np

filepath = "./input.txt"
with open(filepath, "r") as f:
    contents = f.readlines()

# Numerical data as array
arr = np.genfromtxt(filepath, skip_footer=1, dtype=np.int64)
# Operations per column
ops = np.array(contents[-1].split())
# Max digits in every column
sizes = (np.log10(arr.max(axis=0)) + 1).astype(int)

######### Problem 1
p1 = np.sum(np.where(ops == "+", np.sum(arr, axis=0), np.prod(arr, axis=0)))
print("P1 = ", p1)


######## Problem 2
start = 0
final = []
for i, v in enumerate(sizes):
    if ops[i] == "*":
        inner = np.ones(4)
    else:
        inner = np.zeros(4)
    for idx in range(v):
        str1 = ""
        for line in contents:
            if line[0] == "*" or line[0] == "+":
                continue
            str1 += line[start + idx]
        if str1.strip() != "":
            inner[idx] = int(str1)
    start += v + 1
    final.append(inner)

final = np.array(final, dtype=np.int64)
p2 = np.where(ops == "+", np.sum(final, axis=1), np.prod(final, axis=1)).sum()


print("P2 = ", p2)
