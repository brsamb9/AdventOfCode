

use day_7::{to_bag_hashmap, part_one, part_two};

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input.txt")?;

    let bag_connections = to_bag_hashmap(&input);

    println!("Part one - # of parent bags: {}", part_one("shiny gold", &bag_connections));
    // Negative one to exclude the shiny gold bag itself
    println!("Part two - total # of bags: {}", part_two("shiny gold", &bag_connections) - 1);

    Ok(())

}
