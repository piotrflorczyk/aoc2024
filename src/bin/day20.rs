type Pos = (isize, isize);

fn p1() {
    let (mut curr_r, mut curr_c) = (0, 0);
    let grid = include_str!("../../input/day20")
        .lines()
        .enumerate()
        .map(|(r, line)| {
            if let Some(c) = line.chars().position(|ch| ch == 'S') {
                (curr_r, curr_c) = (r as isize, c as isize);
            }
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut path: Vec<Pos> = vec![(curr_r, curr_c)];
    while grid[curr_r as usize][curr_c as usize] != 'E' {
        for (dr, dc) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            if grid[(curr_r + dr) as usize][(curr_c + dc) as usize] != '#'
                && (path.len() == 1 || path[path.len() - 2] != (curr_r + dr, curr_c + dc))
            {
                path.push((curr_r + dr, curr_c + dc));
                curr_r += dr;
                curr_c += dc;
                break;
            }
        }
    }
    let mut p1 = 0;
    for i in 0..path.len() {
        for j in i..path.len() {
            let (start_idx, (start_r, start_c)) = (i, path[i]);
            let (end_idx, (end_r, end_c)) = (j, path[j]);
            if (end_r - start_r).abs() + (end_c - start_c).abs() == 2
                && end_idx - start_idx - 2 >= 100
            {
                p1 += 1;
            }
        }
    }
    println!("{p1}");
}

fn main() {
    p1();
}
