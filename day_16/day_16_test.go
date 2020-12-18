package main

import (
	"fmt"
	"strings"
	"testing"
)

func TestPartOne(t *testing.T) {
	input := "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12"

	sections := strings.Split(input, "\n\n")
	ruleset := parsedRuleSet(strings.Split(sections[0], "\n"))
	nearbyTickets := parsedTickets(strings.Split(sections[2], ":")[1])

	p1, _ := partOne(ruleset, nearbyTickets)

	if p1 != 71 {
		fmt.Println("ans", p1)
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	input := "class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9"

	sections := strings.Split(input, "\n\n")
	ruleset := parsedRuleSet(strings.Split(sections[0], "\n"))
	nearbyTickets := parsedTickets(strings.Split(sections[2], ":")[1])

	_, validTickets := partOne(ruleset, nearbyTickets)

	myTicket := parsedTickets(strings.Split(sections[1], ":")[1])
	p2 := partTwo(ruleset, validTickets, myTicket)
	fmt.Println(p2)

	// Can't test with this input given as a test...
	// return answer of multiplication of the 6 departure numbers once the numbers are assigned
}
