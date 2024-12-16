use std::collections::{HashMap, HashSet};

fn check_in_grid((x, y): (isize, isize), (rows, cols): (isize, isize)) -> bool {
    x >= 0 && x < rows && y >= 0 && y < cols
}

fn p1() {
    let mut anthenas_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let input = include_str!("../../input/day8");
    input.lines().enumerate().for_each(|(i, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, ch)| *ch != '.')
            .for_each(|(j, ch)| {
                anthenas_map
                    .entry(ch)
                    .or_default()
                    .push((i as isize, j as isize));
            });
    });
    let (rows, cols) = (
        input.lines().count() as isize,
        input.lines().next().unwrap().len() as isize,
    );

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    anthenas_map.values().for_each(|anthenas| {
        anthenas
            .iter()
            .enumerate()
            .for_each(|(i, (first_row, first_col))| {
                anthenas
                    .iter()
                    .skip(i + 1)
                    .for_each(|(second_row, second_col)| {
                        let (drow, dcol) = (first_row - second_row, first_col - second_col);
                        if check_in_grid((first_row + drow, first_col + dcol), (rows, cols)) {
                            antinodes.insert((first_row + drow, first_col + dcol));
                        }
                        if check_in_grid((second_row - drow, second_col - dcol), (rows, cols)) {
                            antinodes.insert((second_row - drow, second_col - dcol));
                        }
                    });
            });
    });

    let p1 = antinodes.len();
    println!("P1: {p1}");
}

fn p2() {
    let mut anthenas_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let input = include_str!("../../input/day8");
    input.lines().enumerate().for_each(|(i, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, ch)| *ch != '.')
            .for_each(|(j, ch)| {
                anthenas_map
                    .entry(ch)
                    .or_default()
                    .push((i as isize, j as isize));
            });
    });
    let (rows, cols) = (
        input.lines().count() as isize,
        input.lines().next().unwrap().len() as isize,
    );

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    anthenas_map.values().for_each(|anthenas| {
        anthenas
            .iter()
            .enumerate()
            .for_each(|(i, (first_row, first_col))| {
                anthenas
                    .iter()
                    .skip(i + 1)
                    .for_each(|(second_row, second_col)| {
                        let (drow, dcol) = (first_row - second_row, first_col - second_col);

                        for dir in [-1, 1] {
                            for (row, col) in [(first_row, first_col), (second_row, second_col)] {
                                let mut ctr = 1;
                                while check_in_grid(
                                    (row + dir * ctr * drow, col + dir * ctr * dcol),
                                    (rows, cols),
                                ) {
                                    antinodes
                                        .insert((row + dir * ctr * drow, col + dir * ctr * dcol));
                                    ctr += 1;
                                }
                            }
                        }
                    });
            });
    });

    let p2 = antinodes.len();
    println!("P2: {p2}");
}

fn main() {
    p1();
    p2();
}
