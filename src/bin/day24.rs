use std::collections::HashMap;

const DELIMITER: &str = "\r\n\r\n";

#[derive(Debug, Clone)]
struct Gate {
    in1: String,
    in2: String,
    op: char,
}

impl Gate {
    fn gate_name(&self) -> String {
        let op_name = match self.op {
            '|' => "OR",
            '&' => "AND",
            '^' => "XOR",
            _ => panic!("WTF"),
        };
        self.in1.clone() + "_" + op_name + "_" + &self.in2
    }
}

fn load_input() -> (HashMap<String, u8>, HashMap<String, Gate>) {
    let (input_string, gates_string) = include_str!("../../input/day24")
        .split_once(DELIMITER)
        .unwrap();
    let results = input_string
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            (name.to_string(), val.parse::<u8>().unwrap())
        })
        .collect::<HashMap<_, _>>();

    let gates = gates_string
        .lines()
        .map(|line| {
            let (inp, out) = line.split_once(" -> ").unwrap();
            let inp_val = inp.split(" ").collect::<Vec<_>>();
            let op = match inp_val[1] {
                "AND" => '&',
                "OR" => '|',
                "XOR" => '^',
                _ => panic!("WTF"),
            };
            let gate = Gate {
                in1: inp_val[0].to_string(),
                in2: inp_val[2].to_string(),
                op,
            };
            (out.to_string(), gate)
        })
        .collect::<HashMap<_, _>>();
    (results, gates)
}

fn p1() {
    let (mut results, mut gates) = load_input();
    while !gates.is_empty() {
        let new_results = gates
            .iter()
            .filter(|&(_, gate)| results.contains_key(&gate.in1) && results.contains_key(&gate.in2))
            .map(|(name, gate)| {
                let i1 = *results.get(&gate.in1).unwrap();
                let i2 = *results.get(&gate.in2).unwrap();
                let out = match gate.op {
                    '|' => i1 | i2,
                    '&' => i1 & i2,
                    '^' => i1 ^ i2,
                    _ => panic!("WTF"),
                };
                (name.clone(), out)
            })
            .collect::<HashMap<_, _>>();

        gates.retain(|name, _| !new_results.contains_key(name));
        results.extend(new_results);
    }

    let p1 = results
        .iter()
        .filter(|(name, _)| name.starts_with("z"))
        .fold(0u64, |acc, (name, val)| {
            let pos = name[1..].parse::<u8>().unwrap();
            acc | ((*val as u64) << pos)
        });

    println!("P1: {p1}");
}

fn gen_graphviz(inputs: &HashMap<String, u8>, gates: &HashMap<String, Gate>) {
    println!("digraph {{");
    gates.iter().for_each(|(out, g_out)| {
        gates
            .iter()
            .filter(|(_, g)| g.in1 == *out || g.in2 == *out)
            .for_each(|(_, g)| {
                println!(
                    "{} -> {} [label=\"{}\"]",
                    g_out.gate_name(),
                    g.gate_name(),
                    out
                );
            });
        if !gates.iter().any(|(_, g)| g.in1 == *out || g.in2 == *out) {
            println!("{} -> {}", g_out.gate_name(), out);
        }
        let lbl = match g_out.op {
            '|' => "OR",
            '&' => "AND",
            '^' => "XOR",
            _ => panic!("WTF"),
        };
        let shape = match g_out.op {
            '|' => "square",
            '&' => "triangle",
            '^' => "diamond",
            _ => panic!("WTF"),
        };

        println!("{} [shape={},label=\"{}\"]", g_out.gate_name(), shape, lbl);
    });
    inputs.iter().for_each(|(inp, _)| {
        gates
            .iter()
            .filter(|(_, g)| g.in1 == *inp || g.in2 == *inp)
            .for_each(|(_, g)| {
                println!("{} -> {}", inp, g.gate_name());
            });
    });
    println!("}}");
}

fn get_invalid_outputs(gates: &HashMap<String, Gate>) -> Vec<String> {
    gates
        .iter()
        .filter(|&(out, gate)| {
            if gate.op == '^' {
                // start with x,y or output to z
                if !((gate.in1.starts_with('x') && gate.in2.starts_with('y'))
                    || (gate.in1.starts_with('y') && gate.in2.starts_with('x'))
                    || out.starts_with('z'))
                {
                    return true;
                }
                // XOR that takes in x,y outputs to AND and XOR
                if ((gate.in1.starts_with('x') && gate.in2.starts_with('y'))
                    || (gate.in1.starts_with('y') && gate.in2.starts_with('x')))
                    && gates
                        .iter()
                        .filter(|(_, g)| g.in1 == *out || g.in2 == *out)
                        .any(|(_, g)| g.op == '|')
                {
                    return true;
                }
            }
            // gate leading to output except last one is always XOR
            if out.starts_with("z") && out != "z45" && gate.op != '^' {
                return true;
            }
            // AND gate that takes input x,y except first one outputs to OR
            if (gate.op == '&'
                && (gate.in1 != "x00" && gate.in2 != "x00")
                && ((gate.in1.starts_with('x') && gate.in2.starts_with('y'))
                    || (gate.in1.starts_with('y') && gate.in2.starts_with('x'))))
                && !gates
                    .iter()
                    .filter(|(_, g)| g.in1 == *out || g.in2 == *out)
                    .all(|(_, g)| g.op == '|')
            {
                return true;
            }

            false
        })
        .map(|(out, _)| out.clone())
        .collect::<Vec<_>>()
}

fn p2() {
    let (_, gates) = load_input();
    //gen_graphviz(&inputs, &gates);
    let mut invalid = get_invalid_outputs(&gates);
    invalid.sort();
    println!("P2: {}", invalid.join(","));
}

fn main() {
    p1();
    p2();
}
