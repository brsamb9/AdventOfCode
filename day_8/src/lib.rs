use std::collections::HashSet;


#[derive(Debug, Clone, PartialEq)]
enum InstructOp {
    Acc(i32), // incr / decr global accumulator
    Jmp(i32), // Jumps to a new instruction relative to itself
    Nop(i32), // No operation - does nothing
}
impl From<&str> for InstructOp {
    fn from(s: &str) -> Self {
        // Ideally would like better error handling here - but 'perfect' input.
        let mut split = s.splitn(2, " ");
        let instr = split.next().unwrap();
        let val =  split.next().unwrap().parse::<i32>().expect("Number value wasn't provided");

        match instr {
            "acc" => InstructOp::Acc(val),
            "jmp" => InstructOp::Jmp(val),
            "nop" => InstructOp::Nop(val),
            _ => panic!("Oh, the input wasn't perfect after all"),
        }
    }
}

impl InstructOp {
    fn flip(instr: &Self) -> Self {
        match instr {
            InstructOp::Jmp(v) => InstructOp::Nop(*v),
            InstructOp::Nop(v) => InstructOp::Jmp(*v),
            InstructOp::Acc(v) => InstructOp::Acc(*v),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    Running, 
    Infinite,
}

#[derive(Debug)]
pub struct GameState {
    instructions: Vec<InstructOp>,
    index: usize,
    accumulator: i32,
    jumped_from: HashSet<usize>,
    state: State,
}
impl GameState {
    pub fn new(s: &str) -> Self {
        GameState {
            instructions: s.lines().map(|l| InstructOp::from(l) ).collect::<Vec<InstructOp>>(),
            index: 0,
            accumulator: 0,
            jumped_from: HashSet::new(),
            state: State::Running,
        }
    }

    pub fn part_one(&mut self) -> i32 {
        self.reset();
        self.engine();
        self.accumulator
    }

    pub fn part_two(&mut self) -> i32 {
        // Brute force first then figure out if smarter method
        // Flip Jmp or Nop
        let instruction_clone = self.instructions.clone();
        let total_instruction_length = self.instructions.iter().len();

        let mut index_flip: usize = 0;
        while index_flip < total_instruction_length {
            self.reset();

            while index_flip < total_instruction_length {
                match self.instructions[index_flip] {
                    InstructOp::Jmp(_) | InstructOp::Nop(_) => {
                        self.instructions[index_flip] = InstructOp::flip(&self.instructions[index_flip]);
                        break;
                    },
                    InstructOp::Acc(_) => index_flip += 1,
                } 
            }
            
            self.engine();
            
            match self.state {
                State::Running => break,
                State::Infinite => {
                    if index_flip == total_instruction_length { panic!("Couldn't fix game"); }
                    
                    // revert back to normal for next cycle
                    self.instructions[index_flip] = instruction_clone[index_flip].clone();
                    index_flip += 1;
                },
            }
        }
        self.accumulator
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.jumped_from = HashSet::new();
        self.index = 0;
        self.state = State::Running;
    }


    fn engine(&mut self) -> i32 {
        // As there are no conditionals in instructions -> if jumped from before -> infinite loop
        // Place uncertain accumulator contributions incase second cycle
        let mut possible_acc: i32 = 0;

        while self.index < self.instructions.len() {
            match self.instructions[self.index] {
                InstructOp::Acc(v) => {
                    self.accumulator += v;
                    possible_acc += v;
                    self.index += 1;
                },
                InstructOp::Jmp(v) => {
                    if !self.jumped_from.insert(self.index) {
                        self.state = State::Infinite;
                        break;
                    }
                    // We now know these contributions are fine -> reset uncertain values.
                    possible_acc = 0;
                    // https://stackoverflow.com/questions/54035728/how-to-add-a-negative-i32-number-to-an-usize-variable
                    self.index = match v.is_negative() {
                        true => self.index.checked_sub(v.wrapping_abs() as u32 as usize).expect("Didn't expect a negative index"),
                        false => self.index.checked_add(v as usize).unwrap(),
                    };
                },
                InstructOp::Nop(_) => self.index += 1,
            }
        }
        // remove uncertain values that are in the second cycle
        if self.state == State::Infinite {
            self.accumulator -= possible_acc;
        }

        self.accumulator
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let mut a = GameState::new(&input);
        assert!(a.part_one() == 5);
    }

    #[test]
    fn part_two() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let mut a = GameState::new(&input);
        assert!(a.part_two() == 8);
    }

    #[test]
    fn test_flip() {
        let ins_acc = InstructOp::Acc(1);
        let ins_jmp = InstructOp::Jmp(5);
        let ins_nop = InstructOp::Nop(-4);

        assert!(InstructOp::flip(&ins_acc) == InstructOp::Acc(1));
        assert!(InstructOp::flip(&ins_jmp) == InstructOp::Nop(5));
        assert!(InstructOp::flip(&ins_nop) == InstructOp::Jmp(-4));
    }
}
