use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate(i64, i64, i64);

#[derive(Debug, Clone)]
pub struct ConwayGrid(HashMap<Coordinate, bool>);
// true == active, false == inactive

impl ConwayGrid {
    pub fn new(inital: &str) -> Self {
        let mut grid: HashMap<Coordinate, bool> = HashMap::new();

        for (row, line) in inital.split("\n").enumerate() {
            for (col, letter) in line.chars().enumerate() {
                let pos = Coordinate(row as i64, col as i64, 0);
                let state = match letter {
                    '#' => true,
                    '.' => false,
                    _ => unimplemented!(),
                };
                grid.insert(pos, state);
            }
        }
        ConwayGrid(grid)
    }

    pub fn total_active_states(&self) -> usize {
        self.0.len()
    }

    pub fn simulate(&self) -> Self {
        let mut map_neigh_count: HashMap<Coordinate, u32> = HashMap::new();

        for (k, v) in self.0.iter() {
            if *v {
                for x in -1..=1 {
                    for y in -1..=1 {
                        for z in -1..=1 {
                            if x == 0 && y == 0 && z == 0 {
                                continue;
                            }
                            let coord = Coordinate(k.0 + x, k.1 + y, k.2 + z);
                            let new_entry = map_neigh_count.entry(coord).or_insert(0);
                            *new_entry += 1;
                        }
                    }
                }
            }
        }
        let mut next_state: HashMap<Coordinate, bool> = HashMap::new();

        for (k, v) in map_neigh_count {
            match v {
                3 => {
                    next_state.insert(k, true);
                }
                2 => {
                    if let Some(&b) = self.0.get(&k) {
                        if b {
                            next_state.insert(k, true);
                        }
                    }
                }
                _ => continue,
            }
        }
        ConwayGrid(next_state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_struct() {
        let initial_2d_slice = ".#.\n..#\n###";
        let mut game = ConwayGrid::new(initial_2d_slice);
        for _ in 0..6 {
            game = game.simulate();
        }
        assert!(112 == game.total_active_states());
    }
}
