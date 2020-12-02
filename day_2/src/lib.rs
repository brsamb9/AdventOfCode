#[derive(Debug)]
pub struct PasswordDB {
    info: Vec<PasswordPolicy>,
}
impl PasswordDB {
    pub fn new(s: String) -> Self {
        PasswordDB { 
            info:
                s.split("\n")
                .into_iter()
                .map(|x| {
                    PasswordPolicy::new(x)
                })
                .collect()
        }
    }

    pub fn total_valid_passwords(&self, part: &str) -> u32 {
        match part {
            "one" => self.info.iter().fold(0, |acc, i| acc + i.is_valid_password_part_one()),
            "two" => self.info.iter().fold(0, |acc, i| acc + i.is_valid_password_part_two()),
            _ => panic!("Part argument has to be either 'one' or 'two'"),
        }
        
    }
}

#[derive(Debug)]
struct PasswordPolicy {
    between: (u32, u32),
    letter: char,
    password: String,
}
impl PasswordPolicy {
    fn new(info: &str) -> Self {
        let parsed: Vec<&str> = info.split_whitespace().collect();
            
        let v: Vec<u32> = parsed[0].split("-").map(|i| i.parse::<u32>().unwrap()).collect();
        let between: (u32, u32) = (v[0], v[1]);
        
        let letter = parsed[1].chars().next().unwrap();

        let password: String = parsed[2].to_owned();

        PasswordPolicy{between, letter, password}
    }

    fn is_valid_password_part_one(&self) -> u32 {
        // Rule: if character count is i <= c <= j
        let letter_count: u32 = self.password.chars().fold(0, |acc, i| if i == self.letter {acc+1} else {acc});

        match letter_count <= self.between.1 && letter_count >= self.between.0 {
            true => 1,
            false => 0,
        } 
    }

    fn is_valid_password_part_two(&self) -> u32 {
        // Rule: If character count is in either position, but not both.
        let count = self.password.char_indices().fold(0, |acc, (i, c)| {
            match (i == self.between.0 as usize - 1 || i == self.between.1 as usize - 1) && c == self.letter {
                true => acc + 1,
                false => acc,
            }
        });
        if count == 1 { 1 } else { 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test(){
        let yes = PasswordPolicy{between: (1, 3), letter: "c".chars().next().unwrap(), password: "absc".to_owned()};
        assert!(yes.is_valid_password_part_one() == 1);

        let yes = PasswordPolicy{between: (1, 3), letter: "c".chars().next().unwrap(), password: "accc".to_owned()};
        assert!(yes.is_valid_password_part_one() == 1);

        let no = PasswordPolicy{between: (1, 3), letter: "c".chars().next().unwrap(), password: "absa".to_owned()};
        assert!(no.is_valid_password_part_one() == 0);
    }

    #[test]
    fn second_test(){
        let no = PasswordPolicy{between: (1, 3), letter: "c".chars().next().unwrap(), password: "absc".to_owned()};
        assert!(no.is_valid_password_part_two() == 0);

        let no = PasswordPolicy{between: (1, 3), letter: "c".chars().next().unwrap(), password: "cccc".to_owned()};
        assert!(no.is_valid_password_part_two() == 0);

        let yes = PasswordPolicy{between: (1, 3), letter: "c".chars().next().unwrap(), password: "caaa".to_owned()};
        assert!(yes.is_valid_password_part_two() == 1);
    }
}