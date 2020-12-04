/*
All required fields:
    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID)

Each passport is represented as a sequence of key:value pairs, seperated by spaces or newlines.
- determine if passport is valid (all eight fields) or not.
- 'Special cases': valid even if cid field is missing.

Part one -> check passport all 8 fields, or 7\*, exist
Part two -> include checks on the fields' values 
*/

use day_4::Passports;
use std::{fs, io};

fn main() -> Result<(), io::Error> {

    let input = fs::read_to_string("input.txt")?;

    let entries = Passports::new(&input);
    println!("Part One: {:?}", entries.part_one());
    println!("Part Two:{:?}", entries.part_two());

    Ok(())
}
