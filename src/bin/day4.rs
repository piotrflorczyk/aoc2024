fn count_occurances(grid: &[Vec<char>], i: isize, j: isize) -> u32 {
    let dirs = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    dirs.iter()
        .map(|(dx, dy)| {
            (3 * dx + i >= 0
                && 3 * dx + i < grid.len() as isize
                && 3 * dy + j >= 0
                && 3 * dy + j < grid[i as usize].len() as isize
                && grid[(dx + i) as usize][(dy + j) as usize] == 'M'
                && grid[(2 * dx + i) as usize][(2 * dy + j) as usize] == 'A'
                && grid[(3 * dx + i) as usize][(3 * dy + j) as usize] == 'S') as u32
        })
        .sum::<u32>()
}

fn p1() {
    let grid: Vec<Vec<char>> = include_str!("../../input/day4")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let p1 = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &ch)| ch == 'X')
                .map(|(j, _)| count_occurances(&grid, i as isize, j as isize))
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("P1: {p1}");
}

fn x_mas_matches(grid: &[Vec<char>], i: isize, j: isize) -> bool {
    i > 0
        && i + 1 < grid.len() as isize
        && j > 0
        && j + 1 < grid[i as usize].len() as isize
        && ((grid[(i - 1) as usize][(j - 1) as usize] == 'M'
            && grid[(i + 1) as usize][(j + 1) as usize] == 'S')
            || (grid[(i - 1) as usize][(j - 1) as usize] == 'S'
                && grid[(i + 1) as usize][(j + 1) as usize] == 'M'))
        && ((grid[(i - 1) as usize][(j + 1) as usize] == 'M'
            && grid[(i + 1) as usize][(j - 1) as usize] == 'S')
            || (grid[(i - 1) as usize][(j + 1) as usize] == 'S'
                && grid[(i + 1) as usize][(j - 1) as usize] == 'M'))
}

fn p2() {
    let grid: Vec<Vec<char>> = include_str!("../../input/day4")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let p2 = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, &ch)| ch == 'A' && x_mas_matches(&grid, i as isize, *j as isize))
                .count()
        })
        .sum::<usize>();
    println!("P2: {p2}");
}

fn main() {
    p1();
    p2();
}
