use core::panic;
use std::collections::{HashSet, VecDeque};

const DELIMITER: &str = "\r\n\r\n";

fn p1() {
    let (mut x, mut y) = (0, 0);
    let (grid_str, moves_str) = include_str!("../../input/day15")
        .split_once(DELIMITER)
        .unwrap();
    let mut grid = grid_str
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if let Some(j) = line.find('@') {
                (x, y) = (i as isize, j as isize);
            }
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let moves = moves_str
        .chars()
        .filter(|&ch| ch != '\r' && ch != '\n')
        .collect::<Vec<_>>();

    for mov in moves {
        let d = match mov {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!("WTF"),
        };
        let mut dist = 1;
        while grid[(x + dist * d.0) as usize][(y + dist * d.1) as usize] == 'O' {
            dist += 1;
        }
        if grid[(x + dist * d.0) as usize][(y + dist * d.1) as usize] == '.' {
            while dist != 0 {
                grid[(x + dist * d.0) as usize][(y + dist * d.1) as usize] =
                    grid[(x + (dist - 1) * d.0) as usize][(y + (dist - 1) * d.1) as usize];
                dist -= 1;
            }
            grid[x as usize][y as usize] = '.';
            (x, y) = (x + d.0, y + d.1);
        }
    }
    let p1 = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &ch)| ch == 'O')
                .map(|(j, _)| i * 100 + j)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("P1: {p1}");
}

fn p2() {
    let (grid_str, moves_str) = include_str!("../../input/day15")
        .split_once(DELIMITER)
        .unwrap();
    let mut grid = grid_str
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|ch| match ch {
                    '#' => "##".chars(),
                    'O' => "[]".chars(),
                    '.' => "..".chars(),
                    '@' => "@.".chars(),
                    _ => panic!("WTF"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let moves = moves_str
        .chars()
        .filter(|&ch| ch != '\r' && ch != '\n')
        .collect::<Vec<_>>();

    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row.iter()
                .position(|&ch| ch == '@')
                .map(|j| (i as isize, j as isize))
        })
        .next()
        .unwrap();

    for mov in moves {
        let (dx, dy) = match mov {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!("WTF"),
        };

        let mut queue = VecDeque::from([(x, y)]);
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut to_move = vec![(x, y)];
        while let Some((cell_x, cell_y)) = queue.pop_front() {
            if !visited.insert((cell_x, cell_y)) {
                continue;
            } else {
                to_move.push((cell_x, cell_y));
            }
            let (next_x, next_y) = (cell_x + dx, cell_y + dy);
            match grid[next_x as usize][next_y as usize] {
                '#' => {
                    to_move.clear();
                    break;
                }
                '[' => queue.extend([(next_x, next_y), (next_x, next_y + 1)]),
                ']' => queue.extend([(next_x, next_y), (next_x, next_y - 1)]),
                _ => continue,
            }
        }
        to_move.iter().rev().for_each(|&(cell_x, cell_y)| {
            grid[(cell_x + dx) as usize][(cell_y + dy) as usize] =
                grid[cell_x as usize][cell_y as usize];
            grid[cell_x as usize][cell_y as usize] = '.';
        });
        if !to_move.is_empty() {
            grid[x as usize][y as usize] = '.';
            (x, y) = (x + dx, y + dy);
        }
    }

    let p2 = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &ch)| ch == '[')
                .map(|(j, _)| i * 100 + j)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("P2: {p2}");
}

fn main() {
    p1();
    p2();
}
