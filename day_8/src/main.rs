use day_8::GameState;

fn main() -> Result<(), std::io::Error> {
    // Day_8 - find infinite loop
    let input = std::fs::read_to_string("input.txt")?;

    let mut instructions= GameState::new(&input);
    println!("Part One\n Accumulator before infinite cycle loop: {}", instructions.part_one());
    println!("Part Two\n Fixed Program - value of accumulator -> {}", instructions.part_two());
    
    Ok(())
}

