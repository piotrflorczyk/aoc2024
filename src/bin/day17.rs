#[derive(Debug, Clone, Copy)]
enum Opcode {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        let opcode = value.parse::<u64>().unwrap();
        match opcode {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => panic!("WTF"),
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    instr: Vec<Opcode>,
}

impl Program {
    fn combo_to_value(&self, operand: Opcode) -> u32 {
        let op = operand as u32;
        match op {
            0..=3 => op,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("WTF"),
        }
    }
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<_>>();
        Program {
            reg_a: lines[0].split_once(": ").unwrap().1.parse::<u32>().unwrap(),
            reg_b: lines[1].split_once(": ").unwrap().1.parse::<u32>().unwrap(),
            reg_c: lines[2].split_once(": ").unwrap().1.parse::<u32>().unwrap(),
            instr: lines[4]
                .split_once(": ")
                .unwrap()
                .1
                .split(",")
                .map(Opcode::from)
                .collect::<Vec<_>>(),
        }
    }
}

fn p1() {
    let mut program = Program::from(include_str!("../../input/day17"));

    println!("{program:?}");

    let mut pc = 0;
    while pc < program.instr.len() {
        match program.instr[pc] {
            Opcode::ADV => {
                program.reg_a =
                    program.reg_a / (2u32.pow(program.combo_to_value(program.instr[pc + 1])))
            }
            Opcode::BXL => program.reg_b ^= program.instr[pc + 1] as u32,
            Opcode::BST => program.reg_b = program.combo_to_value(program.instr[pc + 1]) % 8,
            Opcode::JNZ => {
                if program.reg_a != 0 {
                    pc = program.instr[pc + 1] as usize;
                    continue;
                }
            }
            Opcode::BXC => program.reg_b ^= program.reg_c,
            Opcode::OUT => {
                print!("{},", program.combo_to_value(program.instr[pc + 1]) % 8);
            }
            Opcode::BDV => {
                program.reg_b =
                    program.reg_a / (2u32.pow(program.combo_to_value(program.instr[pc + 1])))
            }
            Opcode::CDV => {
                program.reg_c =
                    program.reg_a / (2u32.pow(program.combo_to_value(program.instr[pc + 1])))
            }
        }
        pc += 2
    }
}

fn main() {
    p1();
}
