pub struct Map {
    pub matrix: Vec<Vec<char>>,
    height: usize,
    width: usize,
}
impl Map {
    pub fn input_to_map(s: &str) -> Self {
        Map {
            matrix: s.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>(),
            height: s.lines().fold(0, |acc, _| acc + 1),
            width: s.lines().next().unwrap().len(),
        }
    }

    pub fn part_one(&self) -> usize {
        let (dx, dy) = (3, 1);
        self.number_of_trees(dx, dy)
    }

    pub fn part_two(&self) -> usize {
        vec![(1, 1), (3,1), (5,1), (7,1), (1,2)].iter()
            .fold(1, |acc, (dx, dy )| acc * self.number_of_trees(*dx, *dy))
    }

    fn number_of_trees(&self, dx: usize, dy: usize) -> usize {
        let mut count: usize = 0;

        let (mut x, mut y) = (0, 0);
        while y < self.height {
            if self.matrix[y][x % self.width] == '#' { count += 1}
            x += dx;
            y += dy;
        }
        
        count
    }
}