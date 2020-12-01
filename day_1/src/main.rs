use std::fs;


struct Data {
    vector: Vec<u32>,
}
impl Data {
    fn new(input_text: &str) -> Self{
        let mut input: Vec<u32> = fs::read_to_string(input_text)
            .unwrap()
            .split("\n")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        input.sort_unstable();
        Data{vector: input}
    }

    fn part_one(&self) -> u32 {
        let mut i: usize = 0;
        let mut j: usize = self.vector.len() - 1;
    
        let (mut left, mut right) = (self.vector[i], self.vector[j]);
        
        while left < right {
            let sum = left + right;
            if sum == 2020 {
                break;
            }
            else if sum < 2020 {
                i += 1;
                left = self.vector[i];
            }
            else if sum > 2020 {
                j -= 1;
                right = self.vector[j];
            }
        }
        left * right
    }
    fn part_two(&self) -> Option<u32> {
        // Same but with three values
        
        for i in 0..self.vector.len() - 2 {
            let mut j = i + 1;
            let mut k = self.vector.len() - 1;


            while j < k {
                let sum = self.vector[i] + self.vector[j] + self.vector[k];
                if sum == 2020 {
                    return Some(self.vector[i] * self.vector[j] * self.vector[k]);
                }
                else if sum < 2020 {
                    j += 1;
                }
                else if sum > 2020 {
                    k -= 1;
                }
            }
        }
        None
    }
}

fn main() {
    let data = Data::new("/home/brs/Documents/AdventOfCode/Day1/report_repair/src/input.txt");

    let ans1 = data.part_one();
    println!("Answer One: {}", ans1);


    let ans2 = data.part_two();
    match ans2 {
        Some(val) => println!("Answer Two: {}", val),
        None => println!("Answer Two: Oh..."),
    }
}