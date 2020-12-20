use day_17::ConwayGrid;

fn main() {
    let input: &str =
        ".##.####\n.#.....#\n#.###.##\n#####.##\n#...##.#\n#######.\n##.#####\n.##...#.\n";
    println!(
        "Part 1 (3D) Active states after six turns: {}",
        part_one(input)
    );
    println!(
        "Part 2 (4D) Active states after six turns: {}",
        part_two(input)
    );
}

fn part_one(input: &str) -> usize {
    let mut game = ConwayGrid::new(input);
    println!("{:?}", game);
    for _ in 0..6 {
        game = game.simulate();
    }
    game.total_active_states()
}

fn part_two(input: &str) -> usize {
    unimplemented!()
}
