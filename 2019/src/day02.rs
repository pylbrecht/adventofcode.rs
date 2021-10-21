use std::fs;

pub fn run() {
    let mut content = fs::read_to_string("input/day02.txt").unwrap();
    // get rid of trailing newline
    content.pop();

    let program: Vec<usize> = content
        .split(',')
        .map(|item| match item.parse() {
            Ok(num) => num,
            Err(_) => panic!("can't parse: {}", item),
        })
        .collect();

    let mut computer = IntcodeComputer::new();
    computer.load(&program);
    computer.set(1, 12);
    computer.set(2, 2);
    computer.run();

    println!("{}", computer.memory[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            computer.reset();
            computer.load(&program);
            computer.set(1, noun);
            computer.set(2, verb);
            computer.run();

            if computer.memory()[0] == 19690720 {
                println!("{}", (100 * noun) + verb);
                return;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Halt,
}

impl Instruction {
    fn from_slice(intcode: &[usize]) -> Instruction {
        let opcode = intcode[0];
        match opcode {
            1 => Instruction::Add(intcode[1], intcode[2], intcode[3]),
            2 => Instruction::Multiply(intcode[1], intcode[2], intcode[3]),
            99 => Instruction::Halt,
            _ => panic!("unknown opcode {}", opcode),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Halt,
}

impl Opcode {
    fn from_usize(num: usize) -> Opcode {
        match num {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::Halt,
            _ => panic!("unknown opcode: {}", num),
        }
    }
}

struct IntcodeComputer {
    memory: Vec<usize>,
    ip: usize,
}

impl IntcodeComputer {
    fn new() -> IntcodeComputer {
        IntcodeComputer {
            memory: Vec::new(),
            ip: 0,
        }
    }

    fn load(&mut self, program: &[usize]) {
        self.memory = program.to_vec();
    }

    fn run(&mut self) {
        while self.ip < self.memory.len() {
            match Instruction::from_slice(&self.memory[self.ip..]) {
                Instruction::Add(left, right, dest) => {
                    self.memory[dest] = self.memory[left] + self.memory[right];
                    self.ip += 4
                }
                Instruction::Multiply(left, right, dest) => {
                    self.memory[dest] = self.memory[left] * self.memory[right];
                    self.ip += 4
                }
                Instruction::Halt => break,
            }
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.memory.truncate(0);
    }

    fn memory(&self) -> &[usize] {
        &self.memory
    }

    fn set(&mut self, address: usize, value: usize) {
        self.memory[address] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_from_usize() {
        assert_eq!(Opcode::from_usize(1), Opcode::Add);
        assert_eq!(Opcode::from_usize(2), Opcode::Multiply);
        assert_eq!(Opcode::from_usize(99), Opcode::Halt);
    }

    #[test]
    fn instruction_from_slice() {
        assert_eq!(
            Instruction::from_slice(&[1, 0, 0, 0]),
            Instruction::Add(0, 0, 0)
        );
        assert_eq!(
            Instruction::from_slice(&[2, 0, 0, 0]),
            Instruction::Multiply(0, 0, 0)
        );
        assert_eq!(Instruction::from_slice(&[99]), Instruction::Halt);
    }

    #[test]
    fn add() {
        let program = [1, 0, 0, 0, 99];
        let mut computer = IntcodeComputer::new();
        computer.load(&program);
        computer.run();
        assert_eq!(computer.memory(), [2, 0, 0, 0, 99]);
    }

    #[test]
    fn multiply() {
        let program = [2, 3, 0, 3, 99];
        let mut computer = IntcodeComputer::new();
        computer.load(&program);
        computer.run();
        assert_eq!(computer.memory(), [2, 3, 0, 6, 99]);
    }

    #[test]
    fn multiply2() {
        let program = [2, 4, 4, 5, 99, 0];
        let mut computer = IntcodeComputer::new();
        computer.load(&program);
        computer.run();
        assert_eq!(computer.memory(), [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn add_and_multiply() {
        let program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut computer = IntcodeComputer::new();
        computer.load(&program);
        computer.run();
        assert_eq!(computer.memory(), [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn reset() {
        let program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut computer = IntcodeComputer::new();
        computer.load(&program);
        computer.run();
        assert_eq!(computer.memory(), [30, 1, 1, 4, 2, 5, 6, 0, 99]);

        computer.reset();
        assert_eq!(computer.memory(), []);
    }
}
