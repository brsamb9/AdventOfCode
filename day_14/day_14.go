package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"regexp"
	"strconv"
	"strings"
)

var memory = make(map[int]int)

func main() {
	input, err := ioutil.ReadFile("day_14/input.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(input), "\n")
	// Map to contains memory locations and respected values - (defaulted to zero otherwise)
	var mask string
	re := regexp.MustCompile("mem\\[(\\d+)\\] = (\\d+)")

	for _, line := range lines {
		if strings.HasPrefix(line, "mask") {
			mask = strings.TrimSpace(strings.Split(line, "=")[1])
		} else if strings.HasPrefix(line, "mem") {
			// https://github.com/StefanSchroeder/Golang-Regex-Tutorial/blob/master/01-chapter2.markdown
			addressValue := re.FindAllStringSubmatch(line, -1)[0]
			address, _ := strconv.Atoi(addressValue[1])
			value, _ := strconv.Atoi(addressValue[2])

			memory[address] = applyMask(value, mask)
		}
	}

	fmt.Println("Part one: ", PartOne())
}

/*
mask = "X11001110001101XX01111X1001X01101111"
mem[32163] = 23587
*/
func PartOne() int {
	var sumMemory int = 0
	for _, mem := range memory {
		sumMemory += mem
	}
	return sumMemory
}

func applyMask(val int, mask string) int {
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

func setBit(val int, index int, bit int) int {
	modBit := int(math.Pow(2, float64(index)))

	if bit == 0 {
		return val &^ modBit
	}

	return val | modBit
}

func PartTwo() {

}
