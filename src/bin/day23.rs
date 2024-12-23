use std::collections::{HashMap, HashSet};

fn to_u16(s: &str) -> u16 {
    assert!(s.len() == 2);
    (s.as_bytes()[0] as u16) << 8 | s.as_bytes()[1] as u16
}

fn from_u16(u: u16) -> String {
    String::from_utf8(vec![(u >> 8) as u8, (u & 0xFF) as u8]).unwrap()
}

fn starts_with(u: u16, ch: char) -> bool {
    (u >> 8) as u8 as char == ch
}

fn p1() {
    let mut adj_map: HashMap<u16, HashSet<u16>> = HashMap::new();
    include_str!("../../input/day23").lines().for_each(|line| {
        let (left, right) = line.split_once('-').unwrap();
        adj_map
            .entry(to_u16(left))
            .or_default()
            .insert(to_u16(right));
        adj_map
            .entry(to_u16(right))
            .or_default()
            .insert(to_u16(left));
    });
    let mut triplets: HashSet<Vec<&u16>> = HashSet::new();

    for a in adj_map.keys() {
        for b in adj_map.get(a).unwrap() {
            for c in adj_map.get(b).unwrap() {
                if c != a
                    && adj_map.get(c).unwrap().contains(a)
                    && (starts_with(*a, 't') || starts_with(*b, 't') || starts_with(*c, 't'))
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

fn bron(
    graph: &HashMap<u16, HashSet<u16>>,
    r: HashSet<u16>,
    mut p: HashSet<u16>,
    mut x: HashSet<u16>,
    cliques: &mut Vec<HashSet<u16>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
    } else if !p.is_empty() {
        let nodes = p.iter().cloned().collect::<HashSet<u16>>();
        nodes.iter().for_each(|node| {
            let neighbours = graph.get(node).unwrap();
            let mut to_add: HashSet<u16> = HashSet::new();
            to_add.insert(*node);
            bron(
                graph,
                r.union(&to_add).cloned().collect(),
                p.intersection(neighbours).cloned().collect(),
                x.intersection(neighbours).cloned().collect(),
                cliques,
            );
            p.remove(node);
            x.insert(*node);
        })
    }
}

fn p2() {
    let mut adj_map: HashMap<u16, HashSet<u16>> = HashMap::new();
    include_str!("../../input/day23").lines().for_each(|line| {
        let (left, right) = line.split_once('-').unwrap();
        adj_map
            .entry(to_u16(left))
            .or_default()
            .insert(to_u16(right));
        adj_map
            .entry(to_u16(right))
            .or_default()
            .insert(to_u16(left));
    });
    let mut cliques: Vec<HashSet<u16>> = Vec::new();
    let r: HashSet<u16> = adj_map.keys().copied().collect();
    bron(&adj_map, HashSet::new(), r, HashSet::new(), &mut cliques);

    let mut longest = cliques
        .iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .iter()
        .map(|u| from_u16(*u))
        .collect::<Vec<_>>();
    longest.sort();
    let p2 = longest.join(",");
    println!("P2: {p2}");
}

fn main() {
    p1();
    p2();
}
