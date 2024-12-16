fn p1() {
    let mut disk: Vec<u64> = Vec::new();
    let mut is_file = true;
    let mut ctr = 0;
    include_str!("../../input/day9").chars().for_each(|ch| {
        let file_id = if is_file {
            ctr += 1;
            ctr
        } else {
            0
        };
        disk.extend(std::iter::repeat(file_id).take(ch.to_digit(10).unwrap() as usize));
        is_file = !is_file;
    });

    let mut free_idx = 0;
    let mut last_file_idx = disk.len() - 1;

    while free_idx < last_file_idx {
        if disk[free_idx] != 0 {
            free_idx += 1;
        } else if disk[last_file_idx] == 0 {
            last_file_idx -= 1;
        } else {
            disk.swap(free_idx, last_file_idx);
            free_idx += 1;
            last_file_idx -= 1;
        }
    }

    let p1 = disk
        .iter()
        .enumerate()
        .filter(|(_, &el)| el != 0)
        .map(|(idx, &id)| idx as u64 * (id - 1))
        .sum::<u64>();

    println!("p1: {p1}");
}

#[derive(Debug, Copy, Clone)]
struct BlockInfo {
    id: u64,
    start: u64,
    len: u64,
}

fn p2() {
    let mut file_list: Vec<BlockInfo> = Vec::new();
    let mut free_list: Vec<BlockInfo> = Vec::new();

    let mut is_file = true;
    let mut id = 0;
    let mut curr_idx = 0;
    include_str!("../../input/day9").chars().for_each(|ch| {
        let repeats = ch.to_digit(10).unwrap() as u64;
        if repeats != 0 {
            if is_file {
                file_list.push(BlockInfo {
                    id,
                    start: curr_idx,
                    len: repeats,
                });
                id += 1;
            } else {
                free_list.push(BlockInfo {
                    id: 0,
                    start: curr_idx,
                    len: repeats,
                });
            }
            curr_idx += repeats;
        }
        is_file = !is_file;
    });

    file_list.iter_mut().rev().for_each(|file_block| {
        if let Some(free_idx) = free_list.iter().position(|free_block| {
            free_block.start < file_block.start && free_block.len >= file_block.len
        }) {
            file_block.start = free_list[free_idx].start;
            free_list[free_idx].len -= file_block.len;
            free_list[free_idx].start += file_block.len;
        }
    });
    let p2 = file_list
        .iter()
        .map(|block| (block.start..block.start + block.len).sum::<u64>() * block.id)
        .sum::<u64>();
    println!("p2: {p2}");
}

fn main() {
    p1();
    p2();
}
