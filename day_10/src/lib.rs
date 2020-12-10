
pub struct JoltageAdapterBag {
    bag_of_adapters: Vec<u32>,
}
impl JoltageAdapterBag {
    pub fn new(input: &str) -> Self {
        let mut bag_of_adapters: Vec<u32> = input.lines().map(|i| i.parse::<u32>().unwrap()).collect();
        // Charging outlet -> effective rating of 0
        bag_of_adapters.push(0);
        bag_of_adapters.sort_unstable();
        // built-in adapter -> 3 higher than max adapter
        bag_of_adapters.push(bag_of_adapters[bag_of_adapters.len() - 1] + 3);

        JoltageAdapterBag{bag_of_adapters}
    }

    pub fn part_one(&self) -> u32 {
        let mut jolt_differences: [u32; 3] = [0; 3];

        // helper function
        fn adapter_match(current_adapter: &u32, next_adapter: &u32, jolt_differences: &mut [u32; 3]) -> bool {
            match next_adapter - current_adapter {
                1 => jolt_differences[0] += 1,
                2 => jolt_differences[1] += 1,
                3 => jolt_differences[2] += 1,
                _ => return false,
            }
            true
        }

        for i in 1..self.bag_of_adapters.len() {
            let (current_adapter, next_adapter) = (self.bag_of_adapters[i-1], self.bag_of_adapters[i]);
            adapter_match(&current_adapter, &next_adapter, &mut jolt_differences);
        }

        // # of 1-jolt diff * # of 3-jolt diff
        jolt_differences[0] * jolt_differences[2]
    }

    pub fn part_two(&self) -> usize {
        // From data, there are no differences of 2. Only 1 and 3. If 3, that's the only possibility.
        // possibilies - DP w/ tabulation
        let mut p: [usize;3] = [0, 0, 1];
        for (&j1, &j3) in self.bag_of_adapters.iter().zip(self.bag_of_adapters.iter().skip(1)) {
            p = match j3 - j1 {
                1 => [p[1], p[2], p.iter().sum()],
                3 => [0, 0, p[2]],
                _ => panic!("Unexpected number"),
            }
        }
        p[2]
    }    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_example(){
        // joltage ratings of each adapters
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        let adapter_bag = JoltageAdapterBag::new(input);
        assert!(adapter_bag.part_one() == 35);
        assert!(adapter_bag.part_two() == 8);
    }

    #[test]
    fn larger_example(){
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        let adapter_bag = JoltageAdapterBag::new(input);
        assert!(adapter_bag.part_one() == 220);
        assert!(adapter_bag.part_two() == 19208);
    }
}