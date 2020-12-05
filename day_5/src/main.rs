/*
Binary Boarding - quick notes:
- 128 rows in the place (F: front / B: back); 8 columns (L: left / R: right)
- sequence of letters provided: next letter indicates which half of that ragion the seat is in -> until one row left
- e.g. FBFBBFFRLR -> [0-63] -> [32-63] -> [32-47] -> [40-47] -> [44-47] -> [44-45] -> *row 44*
                    -> RLR -> [0-7] -> [4-7] -> [4-5] -> *column 5*
    decoded to row 44, column 5 -> unique seat ID: (Row * 8 + column) => 357
*/
fn main() -> Result<(), std::io::Error>{
    let input = std::fs::read_to_string("input.txt")?;

    let mut seats: Vec<u32> = input.lines().map(|l| get_seat_id(l)).collect::<Vec<u32>>();
    seats.sort_unstable();

    println!("Highest Seat id: {:?}", seats.iter().next_back().unwrap());
    println!("Empty seat: {:?}", seats.iter()
                                    .zip(seats.iter().skip(1))
                                    .find(|(&id1, &id2)| id2 - id1 != 1)
                                    .expect("No seat found")
                                    .0 + 1
    );

    Ok(())
}

fn get_seat_id(s: &str) -> u32 {
    let (mut row, mut col): ((u32, u32), (u32, u32)) = ((0,128), (0,8));
    for (i, c) in s.chars().enumerate() {
        if i < 7 {
            let dx = (row.1 - row.0) / 2;
            match c {
                'F' => { row.1 -= dx; }
                'B' => { row.0 += dx; }
                _ => panic!("Character not expected - F/B"), // not sure if there's a 'cleaner' way
            }
        } 
        else {
            let dy = (col.1 - col.0) / 2;
            match c {
                'L' => { col.1 -= dy; }
                'R' => { col.0 += dy; }
                _ => panic!("Character not expected - L/R"), 
            }
        }
    }
    row.0 * 8 + col.0
}


