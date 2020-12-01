# AdventOfCode
Fun two parter challenges per day till christmas. 

## Day 1:
__Part 1__: Find the two entries that sum to 2020 -> provide the product of these two entries.<br>
__Part 2__: Same but with three entries. 
<br><br>__Algorithm__<br>
Rather than using brute force to check all pairs in both parts; I opted to sort the vector and use two pointers to quickly find the *only pair*.

Another method is to use hashing (check if (sum - entry) in hashset).
