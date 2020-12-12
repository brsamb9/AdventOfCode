// Unit testing influenced by https://www.reddit.com/r/golang/comments/kbohc7/episode_01_getting_started_with_unit_testing_in/
package main

import (
	"io/ioutil"
	"strings"
	"testing"
)

func TestPartTwo(t *testing.T) {
	var s string = "F10\nN3\nF7\nR90\nF11"
	ans := PartTwo(s)

	if ans != (214 + 72) {
		t.Fail()
	}
}

func TestPartOne_larger(t *testing.T) {
	// E: -3, -2, -3, -5, 30, 5,	N: -1, 81, -3, -44, -4
	// E 32; N 29
	var s string = "W3\nR180\nS1\nF2\nR90\nW3\nF81\nL270\nW5\nF30\nR90\nE5\nS3\nF44\nR180\nS4"

	ans := PartOne(s)
	if ans != (35 - 13 + 81 - 52) {
		t.Fail()
	}
}

func TestShipFacingChanges(t *testing.T) {
	facing := "E"

	facing = turnShip("R", 90, facing)
	if facing != "S" {
		t.Fail()
	}

	facing = turnShip("R", 180, facing)
	if facing != "N" {
		t.Fail()
	}

	facing = turnShip("L", 270, facing)
	if facing != "E" {
		t.Fail()
	}
}

func TestPartOne_input(t *testing.T) {
	// east 10, north 3, east 7, south 11
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(input), "\n")

	for _, instr := range lines {
		if instr == "" {
			panic("empty")
		}
	}
}

func TestPartOne_given(t *testing.T) {
	// east 10, north 3, east 7, south 11
	if PartOne(string("F10\nN3\nF7\nR90\nF11")) != 25 {
		t.Fail()
	}
}

func TestPartOne_small(t *testing.T) {
	// east 10 then west 7
	if PartOne(string("F10\nR180\nF7")) != 3 {
		t.Fail()
	}
}

func TestPartOne_small2(t *testing.T) {
	if PartOne(string("F10\nR180\nF7")) != 3 {
		t.Fail()
	}
}
