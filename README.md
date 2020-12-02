# AdventOfCode
Fun two parter challenges per day till christmas. 

## Day 1: Report Repair
Find the two entries that sum to 2020 -> provide the product ..<br>
__Part 1__:  of two entries.<br>
__Part 2__:  of three entries.<br>
<br>__Algorithm__<br>
Rather than using brute force to check all pairs in both parts; I opted to sort the vector and use two pointers to quickly find the *only pair*.<br>
Another method is to use hashing (check if (sum - entry) in hashset).<br>

## Day 2: Password Philosophy
Find how many passwords are valid according to the following rulesets:<br>
Schema: "i-j character: password" -- e.g. "1-3 a: abcde"<br>
__Part 1__: given character only appears in password for i <= character <= j times.<br>
__Part 2__: given character only appears in password at location i or j.<br>
