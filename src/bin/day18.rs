use std::collections::{BinaryHeap, HashSet};

type Pos = (isize, isize);

fn p1() {
    let (size, bytes) = (71, 1024);

    let blocks = include_str!("../../input/day18")
        .lines()
        .take(bytes)
        .map(|line| {
            let (c_str, r_str) = line.split_once(',').unwrap();
            (
                r_str.parse::<isize>().unwrap(),
                c_str.parse::<isize>().unwrap(),
            )
        })
        .collect::<HashSet<_>>();

    let mut heap: BinaryHeap<(i64, Pos)> = BinaryHeap::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut p1 = 0;
    heap.push((0, (0, 0)));
    while let Some((score, (r, c))) = heap.pop() {
        if !visited.insert((r, c)) {
            continue;
        }
        if (r, c) == (size - 1, size - 1) {
            p1 = -score;
            break;
        }
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .for_each(|&(dr, dc)| {
                if (r + dr) >= 0
                    && (r + dr) < size
                    && (c + dc) >= 0
                    && (c + dc) < size
                    && !blocks.contains(&(r + dr, c + dc))
                {
                    heap.push((score - 1, (r + dr, c + dc)));
                }
            });
    }
    println!("P1: {p1}");
}

fn p2() {
    let (size, bytes) = (71, 1024);

    let all_blocks = include_str!("../../input/day18")
        .lines()
        .map(|line| {
            let (c_str, r_str) = line.split_once(',').unwrap();
            (
                r_str.parse::<isize>().unwrap(),
                c_str.parse::<isize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut blocks: HashSet<Pos> = all_blocks
        .iter()
        .cloned()
        .take(bytes)
        .collect::<HashSet<_>>();
    let mut p2 = (0, 0);
    // TODO: change this to bisect
    for block in all_blocks.iter().skip(bytes + 1) {
        blocks.insert(*block);

        let mut heap = vec![(0, 0)];
        let mut visited: HashSet<Pos> = HashSet::new();
        while let Some((r, c)) = heap.pop() {
            if !visited.insert((r, c)) {
                continue;
            }
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .for_each(|&(dr, dc)| {
                    if (r + dr) >= 0
                        && (r + dr) < size
                        && (c + dc) >= 0
                        && (c + dc) < size
                        && !blocks.contains(&(r + dr, c + dc))
                        && !visited.contains(&(r + dr, c + dc))
                    {
                        heap.push((r + dr, c + dc));
                    }
                });
        }
        if !visited.contains(&(size - 1, size - 1)) {
            p2 = (block.1, block.0);
            break;
        }
    }
    println!("P2: {p2:?}");
}

fn main() {
    p1();
    p2();
}
