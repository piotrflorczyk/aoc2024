const DELIMITER: &str = "\r\n\r\n";

fn p1() {
    let patterns = include_str!("../../input/day25")
        .split(DELIMITER)
        .collect::<Vec<_>>();

    let mut keys = vec![];
    let mut locks = vec![];
    patterns.iter().for_each(|p| {
        let mut lines = p.lines().collect::<Vec<_>>();
        let is_key = lines[0].chars().all(|ch| ch == '#');
        if !is_key {
            lines.reverse();
        }

        let mut code = vec![0; lines[0].len()];
        for i in 1..lines.len() {
            lines[i]
                .chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .for_each(|(idx, _)| {
                    code[idx] += 1;
                });
        }
        if is_key {
            keys.push(code);
        } else {
            locks.push(code);
        }
    });

    let mut p1 = 0;
    for key_code in &keys {
        for lock_code in &locks {
            if key_code
                .iter()
                .zip(lock_code.iter())
                .all(|(x, y)| x + y <= 5)
            {
                p1 += 1;
            }
        }
    }
    println!("P1: {p1}")
}

fn main() {
    p1();
}
