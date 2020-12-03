use std::{fs, io};
use day_3::Map;
fn main() -> Result<(), io::Error>{
    let input = fs::read_to_string("input.txt")?;
    
    let data = Map::input_to_map(&input);
    println!("{}", data.part_one());
    println!("{}", data.part_two());

    Ok(())
}
