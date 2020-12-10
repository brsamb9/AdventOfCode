
use day_10::{JoltageAdapterBag};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let adapter_bag = JoltageAdapterBag::new(&input);
    // Use every adapter
    println!("Product of # 1-jolt differences multiplied by # 3-jolt differences:\n {}", adapter_bag.part_one());

    // # of distinct arrangements
    println!("# of distinct arrangements {}", adapter_bag.part_two());
}

