use regex::Regex;

fn p1() {
    let input = include_str!("../../input/day3");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(input);
    let p1 = matches
        .map(|caps| {
            let n1 = caps[1].parse::<i32>().unwrap();
            let n2 = caps[2].parse::<i32>().unwrap();
            n1 * n2
        })
        .sum::<i32>();
    println!("P1: {p1:?}");
}

fn p2() {
    let input = include_str!("../../input/day3");
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let matches = re.captures_iter(input);
    let mut enabled = true;
    let p2 = matches
        .map(|caps| match &caps[0] {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => {
                let n1 = caps[1].parse::<i32>().unwrap();
                let n2 = caps[2].parse::<i32>().unwrap();
                (enabled as i32) * n1 * n2
            }
        })
        .sum::<i32>();
    println!("P2: {p2:?}");
}

fn main() {
    p1();
    p2();
}
