// Forgot to make a test for yesterday!
package main

import (
	"fmt"
	"testing"
)

func TestPartOne(t *testing.T) {
	test1 := playGame([]int{0, 3, 6}, 10)
	if test1 != 0 {
		t.Fail()
	}

	test2 := playGame([]int{0, 3, 6}, 2020)
	if test2 != 436 {
		fmt.Println(test2)
		t.Fail()
	}

	test3 := playGame([]int{1, 3, 2}, 2020)
	if test3 != 1 {
		fmt.Println(test3)
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	t1 := playGame([]int{0, 3, 6}, 30000000)
	if t1 != 175594 {
		fmt.Println(t1)
		t.Fail()
	}
}
