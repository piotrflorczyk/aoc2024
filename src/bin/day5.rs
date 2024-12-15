use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

const DELIMITER: &str = "\r\n\r\n";

fn get_rules_map(rules: &str) -> HashMap<u32, HashSet<u32>> {
    let mut rules_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    rules.lines().for_each(|line| {
        let (before, after) = line.split_once("|").unwrap();
        // Build a map of forbidden orders i.e. key number cannot ever be before a set of value numbers
        rules_map
            .entry(after.parse::<u32>().unwrap())
            .or_default()
            .insert(before.parse::<u32>().unwrap());
    });
    rules_map
}

fn is_valid_order(rules_map: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    let mut seen: HashSet<u32> = HashSet::new();
    update.iter().rev().all(|&el| {
        // get forbidden orders
        let rules = rules_map.get(&el);
        // if there are no forbidden orders for a given number it's ok
        // if there are some but no number already seen is there it's also ok
        let res = rules.is_none() || seen.is_disjoint(rules.unwrap());
        seen.insert(el);
        res
    })
}

fn p1() {
    let (rules, updates) = include_str!("../../input/day5")
        .split_once(DELIMITER)
        .unwrap();
    let rules_map = get_rules_map(rules);
    let p1 = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|el| el.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|update| is_valid_order(&rules_map, update))
        .map(|update| update[update.len() / 2])
        .sum::<u32>();

    println!("{p1:?}");
}

fn p2() {
    let (rules, updates) = include_str!("../../input/day5")
        .split_once(DELIMITER)
        .unwrap();

    let rules_map = get_rules_map(rules);

    let p2 = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|el| el.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|update| !is_valid_order(&rules_map, update))
        .map(|mut update| {
            update.sort_by(|a, b| {
                if let Some(rule) = rules_map.get(b) {
                    if rule.contains(a) {
                        return Ordering::Less;
                    }
                }
                Ordering::Greater
            });
            update[update.len() / 2]
        })
        .sum::<u32>();

    println!("{p2:?}");
}

fn main() {
    p1();
    p2();
}
