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
                let tmp1 = secret_number * 64; // tmp << 6
                secret_number ^= tmp1;
                secret_number &= 0xFFFFFF;
                let tmp2 = secret_number / 32; // tmp >> 5
                secret_number ^= tmp2;
                secret_number &= 0xFFFFFF;
                let tmp3 = secret_number * 2048; // tmp << 10
                secret_number ^= tmp3;
                secret_number &= 0xFFFFFF;
            }
            secret_number
        })
        .sum::<u64>();

    println!("P1: {p1}");
}

fn main() {
    p1();
}
