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


## Day 5: