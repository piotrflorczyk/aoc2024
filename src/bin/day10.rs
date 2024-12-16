use std::collections::HashSet;

fn p1() {
    let grid = include_str!("../../input/day10")
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut queue = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, cell)| {
                if *cell == 0 {
                    Some((i as isize, j as isize))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let mut cnt = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    while let Some((row, col)) = queue.pop() {
        let curr_level = grid[row as usize][col as usize];
        if curr_level == 0 {
            visited.clear();
        }
        visited.insert((row, col));
        if curr_level == 9 {
            cnt += 1;
        } else {
            [(-1, 0), (1, 0), (0, -1), (0isize, 1isize)]
                .iter()
                .for_each(|(drow, dcol)| {
                    if row + drow >= 0
                        && row + drow < grid.len() as isize
                        && col + dcol >= 0
                        && col + dcol < grid[row as usize].len() as isize
                        && grid[(row + drow) as usize][(col + dcol) as usize] == curr_level + 1
                        && !visited.contains(&(row + drow, col + dcol))
                    {
                        queue.push((row + drow, col + dcol));
                    }
                });
        }
    }
    println!("P1: {cnt}");
}

fn p2() {
    let grid = include_str!("../../input/day10")
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut queue = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, cell)| {
                if *cell == 0 {
                    Some((i as isize, j as isize))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let mut cnt = 0;

    while let Some((row, col)) = queue.pop() {
        let curr_level = grid[row as usize][col as usize];
        if curr_level == 9 {
            cnt += 1;
        } else {
            [(-1, 0), (1, 0), (0, -1), (0isize, 1isize)]
                .iter()
                .for_each(|(drow, dcol)| {
                    if row + drow >= 0
                        && row + drow < grid.len() as isize
                        && col + dcol >= 0
                        && col + dcol < grid[row as usize].len() as isize
                        && grid[(row + drow) as usize][(col + dcol) as usize] == curr_level + 1
                    {
                        queue.push((row + drow, col + dcol));
                    }
                });
        }
    }
    println!("P2: {cnt}");
}

fn main() {
    p1();
    p2();
}
