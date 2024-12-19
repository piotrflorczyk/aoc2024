use std::collections::HashMap;

const DELIMITER: &str = "\r\n\r\n";

fn p1() {
    let (colors_str, combinations_str) = include_str!("../../input/day19")
        .split_once(DELIMITER)
        .unwrap();
    let colors = colors_str.split(", ").collect::<Vec<_>>();
    let combinations = combinations_str.lines().collect::<Vec<_>>();
    let p1 = combinations
        .iter()
        .filter(|combo| {
            let mut queue: Vec<String> = Vec::new();
            queue.push(combo.to_string());

            while let Some(str) = queue.pop() {
                if str.is_empty() {
                    return true;
                }
                colors.iter().for_each(|color| {
                    if str.starts_with(color) {
                        queue.push(str[color.len()..].to_string());
                    }
                });
            }
            false
        })
        .count();
    println!("P1: {p1}");
}

fn dfs(str: String, colors: &Vec<&str>, cache: &mut HashMap<String, usize>) -> usize {
    if cache.contains_key(&str) {
        return cache[&str];
    }
    let combinations = colors
        .iter()
        .filter(|&color| str.starts_with(color))
        .map(|color| dfs(str[color.len()..].to_string(), colors, cache))
        .sum::<usize>();
    cache.insert(str, combinations);
    return combinations;
}

fn p2() {
    let (colors_str, combinations_str) = include_str!("../../input/day19")
        .split_once(DELIMITER)
        .unwrap();
    let colors = colors_str.split(", ").collect::<Vec<_>>();
    let combinations = combinations_str.lines().collect::<Vec<_>>();
    let mut cache: HashMap<String, usize> = HashMap::new();
    cache.insert("".to_string(), 1);

    let p2 = combinations
        .iter()
        .map(|combo| dfs(combo.to_string(), &colors, &mut cache))
        .sum::<usize>();
    println!("P2: {p2}");
}

fn main() {
    p1();
    p2();
}
