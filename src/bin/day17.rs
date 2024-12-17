#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        let opcode = value.parse::<u64>().unwrap();
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
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
            Opcode::Adv => program.reg_a /= 2u32.pow(program.combo_to_value(program.instr[pc + 1])),
            Opcode::Bxl => program.reg_b ^= program.instr[pc + 1] as u32,
            Opcode::Bst => program.reg_b = program.combo_to_value(program.instr[pc + 1]) % 8,
            Opcode::Jnz => {
                if program.reg_a != 0 {
                    pc = program.instr[pc + 1] as usize;
                    continue;
                }
            }
            Opcode::Bxc => program.reg_b ^= program.reg_c,
            Opcode::Out => {
                print!("{},", program.combo_to_value(program.instr[pc + 1]) % 8);
            }
            Opcode::Bdv => {
                program.reg_b =
                    program.reg_a / (2u32.pow(program.combo_to_value(program.instr[pc + 1])))
            }
            Opcode::Cdv => {
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
