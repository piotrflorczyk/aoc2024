use std::collections::HashMap;

fn p1() {
    let mut total_score = 0;

    let grid = include_str!("../../input/day12")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    // step through the whole grid, but do it by region
    for i in 0..grid.len() as isize {
        for j in 0..grid[i as usize].len() as isize {
            // if cell was visited previously, skip
            if visited[i as usize][j as usize] {
                continue;
            }

            // cell was not visited, so it's a start of a new region, mark it as a begining in queue
            let mut area = 0;
            let mut perimeter = 0;
            let mut queue = vec![(i, j)];
            visited[i as usize][j as usize] = true;
            // process each cell of a single region
            while let Some((row, col)) = queue.pop() {
                area += 1;
                // assume we are surrounded by cells from different regions, if that's not the case decrement in neighbour search
                perimeter += 4;
                // for each neighbour with the same id add it to the queueu for processing
                [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)]
                    .iter()
                    .for_each(|(drow, dcol)| {
                        if row + drow >= 0
                            && row + drow < grid.len() as isize
                            && col + dcol >= 0
                            && col + dcol < grid[row as usize].len() as isize
                            && grid[(row + drow) as usize][(col + dcol) as usize]
                                == grid[row as usize][col as usize]
                        {
                            perimeter -= 1;
                            if !visited[(row + drow) as usize][(col + dcol) as usize] {
                                visited[(row + drow) as usize][(col + dcol) as usize] = true;
                                queue.push((row + drow, col + dcol));
                            }
                        }
                    })
            }
            total_score += area * perimeter;
        }
    }
    println!("P1: {total_score}");
}

fn p2() {
    let mut total_score: isize = 0;

    let grid = include_str!("../../input/day12")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    // step through the whole grid, but do it by region
    for i in 0..grid.len() as isize {
        for j in 0..grid[i as usize].len() as isize {
            // if cell was visited previously, skip
            if visited[i as usize][j as usize] {
                continue;
            }

            // cell was not visited, so it's a start of a new region, mark it as a begining in queue
            let mut area: isize = 0;
            let mut perimeters: HashMap<(isize, isize, isize), Vec<isize>> = HashMap::new();

            let mut queue = vec![(i, j)];
            visited[i as usize][j as usize] = true;
            // process each cell of a single region
            while let Some((row, col)) = queue.pop() {
                area += 1;
                // for each neighbour with the same id add it to the queueu for processing
                [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)]
                    .iter()
                    .for_each(|&(drow, dcol)| {
                        if row + drow >= 0
                            && row + drow < grid.len() as isize
                            && col + dcol >= 0
                            && col + dcol < grid[row as usize].len() as isize
                            && grid[(row + drow) as usize][(col + dcol) as usize]
                                == grid[row as usize][col as usize]
                        {
                            // add not visited neighbours with the same region id
                            if !visited[(row + drow) as usize][(col + dcol) as usize] {
                                visited[(row + drow) as usize][(col + dcol) as usize] = true;
                                queue.push((row + drow, col + dcol));
                            }
                        } else {
                            // if neighbour has different id that means there is a perimeter add coordinates to a list
                            // bucketize it by direction and relevant coordinate
                            if drow != 0 {
                                perimeters
                                    .entry((drow, dcol, row + drow))
                                    .or_default()
                                    .push(col + dcol);
                            } else {
                                perimeters
                                    .entry((drow, dcol, col + dcol))
                                    .or_default()
                                    .push(row + drow);
                            }
                        }
                    });
            }
            total_score += area
                * perimeters
                    .values_mut()
                    .map(|v| {
                        // sort list of the perimeter elements with the same coordinate
                        v.sort();
                        // find where the continuity is broken
                        v.iter()
                            .zip(v.iter().skip(1))
                            .filter(|&(&prev, &curr)| curr != prev + 1)
                            .count() as isize
                            + 1
                    })
                    .sum::<isize>();
        }
    }
    println!("P1: {total_score}");
}

fn main() {
    p1();
    p2();
}
