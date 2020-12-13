# AdventOfCode
Fun daily challenges with two parter in Decemeber. To get me posting to github more frequently! These mini-projects won't focus on error-handling too much as the given inputs are 'perfect' but will consist of small test cases to confirm correctness.<br>
Days 1-11 (Rust); 12- (Golang)
## Day 1: Report Repair
Find the two entries in the list of numbers that sum to 2020 -> provide the product ..<br>
__Part 1__:  of two entries.<br>
__Part 2__:  of three entries.<br>
Rather than using brute force to check all pairs in both parts; I opted to sort the vector and use two pointers to quickly find the *only pair*. Another method is to use hashing (check if (sum - entry) in hashset).

## Day 2: Password Philosophy
Find how many passwords are valid according to the following rulesets:<br>
Schema: "i-j character: password" -- e.g. "1-3 a: abcde"<br>
__Part 1__: given character only appears in password for i <= character <= j times.<br>
__Part 2__: given character only appears in password at location i or j.

## Day 3: Toboggan Trajectory
Given a 'map' of open lands (.) and trees (#) and a slope (dx, dy), output the number of trees landed on from the top left corner to the bottom row. Periodic in the x boundary.<br>
__Part 1__: \# of trees with a slope of (3,1).<br>
__Part 2__: Product of \# trees with slopes of ((1,1), (3,1), (5,1), (7,1), (1,2)).

## Day 4: Passport Processing
Strings consisting of 'passport fields' -> parsed ensemble into passports and then check for validity.<br>
__Part 1__: return \# of valid passports: all fields / or just missing country ID .<br>
__Part 2__: return \# of valid passports: Also, additional checks on fields to be within a set of ranges.<br>

## Day 5: Binary Boarding
A sequence of letters, F/B for row \# & L/R for col \#, to decode a seat id on the plane. 
__Part 1__: Return the highest seat id number on the list of inputs<br>
__Part 2__: Find missing seat id (i.e. empty seat) <br>


## Day 6: Custom Customs
Groups of people were given a form of 26 questions (mapped as a-z). The provided input contains the ensemble of groups and their letters that corresponds to questions with 'yes' answers.
__Part 1__: return the total sum of unique yes answers per group. <br>
__Part 2__: return the total sum of yes answers with a consensus in the group. <br>

## Day 7: Handy Haversacks
New rules enforced on luggage: colour-coded and specific quantities of other colour-coded bags. Your bag happens to be "shiny gold".
e.g. "shiny gold bags contain 2 dark red bags.". It is important to notice the hierarchical nature of the given rules.<br>
__Part 1__: return # of bag colors that can contain least one shiny gold bag? <br>
__Part 2__: How many individual bags are required inside your single shiny gold bag? <br>


## Day 8: Handheld Halting
Game instructions of {acc: global accumulation, jmp: jump to instruction, nop: no operation} are provided with a value per line. But these are stuck in an infinite loop.<br>
__Part 1__: Return the accumulation of points before it goes into its next cycle in the infinite loop. <br>
As there are no conditionals in this naive instruction set, whatever instruction we've jumped from before can be put into a hashset -> check if jumped from before == infinite loop.<br>
__Part 2__: Fix the infinite loop and return the completed accumulation. <br>

## Day 9: Encoding Error
Non-standardised port outputs a series of encrypted numbers (an old cypher with an important weakness!). Starts: preamble of 25 numbers, then each number you receive should be the sum of any two of the 25 immediately previous numbers - may be more than one pair: __two numbers in a pair must be different__. <br>
__Part 1__: What is the first number in the sequence that isn't the sum of two numbers in the moving window.<br>
__Part 2__: Find the slice of an array that sums up to the previous invalid number in part 1 -> return sum of smallest and largest number of this.

## Day 10: Adapter Array
User has a bag of 'Joltage' adapters rated by specific output joltage (input). Can link adapter if 1/2/3 less than its rating.<br>
__Part 1__: Product of # 1-jolt differences multiplied by # 3-jolt differences<br>
__Part 2__: # of distinct possible arrangements<br>

## Day 11: Seating System
Seat layout mapped as a grid of floor (.), empty seat (L), or occupied seat (#). All decisions are based on the number of occupied seats adjacent with the ruleset. Flip states of the seat if: 
- Empty -> occupied if no occupied seats adjacent to it.
- Occupied -> empty if four of more seats that are occupied adjacent.
- Otherwise seat's state does not change.

Like a naive version of game of life with two rules.<br>
__Part 1__: As explained above. Count number of seats when states are in equilibrium.<br>
__Part 2__: Changes to ruleset, seats adjacent changes meaning to any occupied seats in all eight directions. Also changing states from occupied is changed from 4 to 5 adjacent.<br>

## Day 12: Rain Risk - Switching to Golang for fun.
Ferry navigation set: to move a direction { N : north, S: south, E: east, W: west, F: forward } with a given value or turn {L: left, R: right} given number of degrees. Starts by facing east - only the L and R actions change the direction the ship is facing. 
Ferry's needs to evade.
__Part 1__: Move ship as instruction - return the Manhattan distance from original placement <br>
__Part 2__: Move waypoint relative to ship, and the ship itself. Return the Manhattan distance from original placment<br>

## Day 13 Shuttle Search
Bus schedule given with timestamps. <br>
__Part 1__: Given a starting time, calculate the next departing bus's id multiplied by the additional waiting minutes.<br>
__Part 2__: This was rather difficult, learnt about Chinese Remainder theorem - Rosetta code is an interesting website. Return the next bus such that all listed bus IDs depart at offsets matching their position in the list. <br>

