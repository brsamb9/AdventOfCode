use day_11::simulate;

fn main() {
    let initial_input = std::fs::read_to_string("input.txt").unwrap();

    // Not the fastest/cleanest implementation - cloned map to have a reference. But it works!
    println!("Part 1: {}", simulate(&initial_input, 4));
    println!("Part 2: {}", simulate(&initial_input, 5));
}
