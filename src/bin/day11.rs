use std::collections::HashMap;

fn evolve_stone(value: u64) -> Vec<u64> {
    if value == 0 {
        vec![1]
    } else if (value.ilog10() + 1) % 2 == 0 {
        let digits = value.ilog10() + 1;
        vec![value / 10u64.pow(digits / 2), value % 10u64.pow(digits / 2)]
    } else {
        vec![value * 2024]
    }
}

fn simulate(stone: u64, blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if cache.contains_key(&(stone, blinks)) {
        return cache[&(stone, blinks)];
    }
    let count = if blinks == 0 {
        1
    } else {
        evolve_stone(stone)
            .iter()
            .map(|s| simulate(*s, blinks - 1, cache))
            .sum::<u64>()
    };
    cache.insert((stone, blinks), count);
    count
}

fn p1() {
    let stones = include_str!("../../input/day11")
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut memo_cache: HashMap<(u64, u64), u64> = HashMap::new();
    let p1 = stones
        .iter()
        .map(|s| simulate(*s, 25, &mut memo_cache))
        .sum::<u64>();
    println!("P1: {p1}");
}

fn p2() {
    let stones = include_str!("../../input/day11")
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut memo_cache: HashMap<(u64, u64), u64> = HashMap::new();
    let p2 = stones
        .iter()
        .map(|s| simulate(*s, 75, &mut memo_cache))
        .sum::<u64>();
    println!("P2: {p2}");
}
fn main() {
    p1();
    p2();
}
