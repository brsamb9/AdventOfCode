use std::collections::{HashMap, HashSet, VecDeque};

type BagEntry<'a> = HashMap<String, Vec<(u32, String)>>;

pub fn to_bag_hashmap(input: &str) -> BagEntry {
    let mut bag_map: BagEntry = HashMap::new();
    for line in input.lines() {
        let mut tmp = line.trim().splitn(2, "contain");
        let name: String = tmp.next().unwrap().split("bag").next().unwrap().trim().to_string();
        
        let in_bag = tmp.next().unwrap();
        let can_contain: Vec<(u32, String)> = match in_bag.find("no").is_some() {
            true => vec![],
            false => {
                let mut contains = vec![];
                for cb in in_bag.split(',') {
                    let mut bag_parse = cb.split_whitespace();
                    let num: u32 = bag_parse.next().unwrap().parse::<u32>().expect("Invalid input");
                    let bag: String  = bag_parse.take(2).collect::<Vec<&str>>().join(" ");
                    contains.push((num, bag));
                }
                contains
            }
        };
        bag_map.insert(name, can_contain );
    }
    bag_map
} 

pub fn part_one (chosen_bag: &str, bag_map: &BagEntry) -> usize {

    // Given hash_map is the top-down hierarchy - reverse this to figure out the respective 'parent/larger bags' 
    let mut reversed_hashmap: HashMap<&str, Vec<&str>> = HashMap::new();
    for (parent_bag, contains) in bag_map.iter() {
        for (_, child_bag) in contains.iter() {
            match reversed_hashmap.contains_key(&child_bag[..]) {
                true => {
                    if let Some(x) = reversed_hashmap.get_mut(&child_bag[..]) {
                        x.push(parent_bag);
                    };
                },
                false => {reversed_hashmap.insert(&child_bag[..], vec![parent_bag]);},
            }
        }
    }

    // Then placing all the larger bags into a set via help from a queue (iteratively figure out the larger bags)
    let mut parent_set: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();

    match reversed_hashmap.get(chosen_bag){
        Some(v) => v.iter().for_each(|&name| queue.push_back(name)),
        None => return 0,
    }

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        if parent_set.insert(next) {
            if let Some(values) = reversed_hashmap.get(next) {
                values.iter().for_each(|&i| {
                    if !parent_set.contains(i) {
                        queue.push_back(i);
                    }
                })
            }
        }
    }
    
    parent_set.iter().count()
}

pub fn part_two(chosen_bag: &str, bag_map: &BagEntry) -> u32 {
    // Recursion to count the product 
    1 + bag_map[chosen_bag].iter().map(|(c,b)| c * part_two(b, bag_map)).sum::<u32>()
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_one(){
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\
        \ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\
        \nbright white bags contain 1 shiny gold bag.\
        \nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\
        \nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\
        \ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\
        \nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\
        \nfaded blue bags contain no other bags.\
        \ndotted black bags contain no other bags.";
        
        let all_bags = to_bag_hashmap(&input);
        assert!(part_one("shiny gold", &all_bags) == 4);
        assert!(part_one("dark olive", &all_bags) == 5);
        assert!(part_one("dotted black", &all_bags) == 7);
        assert!(part_one("light red", &all_bags) == 0);
    }
    

    #[test]
    fn test_part_two(){
        let input = "shiny gold bags contain 2 dark red bags.\
        \ndark red bags contain 2 dark orange bags.\
        \ndark orange bags contain 2 dark yellow bags.\
        \ndark yellow bags contain 2 dark green bags.\
        \ndark green bags contain 2 dark blue bags.\
        \ndark blue bags contain 2 dark violet bags.\
        \ndark violet bags contain no other bags.";
        let all_bags = to_bag_hashmap(&input);

        assert!(part_two("shiny gold", &all_bags) - 1 == 126);
    }
}