package main

import (
	"fmt"
	"os"
)

func swapper(N int, characters []byte, sv []byte, checkID int, charID int) {
	elem := sv[charID]
	reset := false
	for j := checkID; j < N; j++ {
		elem2 := characters[j]
		if elem > elem2 && !reset {
			characters[j] = elem
			reset = true
			continue
		}
		if reset {
			characters[j] = '0'
		}
	}
}

func problem(N int, lines []byte, res *int64) error {
	start := 0
	for start < len(lines) {
		// find next newline
		end := start
		for end < len(lines) && lines[end] != '\n' {
			end++
		}
		line := lines[start:end]
		start = end + 1

		if len(line) == 0 {
			continue
		}

		// characters = array filled with '0'
		characters := make([]byte, N)
		for i := range N {
			characters[i] = '0'
		}

		it := 0
		for it < len(line) {
			rem := len(line) - it
			checkFrom := 0
			if rem <= N {
				checkFrom = N - rem
			}
			swapper(N, characters, line, checkFrom, it)
			it++
		}

		// accumulate value
		var val int64 = 0
		for _, elem := range characters {
			num := int64(elem - '0')
			val = 10*val + num
		}
		*res += val
	}
	return nil
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("No filepath provided")
		return
	}
	filepath := os.Args[1]

	_, err := os.Stat(filepath)
	if err != nil {
		fmt.Printf("File does not exist: %v\n", err)
		return
	}

	data, err := os.ReadFile(filepath)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	var res int64 = 0
	problem(2, data, &res)
	fmt.Printf("%d\n", res)

	var res2 int64 = 0
	problem(12, data, &res2)
	fmt.Printf("%d\n", res2)
}
