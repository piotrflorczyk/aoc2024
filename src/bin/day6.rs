const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn next_pos((cur_x, cur_y): (isize, isize), dir: usize) -> (isize, isize) {
    (cur_x + DIRS[dir].0, cur_y + DIRS[dir].1)
}

fn p1() {
    let mut grid: Vec<Vec<char>> = include_str!("../../input/day6")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(
                move |(j, &value)| {
                    if value == '^' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .next()
        .unwrap();

    let mut curr_dir = 0;
    loop {
        let (next_x, next_y) = next_pos((x as isize, y as isize), curr_dir);
        if next_x < 0
            || next_x >= grid.len() as isize
            || next_y < 0
            || next_y >= grid[0].len() as isize
        {
            break;
        }
        if grid[next_x as usize][next_y as usize] == '#' {
            curr_dir = (curr_dir + 1) % 4;
        } else {
            (x, y) = (next_x as usize, next_y as usize);
            grid[x][y] = 'x';
        }
    }
    let visited = grid
        .iter()
        .map(|row| row.iter().filter(|&&ch| ch != '#' && ch != '.').count())
        .sum::<usize>();
    println!("P1: {visited:?}");
}

// We check if we are in the loop by running the simulation and checking whether we will step out of the grid or encounter the same path
fn is_looping(orig_grid: &[Vec<u8>], pos: (usize, usize), dir: usize) -> bool {
    let mut grid = orig_grid.to_owned();
    let mut curr_dir = dir;
    let (mut x, mut y) = pos;
    loop {
        let (next_x, next_y) = next_pos((x as isize, y as isize), curr_dir);
        if next_x < 0
            || next_x >= grid.len() as isize
            || next_y < 0
            || next_y >= grid[0].len() as isize
        {
            return false;
        }

        if grid[next_x as usize][next_y as usize] == b'#' {
            // obstacle => change dir
            curr_dir = (curr_dir + 1) % 4;
        } else if grid[next_x as usize][next_y as usize] & (1 << curr_dir) != 0 {
            // visited cell with the same direction => we are in the loop
            return true;
        } else {
            // visited cell with different direction or empty => add new direction
            (x, y) = (next_x as usize, next_y as usize);
            grid[x][y] |= 1 << curr_dir;
        }
    }
}

fn p2() {
    let (mut x, mut y) = (0, 0);
    let mut grid: Vec<Vec<u8>> = include_str!("../../input/day6")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| match ch {
                    '.' => 0,
                    '^' => {
                        (x, y) = (i, j);
                        1
                    }
                    _ => ch as u8,
                })
                .collect()
        })
        .collect();

    let mut curr_dir = 0;
    let mut cnt = 0;
    // simulate original path, and at each step try placing an obstacle and checking if it will cause a loop
    loop {
        let (next_x, next_y) = next_pos((x as isize, y as isize), curr_dir);
        if next_x < 0
            || next_x >= grid.len() as isize
            || next_y < 0
            || next_y >= grid[0].len() as isize
        {
            break;
        }
        if grid[next_x as usize][next_y as usize] == b'#' {
            // obstacle => change dir
            curr_dir = (curr_dir + 1) % 4;
        } else {
            if grid[next_x as usize][next_y as usize] == 0 {
                // empty cell => place an obstacle, check, marke as visited with curr direction
                grid[next_x as usize][next_y as usize] = b'#';
                cnt += (is_looping(&grid, (x, y), curr_dir)) as i32;
                grid[next_x as usize][next_y as usize] = 0;
            }
            (x, y) = (next_x as usize, next_y as usize);
            grid[x][y] = 1 << curr_dir;
        }
    }
    println!("P2: {cnt:?}");
}

fn main() {
    p1();
    p2();
}
