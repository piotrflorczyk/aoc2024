fn p1() {
    let p1 = include_str!("../../input/day2")
        .lines()
        .map(|line| {
            let mut elements = line.split_whitespace().map(|el| el.parse::<i32>().unwrap());
            // get the first element of the row
            let mut prev = elements.next().unwrap();
            let mut dir = 0;
            elements.all(|el| {
                // save previous direction either positive or negative or 0 in the begining
                let prev_dir = dir;
                // set direction to previous element - current element
                dir = prev - el;
                prev = el;
                // check if distance is in <1; 3> and we are going in the same direction (prev_dir * dir) isn't negative
                dir.abs() >= 1 && dir.abs() <= 3 && prev_dir * dir >= 0
            }) as i32
        })
        .sum::<i32>();
    println!("P1: {p1:?}");
}

fn all_must_match<'a>(mut iterator: impl Iterator<Item = &'a i32>, dir: i32) -> bool {
    let mut prev = *iterator.next().unwrap();
    iterator.all(|&el| {
        let diff = prev - el;
        prev = el;
        diff.abs() >= 1 && diff.abs() <= 3 && diff * dir > 0
    })
}

fn match_with_one_error(elements: &[i32], dir: i32) -> bool {
    let mut prev = elements[0];
    for (idx, &el) in elements.iter().enumerate().skip(1) {
        let diff = prev - el;
        prev = el;
        // if we find an error, check if it would be fine without current or previous element
        if !(diff.abs() >= 1 && diff.abs() <= 3 && diff * dir >= 0) {
            return all_must_match(
                elements
                    .iter()
                    .take(idx)
                    .chain(elements.iter().skip(idx + 1)),
                dir,
            ) || all_must_match(
                elements
                    .iter()
                    .take(idx - 1)
                    .chain(elements.iter().skip(idx)),
                dir,
            );
        }
    }
    true
}

fn p2() {
    let p2 = include_str!("../../input/day2")
        .lines()
        .map(|line| {
            let elements = line
                .split_whitespace()
                .map(|el| el.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (match_with_one_error(&elements, -1) || match_with_one_error(&elements, 1)) as i32
        })
        .sum::<i32>();
    println!("P2: {p2:?}");
}

fn main() {
    p1();
    p2();
}
