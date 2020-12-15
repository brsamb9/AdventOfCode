package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"regexp"
	"strconv"
	"strings"
)

var memoryP1 = make(map[int]int)
var memoryP2 = make(map[int]int)

func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(input), "\n")

	// https://github.com/StefanSchroeder/Golang-Regex-Tutorial/blob/master/01-chapter2.markdown
	re := regexp.MustCompile("mem\\[(\\d+)\\] = (\\d+)")

	var mask string
	for _, line := range lines {
		if strings.HasPrefix(line, "mask") {
			mask = strings.TrimSpace(strings.Split(line, "=")[1])
		} else if strings.HasPrefix(line, "mem") {
			addressValue := re.FindAllStringSubmatch(line, -1)[0]
			address, _ := strconv.Atoi(addressValue[1])
			value, _ := strconv.Atoi(addressValue[2])

			memoryP1[address] = applyMaskP1(value, mask)

			address2 := applyMaskP2(address, mask)
			for _, ad := range address2 {
				memoryP2[ad] = value
			}
		}
	}

	fmt.Println("Part one: ", PartOne())
	fmt.Println("Part one: ", PartTwo())

}

func PartOne() int {
	var sumMemory int = 0
	for _, mem := range memoryP1 {
		sumMemory += mem
	}
	return sumMemory
}

func PartTwo() int {
	var sumMemory int = 0
	for _, mem := range memoryP2 {
		sumMemory += mem
	}
	return sumMemory
}

func applyMaskP1(val int, mask string) int {
	l := len(mask) - 1
	for i := 0; i < len(mask); i++ {
		bit := mask[l-i]

		switch bit {
		case '1':
			val = setBit(val, i, 1)
		case '0':
			val = setBit(val, i, 0)
		}
	}

	return val
}

func applyMaskP2(val int, mask string) []int {
	floating := []int{}

	l := len(mask) - 1
	for i := 0; i < len(mask); i++ {
		bit := mask[l-i]

		switch bit {
		case '1':
			val = setBit(val, i, 1)
		case 'X':
			floating = append(floating, i)
		}
		// case '0' - unchanged
	}

	valuesLength := int(math.Pow(2, float64(len(floating))))
	values := make([]int, valuesLength)

	for i := 0; i < valuesLength; i++ {
		for j, floatIndex := range floating {
			bit := 1
			if i&int(math.Pow(2, float64(j))) == 0 {
				bit = 0
			}

			val = setBit(val, floatIndex, bit)
		}

		values[i] = val
	}

	return values
}

func setBit(val int, index int, bit int) int {
	modBit := int(math.Pow(2, float64(index)))

	if bit == 0 {
		return val &^ modBit
	}

	return val | modBit
}
