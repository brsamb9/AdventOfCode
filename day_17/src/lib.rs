use std::collections::{hash_map::IterMut, HashMap, VecDeque};
use std::iter;

#[derive(Debug, Clone)]
struct Slice2D(VecDeque<VecDeque<bool>>);
// true == active, false == inactive

impl Slice2D {
    fn new(inital: &str) -> Self {
        let cross_section = inital
            .split("\n")
            .map(|l| {
                l.chars()
                    .map(|l| match l {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Error: unrecognised input"),
                    })
                    .collect::<VecDeque<bool>>()
            })
            .collect::<VecDeque<VecDeque<bool>>>();

        Slice2D(cross_section)
    }

    fn empty(size: &usize) -> Self {
        // Square
        let v: VecDeque<bool> = iter::repeat(false).take(*size).collect();
        let frame: VecDeque<VecDeque<bool>> = iter::repeat(v).take(*size).collect();
        Slice2D(frame)
    }
    fn number_of_alive_in_frame(&self) -> usize {
        // Not elegent ->
        self.0
            .iter()
            .map(|v| v.iter().filter(|&i| *i).count())
            .sum()
    }
}
#[derive(Debug, Clone)]
struct ConwayGrid(HashMap<i32, Slice2D>);

impl ConwayGrid {
    pub fn new(inital: &str, zID: i32) -> Self {
        let mut map: HashMap<i32, Slice2D> = HashMap::new();

        let initial: Slice2D = Slice2D::new(inital);
        map.insert(zID, initial);

        ConwayGrid(map)
    }

    pub fn total_active_states(&self) -> usize {
        self.0
            .iter()
            .fold(0, |acc, (_, v)| acc + v.number_of_alive_in_frame())
    }

    pub fn simulate_turn(&mut self) {
        let state_copy = self.0.clone();
        self.apply_rules(&state_copy);
    }

    fn apply_rules(&self, state_copy: &HashMap<i32, Slice2D>) {
        let should_expand = false;
        if should_expand {
            self.expansion();
        }
    }

    fn expansion(&mut self) {
        // Add inactive dimensions to edges. E.g. Z = 0 -> Z = -1, 0, 1
        let size: &usize = &self.0[&0].0[0].len();

        let minD = &self.0.keys().min().unwrap();
        self.0.insert(*minD - 1, Slice2D::empty(size));

        let maxD = &self.0.keys().max().unwrap();
        self.0.insert(*maxD + 1, Slice2D::empty(size));

        // And expand the vectors -> e.g. 0 to left and right - [0,1,1,0,1] -> [0,0,1,1,0,1,0]
        self.0.iter_mut().for_each(|(_, frame)| {
            frame.0.iter_mut().for_each(|v| {
                v.push_front(false);
                v.push_back(false);
            })
        });
    }

    fn neighbours(&self, index: (u32, u32, i32)) -> [bool; 26] {
        // Infinite -> no edges to worry about
        [true; 26]
    }

    fn atIndex(&self, x: usize, y: usize, z: i32) -> bool {
        self.0[&z].0[x][y]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_struct() {
        let initial_2D_slice = ".#.\n..#\n###";
        let mut Game = ConwayGrid::new(initial_2D_slice, 0);

        assert!(5 == Game.total_active_states());
        println!("{:?}", Game);
        Game.simulate_turn();
        println!("{:?}", Game);
    }
}
