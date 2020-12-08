# AdventOfCode
Fun daily challenges with two parter in Decemeber.

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
Game instructions of {acc: global accumulation, jmp: jump to instruction, nop: no operation} are provided with a value per line. But these are stuck in an infinite loop.
__Part 1__: Return the accumulation of points before it goes into its next cycle in the infinite loop. <br>
As there are no conditionals in this naive instruction set, whatever instruction we've jumped from before can be put into a hashset -> check if jumped from before == infinite loop.
__Part 2__: Fix the infinite loop and return the completed accumulation. <br>