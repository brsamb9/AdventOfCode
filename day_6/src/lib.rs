use std::{collections::{HashMap, HashSet}, vec};
use std::iter::FromIterator;

pub fn part_one(groups: &Vec<&str>) -> usize {
    let yes_answers: Vec<Vec<char>> = groups.iter().map(|&g| g.chars()
                                .filter(|c| c.is_alphabetic())
                                .collect() )
                        .collect();

    let sets: Vec<HashSet<char>> = yes_answers.iter().map(|q| vec_to_set(q)).collect();
    sets.iter().map(|s| s.iter().count()).sum()
}


fn vec_to_set(v: &Vec<char>) -> HashSet<char> {
        HashSet::from_iter(v.clone())
}


pub fn part_two(groups: &Vec<&str>) -> usize {
    let mut consensus_vec: Vec<usize> = vec![];

    for group in groups.iter() {
        
        let mut hashmap: HashMap<char, u32> = HashMap::new();
        let mut num_of_lines: u32 = 0;
        
        let per_person = group.split('\n');
        for l in per_person.into_iter() {
            num_of_lines += 1;
            for c in l.chars(){
                hashmap.insert(c, hashmap.get(&c).or_else(|| Some(&0)).unwrap()+1);
            }
        }
        
        let mut consensus: Vec<char> = vec![];
        hashmap.into_iter().for_each(|(c, n)| if n == num_of_lines {consensus.push(c)});

        if consensus.len() != 0 {
            consensus_vec.push(consensus.iter().count());
        } 
    }
    consensus_vec.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test(){
        let i = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let groups: Vec<&str> = i.split("\n\n").collect();

        assert!(part_one(&groups) == 11);
        assert!(part_two(&groups) == 6);
    }
}