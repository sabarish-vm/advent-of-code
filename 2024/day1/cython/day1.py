import numpy as np
from day1 import problem1, problem2


def main():
    data = np.loadtxt("../input.txt", dtype=np.int32)
    col1 = np.sort(data[:, 0])
    col2 = np.sort(data[:, 1])

    problem1(col1, col2)
    problem2(col1, col2)


if __name__ == "__main__":
    main()
