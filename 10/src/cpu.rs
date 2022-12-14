#[derive(Debug)]
pub struct Registers {
    pub pc: usize,
    pub x: isize,
}

#[derive(Debug)]
pub enum OpCode {
    Noop,
    Addx,
}

impl OpCode {
    pub fn cycle_count(self: &Self) -> usize {
        match self {
            OpCode::Noop => 1,
            OpCode::Addx => 2,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub argument: Option<isize>,
}

pub struct Cpu {
    pub registers: Registers,
    pub program: Vec<Instruction>,
    cycle_count: usize,
    pub current_instruction_cycle: usize,
}

impl Cpu {
    pub fn load_from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Cpu {
        return Cpu {
            registers: Registers { x: 1, pc: 0 },
            program: lines.map(|line| Instruction::from_line(line)).collect(),
            cycle_count: 0,
            current_instruction_cycle: 0,
        };
    }

    pub fn run_cycle(self: &mut Self) {
        log::debug!(
            "Starting cycle {} - {}",
            self.cycle_count,
            self.current_instruction_cycle
        );

        if self.registers.pc >= self.program.len() {
            self.cycle_count += 1;
            return;
        }

        let instruction = &self.program[self.registers.pc];

        let cycles = instruction.opcode.cycle_count();

        if self.current_instruction_cycle == 0 {
            log::debug!("Starting instruction {:?}", instruction);
        }

        self.cycle_count += 1;
        self.current_instruction_cycle += 1;

        if self.current_instruction_cycle < cycles {
            return;
        }

        log::debug!("Dispatching instruction {:?}", instruction);

        match instruction.opcode {
            OpCode::Noop => {}
            OpCode::Addx => {
                self.registers.x += instruction.argument.unwrap();
            }
        }

        log::debug!("Reset (x now {})", self.registers.x);
        self.current_instruction_cycle = 0;
        self.registers.pc += 1;
    }
}

impl Instruction {
    pub fn from_line(line: &str) -> Instruction {
        let parts: Vec<&str> = line.split_whitespace().collect();

        return match parts[0] {
            "noop" => Instruction {
                opcode: OpCode::Noop,
                argument: None,
            },
            "addx" => Instruction {
                opcode: OpCode::Addx,
                argument: Some(parts[1].parse::<isize>().unwrap()),
            },
            _ => panic!("Unhandled instruction {}", parts[0]),
        };
    }
}
