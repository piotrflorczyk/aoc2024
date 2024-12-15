fn p1() {
    let equations = include_str!("../../input/day7")
        .lines()
        .map(|line| {
            let (res_str, num_str) = line.split_once(": ").unwrap();
            let res = res_str.parse::<u64>().unwrap();
            let nums = num_str
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (res, nums)
        })
        .collect::<Vec<_>>();
    let p1 = equations
        .iter()
        .filter(|(res, nums)| {
            let mut queue = vec![(nums[0], '+', 0), (nums[0], '*', 0)];

            while let Some((tmp_res, op, idx)) = queue.pop() {
                if idx + 1 >= nums.len() {
                    if tmp_res == *res {
                        return true;
                    }
                    continue;
                }
                let tmp = match op {
                    '+' => tmp_res + nums[idx + 1],
                    '*' => tmp_res * nums[idx + 1],
                    _ => panic!("WTF"),
                };
                if tmp <= *res {
                    queue.push((tmp, '+', idx + 1));
                    queue.push((tmp, '*', idx + 1));
                }
            }
            false
        })
        .map(|(res, _)| res)
        .sum::<u64>();
    println!("P1: {p1}");
}

fn get_multiplier(mut num: u64) -> u64 {
    let mut mul = 1;
    while num != 0 {
        num /= 10;
        mul *= 10;
    }
    mul
}

fn p2() {
    let equations = include_str!("../../input/day7")
        .lines()
        .map(|line| {
            let (res_str, num_str) = line.split_once(": ").unwrap();
            let res = res_str.parse::<u64>().unwrap();
            let nums = num_str
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (res, nums)
        })
        .collect::<Vec<_>>();
    let p2 = equations
        .iter()
        .filter(|(res, nums)| {
            let mut queue = vec![(nums[0], '+', 0), (nums[0], '*', 0), (nums[0], '|', 0)];

            while let Some((tmp_res, op, idx)) = queue.pop() {
                if idx + 1 >= nums.len() {
                    if tmp_res == *res {
                        return true;
                    }
                    continue;
                }
                let tmp = match op {
                    '+' => tmp_res + nums[idx + 1],
                    '*' => tmp_res * nums[idx + 1],
                    '|' => tmp_res * get_multiplier(nums[idx + 1]) + nums[idx + 1],
                    _ => panic!("WTF"),
                };
                if tmp <= *res {
                    queue.push((tmp, '+', idx + 1));
                    queue.push((tmp, '*', idx + 1));
                    queue.push((tmp, '|', idx + 1));
                }
            }
            false
        })
        .map(|(res, _)| res)
        .sum::<u64>();
    println!("P2: {p2}");
}

fn main() {
    p1();
    p2();
}
