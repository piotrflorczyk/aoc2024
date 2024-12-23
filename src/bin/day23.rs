use std::collections::{HashMap, HashSet};

fn p1() {
    let mut adj_map: HashMap<String, HashSet<String>> = HashMap::new();
    include_str!("../../input/day23").lines().for_each(|line| {
        let (left, right) = line.split_once('-').unwrap();
        adj_map
            .entry(left.to_string())
            .or_default()
            .insert(right.to_string());
        adj_map
            .entry(right.to_string())
            .or_default()
            .insert(left.to_string());
    });
    let mut triplets: HashSet<Vec<&String>> = HashSet::new();

    for a in adj_map.keys() {
        for b in adj_map.get(a).unwrap() {
            for c in adj_map.get(b).unwrap() {
                if c != a
                    && adj_map.get(c).unwrap().contains(a)
                    && (a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
                {
                    let mut trip = vec![a, b, c];
                    trip.sort();
                    triplets.insert(trip);
                }
            }
        }
    }
    let p1 = triplets.len();
    println!("P1: {p1}");
}

fn main() {
    p1();
}
