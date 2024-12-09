use std::collections::BinaryHeap;
use std::collections::HashMap;

fn p1() {
    let mut list_one: BinaryHeap<i64> = BinaryHeap::new();
    let mut list_two: BinaryHeap<i64> = BinaryHeap::new();

    include_str!("../../input/day1").lines().for_each(|line| {
        let mut splitted = line.split_whitespace();
        let first = splitted.next().unwrap().parse::<i64>().unwrap();
        let second = splitted.next().unwrap().parse::<i64>().unwrap();
        list_one.push(first);
        list_two.push(second);
    });

    assert!(list_one.len() == list_two.len());

    let res = list_one
        .into_sorted_vec()
        .iter()
        .zip(list_two.into_sorted_vec().iter())
        .map(|(first, second)| (first - second).abs())
        .sum::<i64>();

    println!("P1: {res}");
}

fn p2() {
    let mut ids: Vec<u64> = Vec::new();
    let mut freq: HashMap<u64, u64> = HashMap::new();

    include_str!("../../input/day1").lines().for_each(|line| {
        let mut splitted = line.split_whitespace();
        let first = splitted.next().unwrap().parse::<u64>().unwrap();
        let second = splitted.next().unwrap().parse::<u64>().unwrap();
        ids.push(first);
        *freq.entry(second).or_insert(0) += 1;
    });

    let res = ids
        .iter()
        .map(|id| id * freq.get(id).unwrap_or(&0))
        .sum::<u64>();

    println!("P2: {res:?}");
}

fn main() {
    p1();
    p2();
}
