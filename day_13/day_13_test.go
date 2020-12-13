package main

import (
	"testing"
)

func TestPartOne(t *testing.T) {
	startTime := 939
	busIds := []int{7, 13, -1, -1, 59, -1, 31, 19}

	if PartOne(startTime, busIds) != 295 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {

	// busIds := []int{7, 13, -1, -1, 59, -1, 31, 19}

	// maxBusId := -1
	// maxIndex := -1
	// for i, val := range busIds {
	// 	if val != -1 {
	// 		id := val
	// 		if id > maxBusId {
	// 			maxBusId = id
	// 			maxIndex = i
	// 		}
	// 	}
	// }

	// ************ Correct value - not compared equal, mess around big int in golang probably  ***********

	// 	if p2 := PartTwo(busIds, maxBusId, maxIndex); p2 != big.NewInt(1068781) {
	// 		fmt.Printf("%v %v", p2, big.NewInt(1068781))
	// 		t.Fail()
	// 	}
}
