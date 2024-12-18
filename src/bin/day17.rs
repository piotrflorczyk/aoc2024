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

#[derive(Debug, Clone, Copy)]
struct State {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

#[derive(Debug, Clone)]
struct Program {
    state: State,
    instr: Vec<Opcode>,
}

impl State {
    fn combo_to_value(&self, operand: Opcode) -> u64 {
        let op = operand as u64;
        match op {
            0..=3 => op,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("WTF"),
        }
    }
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<_>>();
        State {
            reg_a: lines[0].split_once(": ").unwrap().1.parse::<u64>().unwrap(),
            reg_b: lines[1].split_once(": ").unwrap().1.parse::<u64>().unwrap(),
            reg_c: lines[2].split_once(": ").unwrap().1.parse::<u64>().unwrap(),
        }
    }
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<_>>();
        Program {
            state: State::from(value),
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

fn emulate(program: &Program) -> Vec<u64> {
    let mut state = program.state.clone();
    let mut pc = 0;
    let mut output = Vec::new();
    while pc < program.instr.len() {
        match program.instr[pc] {
            Opcode::Adv => {
                state.reg_a /= 2u64.pow(state.combo_to_value(program.instr[pc + 1]) as u32)
            }
            Opcode::Bxl => state.reg_b ^= program.instr[pc + 1] as u64,
            Opcode::Bst => state.reg_b = state.combo_to_value(program.instr[pc + 1]) % 8,
            Opcode::Jnz => {
                if state.reg_a != 0 {
                    pc = program.instr[pc + 1] as usize;
                    continue;
                }
            }
            Opcode::Bxc => state.reg_b ^= state.reg_c,
            Opcode::Out => {
                output.push(state.combo_to_value(program.instr[pc + 1]) % 8);
                /* if output.len() == output_count {
                    break;
                }*/
            }
            Opcode::Bdv => {
                state.reg_b =
                    state.reg_a / 2u64.pow(state.combo_to_value(program.instr[pc + 1]) as u32)
            }
            Opcode::Cdv => {
                state.reg_c =
                    state.reg_a / 2u64.pow(state.combo_to_value(program.instr[pc + 1]) as u32)
            }
        }
        pc += 2
    }
    output
}

fn p1() {
    let program = Program::from(include_str!("../../input/day17"));
    let out = emulate(&program);
    let p1 = out
        .iter()
        .map(|el| format!("{}", el))
        .collect::<Vec<_>>()
        .join(",");
    println!("P1: {p1}");
}

fn p2() {
    let mut program = Program::from(include_str!("../../input/day17"));
    let expected_output = program.instr.iter().map(|&x| x as u64).collect::<Vec<_>>();
    let mut prev_valid: Vec<u64> = Vec::new();
    let mut valid: Vec<u64> = Vec::new();
    /*
       Loop while A != 0 {
           C = A >> ((A & 0b111) ^ 0b11) //[1]
           B = (A & 0b111) ^ C //[2]
           B = B ^ 0b11
           OUT B & 0b111
           A >>= 3
       }
       At each iteration we take last 3 bits [2] and and 3 bits from A >> [7-0]
    */
    for i in 1..=expected_output.len() {
        if i == 1 {
            // First iteration we check all 10 bits that could affect last element
            for val in 0b0..=0b1_111_111_111 {
                program.state.reg_a = val;
                // run emulation and compare if the 1st elemente from output matches
                let emu_out = emulate(&program);
                if emu_out[0] == expected_output[0] {
                    valid.push(val);
                }
            }
        } else {
            // to each element that was valid in previous step append every value from 0-0b111 and test
            for val in 0b0..=0b111 {
                prev_valid.iter().for_each(|v| {
                    let reg_a_val = val << (7 + 3 * (i - 1)) | v;
                    program.state.reg_a = reg_a_val;
                    let emu_out = emulate(&program);
                    if emu_out.len() <= expected_output.len()
                        && emu_out.len() >= i
                        && emu_out[..i] == expected_output[..i]
                    {
                        valid.push(reg_a_val);
                    }
                });
            }
        }
        std::mem::swap(&mut prev_valid, &mut valid);
        valid.clear();
    }
    let p2 = prev_valid[0];
    println!("P2: {p2}");
}

fn main() {
    p1();
    p2();
}
