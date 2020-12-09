use std::{collections::{HashSet, VecDeque}, vec};

#[derive(Debug)]
struct XMAS {
    queue: VecDeque<isize>,
    size: usize,
}

impl XMAS {
    fn new(size: usize) -> Self {
        XMAS {queue: VecDeque::new(), size: size}
    }

    fn next(&mut self, next: isize){
        // Queue data structure: dequeue 'oldest' and enqueue next value for the following iteration
        if self.queue.len() == self.size {
            self.queue.pop_front();
        }
        self.queue.push_back(next);
    }

    fn sum_of_two_numbers_exists(&self, sum: isize) -> bool {
        let mut pair_check: HashSet<isize> = HashSet::new();

        self.queue.iter().any(|i| {
            pair_check.insert(sum - i);
            // remember pair must have two unique numbers
            pair_check.contains(i) && (*i != sum - i)
        })

    }
}


pub fn part_one(input: &str, size: usize) -> Option<isize> {
    let mut xmas = XMAS::new(size);
    
    for i in input.lines().map(|n| n.parse::<isize>().expect("Expected only numbers")) {
        let pair_found: bool = xmas.queue.len() == size && !xmas.sum_of_two_numbers_exists(i);
        match pair_found {
            true => return Some(i),
            false => xmas.next(i), // Place into queue for next sequence
        }
    }

    None // Couldn't find answer
}


pub fn part_two(input: &str, invalid_number: isize) -> Option<isize> {
    let vector: Vec<isize> = input.lines()
                                .map(|n| n.parse::<isize>().expect("Number expected"))
                                .collect::<Vec<isize>>();

    // two pointers to find contiguous set of at least two numbers
    for slow_p in 0..vector.len() {
        let (mut sum_vec, mut sum_val) = (vec![], 0);
        for fast_p in slow_p..vector.len() {
            let val = vector[fast_p];
            sum_vec.push(val);
            sum_val += val;

            if sum_val == invalid_number && sum_vec.len() > 2 {
                return Some(sum_vec.iter().min().unwrap() + sum_vec.iter().max().unwrap());
            }
            else if sum_val > invalid_number {
                break;
            }
        }
    }
    None // Couldn't find answer
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all(){
        let input: &str = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";

        let a = part_one(input, 5);
        assert!(a == Some(127));

        let b = part_two(input, a.unwrap());
        println!("{:?}", b);
        assert!(b == Some(62));

    }

}