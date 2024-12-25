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

        let code = lines
            .iter()
            .skip(1)
            .fold(vec![0; lines[0].len()], |mut code, line| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '#')
                    .for_each(|(idx, _)| {
                        code[idx] += 1;
                    });
                code
            });
        if is_key {
            keys.push(code);
        } else {
            locks.push(code);
        }
    });

    let p1 = keys
        .iter()
        .map(|key_code| {
            locks
                .iter()
                .filter(|&lock_code| {
                    key_code
                        .iter()
                        .zip(lock_code.iter())
                        .all(|(x, y)| x + y <= 5)
                })
                .count()
        })
        .sum::<usize>();

    println!("P1: {p1}")
}

fn main() {
    p1();
}
