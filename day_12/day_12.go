package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

//  part one answer
func PartOne(instructions string) int {
	lines := strings.Split(instructions, "\n")

	state := make(map[string]int)
	for _, d := range []string{"N", "S", "W", "E"} {
		state[d] = 0
	}

	facing := "E"

	for _, instr := range lines {
		val, _ := strconv.Atoi(instr[1:])

		switch dir := instr[:1]; {
		case dir == "L" || dir == "R":
			facing = turnShip(dir, val, facing)
		case dir == "F":
			state[facing] += val
		default: // N,E,W,S
			state[dir] += val
		}
	}

	fmt.Println(state)
	upDown := state["N"] - state["S"]
	leftRight := state["W"] - state["E"]
	if upDown < 0 {
		upDown *= -1
	}
	if leftRight < 0 {
		leftRight *= -1
	}

	return upDown + leftRight
}

func PartTwo(instructions string) int {
	n := 0
	e := 0

	wpN := 1
	wpE := 10

	lines := strings.Split(instructions, "\n")
	for _, instr := range lines {
		val, _ := strconv.Atoi(instr[1:])

		switch instr[:1] {
		case "N":
			wpN += val
		case "S":
			wpN -= val
		case "E":
			wpE += val
		case "W":
			wpE -= val
		case "L":
			nRot := val / 90
			for i := 0; i < nRot; i++ {
				lstN := wpN
				lstE := wpE
				wpN = lstE
				wpE = -lstN
			}
		case "R":
			nRot := val / 90
			for i := 0; i < nRot; i++ {
				lstN := wpN
				lstE := wpE
				wpN = -lstE
				wpE = lstN
			}
		case "F":
			n += wpN * val
			e += wpE * val
		default:
			panic("Letter not expected")
		}
	}

	if n < 0 {
		n *= -1
	}
	if e < 0 {
		e *= -1
	}
	return n + e
}

func turnShip(turnDirection string, degreeTurn int, facing string) string {

	if degreeTurn%90 != 0 {
		panic("Naively expected a certain range of degree turns")
	}

	degreeIndexTurn := (degreeTurn / 90) % 4

	arrayDirections := []string{"N", "E", "S", "W"} // clockwise in left to right indexing

	// not a fan of this
	currIndex := 0
	switch facing {
	case "N":
		currIndex = 0
	case "E":
		currIndex = 1
	case "S":
		currIndex = 2
	case "W":
		currIndex = 3
	default:
		panic("Not expected")
	}

	if turnDirection == "R" {
		currIndex = (currIndex + degreeIndexTurn) % 4
	} else { // going left
		i := currIndex + 4 - degreeIndexTurn
		if i < 0 {
			i *= -1
		}
		currIndex = i % 4
	}

	return arrayDirections[currIndex]
}

func main() {
	input, err := ioutil.ReadFile("day_12/input.txt")
	if err != nil {
		panic(err)
	}

	p1 := PartOne(string(input))
	fmt.Println("Answer for part one: ", p1)

	p2 := PartTwo(string(input))
	fmt.Println("Answer for part two: ", p2)
}
