use day_9::{part_one, part_two};
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let invalid_number = part_one(&input, 25);
    if let Some(answer_part_one) =  invalid_number{
        println!("Invalid number found:\n {:?}", answer_part_one);
    }

    if let Some(answer_part_two) = part_two(&input, invalid_number.unwrap()) {
        println!("Sum of smallest and largest numbers in a contigous array (summing to invalid number):\n {:?}", answer_part_two);
    }
}
