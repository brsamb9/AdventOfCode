package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Boundary struct {
	lower, higher int
}

type TicketRule struct {
	name       string
	boundaries [2]Boundary
}

type Ticket struct {
	numbers []int
}

func main() {
	input, err := ioutil.ReadFile("day_16/input.txt")
	if err != nil {
		panic(err)
	}

	sections := strings.Split(string(input), "\n\n")

	// Parsed
	ruleset := parsedRuleSet(strings.Split(sections[0], "\n"))
	nearbyTickets := parsedTickets(strings.Split(sections[2], ":")[1])

	p1, validTickets := partOne(ruleset, nearbyTickets)
	fmt.Println("Part one: ", p1)

	myTicket := parsedTickets(strings.Split(sections[1], ":")[1])

	p2 := partTwo(ruleset, validTickets, myTicket[0])
	fmt.Println("Part two: ", p2)
}

func partOne(ticketRules []TicketRule, nearbyTickets []Ticket) (int, []Ticket) {
	var validTickets []Ticket
	invalidTicketSum := 0

	// Go through each ticket to check their values are within any of the provided ranges in the rulests
	for _, ticket := range nearbyTickets {
		errorTicketIds := idErrorsSum(ticket, ticketRules)

		if errorTicketIds != 0 {
			invalidTicketSum += errorTicketIds
		} else {
			validTickets = append(validTickets, ticket)
		}
	}
	return invalidTicketSum, validTickets
}

func partTwo(ruleset []TicketRule, validTickets []Ticket, myTicket Ticket) int {
	colFieldMap := determineFields(ruleset, validTickets)

	ans := 1
	for col, name := range colFieldMap {
		if strings.HasPrefix(name, "departure") {
			ans *= myTicket.numbers[col]
		}
	}
	return ans

}

func possibilityMap(rulesLeft []TicketRule) map[string][]int {
	m := map[string][]int{}
	for _, r := range rulesLeft {
		m[r.name] = []int{}
	}
	return m
}

func determineFields(ticketRules []TicketRule, tickets []Ticket) map[int]string {
	fieldNames := map[int]string{}

	for i, n := range ticketRules {
		fmt.Println(i, n.name)
	}

	for len(ticketRules) > 0 {
		possibilities := possibilityMap(ticketRules)

		for _, rule := range ticketRules {
			for i := 0; i < len(tickets[0].numbers); i++ {
				colValid := true

				if _, ok := fieldNames[i]; ok {
					continue
				}

				for j := 0; j < len(tickets); j++ {
					field := tickets[j].numbers[i]
					withinBound := false
					for _, bound := range rule.boundaries {
						if bound.lower <= field && field <= bound.higher {
							withinBound = true
						}
					}
					if !withinBound {
						colValid = false
						break
					}
				}
				if colValid {
					possibilities[rule.name] = append(possibilities[rule.name], i)
				}
			}
		}

		for k, v := range possibilities {
			if len(v) == 1 {
				fieldNames[v[0]] = k
				ticketRules = removeTicket(ticketRules, k)
			}
		}

	}

	fmt.Println(fieldNames)
	return fieldNames
}

func removeTicket(ticketRules []TicketRule, key string) []TicketRule {
	var ticketsLeft []TicketRule
	for _, rule := range ticketRules {
		if rule.name == key {
			continue
		}
		ticketsLeft = append(ticketsLeft, rule)
	}
	return ticketsLeft
}

func idErrorsSum(ticket Ticket, ticketRules []TicketRule) int {
	err := 0
	for _, ticketNum := range ticket.numbers {
		valid := isValidNum(ticketNum, ticketRules)
		if !valid {
			err += ticketNum
		}
	}
	return err
}

func isValidNum(ticketNum int, ticketRules []TicketRule) bool {
	valid := false
	for _, rule := range ticketRules {
		for _, bound := range rule.boundaries {
			if bound.lower <= ticketNum && ticketNum <= bound.higher {
				valid = true
			}
		}
	}
	return valid
}

func parsedTickets(nearbyTicketsInput string) []Ticket {
	var tickets []Ticket

	lines := strings.Split(nearbyTicketsInput, "\n")
	for _, line := range lines {
		if line == "" {
			continue
		}
		values := strings.Split(line, ",")
		var nums []int
		for _, val := range values {
			number, _ := strconv.Atoi(val)
			nums = append(nums, number)
		}
		tickets = append(tickets, Ticket{numbers: nums})
	}
	return tickets
}

func parsedRuleSet(rules []string) []TicketRule {
	parsedRuleSet := make([]TicketRule, len(rules))

	for i, rule := range rules {

		// Always follows: Name of row: i-j or k-l
		lineSplit := strings.Split(rule, ":")
		boundaries := strings.Split(lineSplit[1], " or ")

		var ticketInfo TicketRule

		ticketInfo.name = lineSplit[0]

		for j, pair := range boundaries {
			split := strings.Split(pair, "-")
			lower, _ := strconv.Atoi(strings.TrimSpace(split[0]))
			higher, _ := strconv.Atoi(strings.TrimSpace(split[1]))

			ticketInfo.boundaries[j] = Boundary{lower: lower, higher: higher}
		}

		parsedRuleSet[i] = ticketInfo
	}
	return parsedRuleSet
}
