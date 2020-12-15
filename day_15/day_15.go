package main

import "fmt"

func main() {
	startingNumbers := []int{18, 11, 9, 0, 5, 1}
	p1 := playGame(startingNumbers, 2020)
	p2 := playGame(startingNumbers, 30000000)
	fmt.Println(p1)
	fmt.Println(p2)
}

func playGame(initialInput []int, upToTurn int) int {
	memoryGame := &MemGame{
		numsCount:  make(map[int]int),
		indexPairs: make(map[int][]int),
		turn:       1,
	}

	prevNum := initialInput[0]
	for i := 0; i < upToTurn; i++ {
		if i < len(initialInput) {
			memoryGame.nextNumber(prevNum)
			prevNum = initialInput[i]
		} else {
			prevNum = memoryGame.nextNumber(prevNum)
		}
		// fmt.Println("Turn ", memoryGame.turn, ":", memoryGame.numsCount, memoryGame.indexPairs, "->", prevNum)
		memoryGame.update(prevNum)
	}
	return prevNum
}

type MemGame struct {
	numsCount  map[int]int
	indexPairs map[int][]int
	turn       int
}

func (mG *MemGame) nextNumber(num int) int {
	var ans int
	if mG.numsCount[num] >= 2 {
		arr := mG.indexPairs[num]
		ans = arr[0] - arr[1]
	} else {
		ans = 0
	}
	return ans
}

func (mG *MemGame) update(num int) {
	mG.numsCount[num]++

	switch mG.numsCount[num] {
	case 1:
		mG.indexPairs[num] = []int{0, mG.turn}
	case 2:
		mG.indexPairs[num][0] = mG.turn
	default:
		mG.indexPairs[num][1] = mG.indexPairs[num][0]
		mG.indexPairs[num][0] = mG.turn
	}

	mG.turn++
}
