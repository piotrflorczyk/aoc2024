fn p1() {
    let seeds = include_str!("../../input/day22")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let p1 = seeds
        .iter()
        .map(|&seed| {
            let mut secret_number = seed;
            for _ in 0..2000 {
                secret_number ^= secret_number << 6;
                secret_number &= 0xFFFFFF;
                secret_number ^= secret_number >> 5;
                secret_number &= 0xFFFFFF;
                secret_number ^= secret_number << 11;
                secret_number &= 0xFFFFFF;
            }
            secret_number
        })
        .sum::<u64>();

    println!("P1: {p1}");
}

fn p2() {
    let seeds = include_str!("../../input/day22")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut sold = vec![0; 0xFFFFF];
    seeds.iter().for_each(|&seed| {
        let mut visited = vec![false; 0xFFFFF];
        let mut prev = seed;
        let mut diff_key = 0;
        for i in 0..2000 {
            let mut secret_number = prev;
            secret_number ^= secret_number << 6;
            secret_number &= 0xFFFFFF;
            secret_number ^= secret_number >> 5;
            secret_number &= 0xFFFFFF;
            secret_number ^= secret_number << 11;
            secret_number &= 0xFFFFFF;

            let diff = 9 + (secret_number % 10) as i64 - (prev % 10) as i64;
            diff_key = ((diff_key << 5) | diff) & 0xFFFFF;
            prev = secret_number;

            if i >= 3 {
                if !visited[diff_key as usize] {
                    visited[diff_key as usize] = true;
                    sold[diff_key as usize] += secret_number % 10;
                }
            }
        }
    });
    let p2 = sold.iter().max().unwrap();
    println!("P2: {p2}");
}

fn main() {
    p1();
    p2();
}
