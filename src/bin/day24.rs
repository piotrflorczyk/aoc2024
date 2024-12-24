use std::collections::HashMap;

const DELIMITER: &str = "\r\n\r\n";

#[derive(Debug, Clone)]
struct Gate {
    in1: String,
    in2: String,
    op: char,
}

fn p1() {
    let (input_string, gates_string) = include_str!("../../input/day24")
        .split_once(DELIMITER)
        .unwrap();
    let mut results = input_string
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            (name.to_string(), val.parse::<u8>().unwrap())
        })
        .collect::<HashMap<_, _>>();

    let mut gates = gates_string
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

fn main() {
    p1();
}
