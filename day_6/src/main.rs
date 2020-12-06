/*
Day 6: Custom Customs
- Form of 26 yes/no questions marked from a to z
- Only interested in which anyone in your group answers "yes" to. 


*/
use day_6::{part_one, part_two};
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let groups: Vec<&str> = input.split("\n\n").collect();

    println!("Sum of unique yes answers from all groups: {:?}", part_one(&groups));
    println!("Sum of concensus yes answers from every person in the group: {:?}", part_two(&groups));
}