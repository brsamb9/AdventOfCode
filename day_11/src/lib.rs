#[derive(Debug, Eq, PartialEq)]
struct SeatSystem {
    map: Vec<Vec<char>>,
    row_length: i32,
    col_length: i32,
}
impl SeatSystem {
    fn new(s: &str) -> Self {
        let map = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let row_length = map.len() as i32;
        let col_length = map[0].len() as i32;

        SeatSystem {
            map,
            row_length,
            col_length,
        }
    }

    fn next_state(&mut self, num_of_seats_rule: usize) -> bool {
        // Included a boolean output to indicate if changes have been made
        let mut changes_made = false;
        let state_copy = self.map.clone();
        for (i, v) in state_copy.iter().flatten().enumerate() {
            let row: usize = i / self.col_length as usize;
            let col: usize = i % self.col_length as usize;

            let count = match num_of_seats_rule {
                // not the cleanest implemenation at all - but can do much better next time!
                // part one -> immediate neighbours
                4 => self
                    .valid_neighbours((row, col))
                    .iter()
                    .map(|(i, j)| state_copy[*i][*j])
                    .filter(|c| c == &'#')
                    .count(),
                // part two -> continues in all directions until neighbour found
                5 => {
                    let mut count_o = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            let (mut dx_step, mut dy_step): (isize, isize) = (dx, dy);

                            let next_seat = loop {
                                let ny = (row as isize) + dy_step;
                                let nx = (col as isize) + dx_step;

                                if ny < 0
                                    || nx < 0
                                    || ny >= self.row_length as isize
                                    || nx >= self.col_length as isize
                                {
                                    break false;
                                }

                                match state_copy[ny as usize][nx as usize] {
                                    '#' => break true,
                                    'L' => break false,
                                    '.' => {
                                        dx_step += dx;
                                        dy_step += dy;
                                    }
                                    _ => unimplemented!(),
                                }
                            };
                            if next_seat {
                                count_o += 1;
                            }
                        }
                    }
                    count_o
                }
                _ => unimplemented!(),
            };

            match v {
                'L' => {
                    if count == 0 {
                        self.map[row][col] = '#';
                        changes_made = true
                    }
                }
                '#' => {
                    if count >= num_of_seats_rule {
                        self.map[row][col] = 'L';
                        changes_made = true
                    }
                }
                '.' => (),
                _ => unimplemented!(),
            }
        }
        changes_made
    }

    fn valid_neighbours(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        let i_curr: i32 = row as i32;
        let j_curr: i32 = col as i32;

        let valid_row_range: std::ops::Range<i32> = 0..self.row_length;
        let valid_col_range: std::ops::Range<i32> = 0..self.col_length;
        [
            (i_curr - 1, j_curr + 1),
            (i_curr, j_curr + 1),
            (i_curr + 1, j_curr + 1),
            (i_curr - 1, j_curr),
            (i_curr + 1, j_curr),
            (i_curr - 1, j_curr - 1),
            (i_curr, j_curr - 1),
            (i_curr + 1, j_curr - 1),
        ]
        .iter()
        .filter(|(i, j)| valid_row_range.contains(&i) && valid_col_range.contains(&j))
        .cloned()
        .map(|(t_row, t_col)| (t_row as usize, t_col as usize))
        .collect::<Vec<(usize, usize)>>()
    }
}

pub fn simulate(s: &str, num_of_seats_rule: usize) -> usize {
    let mut seating_map = SeatSystem::new(s);
    loop {
        if !seating_map.next_state(num_of_seats_rule) {
            break;
        }
    }
    seating_map
        .map
        .iter()
        .flatten()
        .fold(0, |acc, c| if c == &'#' { acc + 1 } else { acc })
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> String {
        "\
        L.LL.LL.LL\n\
        LLLLLLL.LL\n\
        L.L.L..L..\n\
        LLLL.LL.LL\n\
        L.LL.LL.LL\n\
        L.LLLLL.LL\n\
        ..L.L.....\n\
        LLLLLLLLLL\n\
        L.LLLLLL.L\n\
        L.LLLLL.LL"
            .to_owned()
    }

    #[test]
    fn test_part_one() {
        let initial_state = setup();
        let ans = simulate(&initial_state, 4);
        assert!(ans == 37);
    }

    #[test]
    fn test_part_two() {
        let initial_state = setup();
        let ans = simulate(&initial_state, 5);
        assert!(ans == 26);
    }
}
