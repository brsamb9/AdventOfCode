package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"math/big"
	"strconv"
	"strings"
)

func main() {
	input, err := ioutil.ReadFile("day_13/input.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(input), "\n")

	// Parse data
	startTime, _ := strconv.Atoi(lines[0])
	busIds := []int{}
	maxBusId := -1
	maxIndex := -1
	for i, val := range strings.Split(lines[1], ",") {
		if val != "x" {
			id, _ := strconv.Atoi(val)
			if id > maxBusId {
				maxBusId = id
				maxIndex = i
			}
			busIds = append(busIds, id)
		} else {
			busIds = append(busIds, -1)
		}
	}

	// Part one
	p1 := PartOne(startTime, busIds)
	fmt.Println("Next bus id multiplied by the additional waiting minutes: ", p1)
	p2 := PartTwo(busIds, maxBusId, maxIndex)
	fmt.Println("Next bus such that all listed bus IDs depart at offsets matching their position in the list: ", p2)

}

func PartOne(startTime int, arr []int) int {
	chosenBus := -1
	minWait := math.MaxInt32
	for _, busId := range arr {
		if busId == -1 {
			continue
		}

		time := ((startTime/busId + 1) * busId) - startTime
		if time < minWait {
			chosenBus = busId
			minWait = time
		}
	}
	return chosenBus * minWait
}

func PartTwo(ids []int, max int, maxIndex int) *big.Int {
	// Struggled quite a bit on this one! Apparently use chinese remainder theorem
	var newIds []*big.Int
	var modulos []*big.Int

	for i, id := range ids {
		if id == -1 {
			continue
		}
		newIds = append(newIds, big.NewInt(int64(id)))
		modulos = append(modulos, big.NewInt(int64(id-i%id)))
	}
	result, _ := chineseRemainderTheorem(modulos, newIds)
	return result
}

func chineseRemainderTheorem(a, n []*big.Int) (*big.Int, error) {
	// https://rosettacode.org/wiki/Chinese_remainder_theorem
	p := new(big.Int).Set(n[0])
	for _, n1 := range n[1:] {
		p.Mul(p, n1)
	}
	var x, q, s, z big.Int
	for i, n1 := range n {
		q.Div(p, n1)
		z.GCD(nil, &s, n1, &q)
		if z.Cmp(big.NewInt(1)) != 0 {
			return nil, fmt.Errorf("%d not coprime", n1)
		}
		x.Add(&x, s.Mul(a[i], s.Mul(&s, &q)))
	}
	return x.Mod(&x, p), nil
}
