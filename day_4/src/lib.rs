use std::collections::HashMap;

struct Passport {
    entries: HashMap<String, String>,
}
impl Passport {
    fn new(entries: Vec<&str>) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();

        for e in entries.into_iter() {
            let mut message = e.splitn(2, ":");
            let key = message.next().unwrap();
            let value = message.next().unwrap();
            map.insert(key.to_owned(), value.to_owned());
        }
        Passport{entries: map}
    }

    fn valid_passport(&self) -> bool {
        let num = ["byr".to_owned(), "iyr".to_owned(), "eyr".to_owned(), "hgt".to_owned(), 
                            "hcl".to_owned(), "ecl".to_owned(), "pid".to_owned(), "cid".to_owned()]
                        .iter()
                        .filter(|f| self.entries.contains_key(*f))
                        .count();

        match num {
            8 => true,
            7 => { if self.entries.contains_key("cid") { false } else { true } },
            _ => false,
        }
    }
    fn data_validation(&self) -> bool {
        self.entries
            .iter()
            .all(|(k, v)| match &k[..] {
                "byr" => matches!(v.parse::<u32>(), Ok(1920..=2002)),
                "iyr" => matches!(v.parse::<u32>(), Ok(2010..=2020)),
                "eyr" => matches!(v.parse::<u32>(), Ok(2020..=2030)),
                "hgt" => {
                    let (height, unit) = (&v[..v.len()-2], &v[v.len()-2..]);
                    match unit {
                        "cm" => matches!(height.parse::<u32>(), Ok(150..=193)),
                        "in" => matches!(height.parse::<u32>(), Ok(59..=76)),
                        _ => false,
                    }
                },
                "hcl" => {
                    let mut c = v.chars();
                    c.next().unwrap() == '#' && c.all(|i| matches!(i, '0'..='9' | 'a'..='f'))
                },
                "ecl" => matches!(&v[..], "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
                "pid" => v.len() == 9 && v.chars().all(|c| matches!(c, '0'..='9')),
                "cid" => true,
                _ => panic!("huh"),
                }
            )   
    }
}
pub struct Passports {
    records: Vec<Passport>,
}

impl Passports {
    pub fn new(input: &str) -> Self {
        let raw_passports = Passports::split_text_into_passports(input);
        let records = raw_passports
            .into_iter()
            .map(|p| Passport::new(p))
            .collect::<Vec<Passport>>();
        
        Passports {records}
    }

    pub fn part_one(&self) -> usize {
        self.records.iter().filter(|i| i.valid_passport()).count()
    }

    pub fn part_two(&self) -> usize {
        self.records.iter().filter(|i| i.valid_passport() && i.data_validation()).count()
    }

    fn split_text_into_passports(s: &str) -> Vec<Vec<&str>> {
        s.split("\n\n")
            .map(|x| x.split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>()
    }
}