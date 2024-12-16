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

fn get_multiplier(num: i64) -> u32 {
    num.ilog10() + 1
}

fn ends_with(res: i64, num: i64) -> bool {
    let multiplier = get_multiplier(num);
    res % 10i64.pow(multiplier) == num
}

fn p2() {
    let equations = include_str!("../../input/day7")
        .lines()
        .map(|line| {
            let (res_str, num_str) = line.split_once(": ").unwrap();
            let res = res_str.parse::<i64>().unwrap();
            let nums = num_str
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (res, nums)
        })
        .collect::<Vec<_>>();
    let p2 = equations
        .iter()
        .filter(|(res, nums)| {
            let mut queue = vec![(*res, (nums.len() - 1) as i64)];

            while let Some((tmp_res, idx)) = queue.pop() {
                if idx == 0 {
                    if tmp_res == nums[idx as usize] {
                        return true;
                    }
                    continue;
                }
                if tmp_res >= nums[idx as usize] {
                    queue.push((tmp_res - nums[idx as usize], idx - 1));
                }
                if tmp_res % nums[idx as usize] == 0 {
                    queue.push((tmp_res / nums[idx as usize], idx - 1));
                }
                if ends_with(tmp_res, nums[idx as usize]) {
                    queue.push((
                        tmp_res / 10i64.pow(get_multiplier(nums[idx as usize])),
                        idx - 1,
                    ));
                }
            }
            false
        })
        .map(|(res, _)| res)
        .sum::<i64>();
    println!("P2: {p2}");
}

fn main() {
    p1();
    p2();
}
