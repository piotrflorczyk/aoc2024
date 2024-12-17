const DELIMITER: &str = "\r\n\r\n";

fn dir(ch: char) -> (isize, isize) {
    match ch {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => panic!("WTF"),
    }
}

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
        let d = dir(mov);
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

fn main() {
    p1();
}
