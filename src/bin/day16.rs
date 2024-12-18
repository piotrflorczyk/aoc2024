use std::collections::{BinaryHeap, HashMap, HashSet};

type Pos = (isize, isize);

fn p1_p2() {
    let grid = include_str!("../../input/day16")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_r, start_c) = (grid.len() as isize - 2, 1isize);
    let (end_r, end_c) = (1isize, grid[0].len() as isize - 2);

    // store: score, curr pos, direction, traveled_path
    let mut heap: BinaryHeap<(i64, Pos, Pos, Vec<Pos>)> = BinaryHeap::new();
    // maps (cell, dir) -> best score
    let mut visited: HashMap<(Pos, Pos), i64> = HashMap::new();
    let mut best_score = 999999999;
    let mut paths: Vec<Vec<Pos>> = Vec::new();
    heap.push((0, (start_r, start_c), (0, 1), Vec::new()));
    while let Some((score, (r, c), (dr, dc), mut path)) = heap.pop() {
        if visited.contains_key(&((r, c), (dr, dc))) && visited[&((r, c), (dr, dc))] < -score {
            continue;
        }
        visited.insert(((r, c), (dr, dc)), -score);
        if (r, c) == (end_r, end_c) {
            if -score <= best_score {
                best_score = -score;
                paths.push(path.clone());
            } else {
                break;
            }
        }
        path.push((r, c));
        if grid[(r - dc) as usize][(c - dr) as usize] != '#' {
            heap.push((score - 1001, (r - dc, c - dr), (-dc, -dr), path.clone()));
        }
        if grid[(r + dc) as usize][(c + dr) as usize] != '#' {
            heap.push((score - 1001, (r + dc, c + dr), (dc, dr), path.clone()));
        }
        if grid[(r + dr) as usize][(c + dc) as usize] != '#' {
            heap.push((score - 1, (r + dr, c + dc), (dr, dc), path.clone()));
        }
    }

    let mut unique_tiles: HashSet<Pos> = HashSet::new();
    paths.iter().for_each(|p| {
        unique_tiles.extend(p.iter());
    });

    let p2 = unique_tiles.len() + 1; // Add 'E'
    println!("P1: {best_score}");
    println!("P2: {p2}");
}

fn main() {
    p1_p2();
}
