use std::{fs, io};
use day_2::{PasswordDB}; 

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input.txt")?;
    let formated_passwords: PasswordDB = PasswordDB::new(input);

    println!("Part One: {}", formated_passwords.total_valid_passwords("one"));
    println!("Part Two: {}", formated_passwords.total_valid_passwords("two"));

    Ok(())
}
