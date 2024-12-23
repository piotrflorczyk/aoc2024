use std::collections::HashMap;

struct Ctx {
    numpad: HashMap<char, (isize, isize)>,
    dirpad: HashMap<char, (isize, isize)>,
    num_levels: u8,
}

impl Ctx {
    fn init(num_levels: u8) -> Self {
        Self {
            numpad: HashMap::from([
                ('7', (0, 0)),
                ('8', (0, 1)),
                ('9', (0, 2)),
                ('4', (1, 0)),
                ('5', (1, 1)),
                ('6', (1, 2)),
                ('1', (2, 0)),
                ('2', (2, 1)),
                ('3', (2, 2)),
                ('0', (3, 1)),
                ('A', (3, 2)),
            ]),
            dirpad: HashMap::from([
                ('^', (0, 1)),
                ('A', (0, 2)),
                ('<', (1, 0)),
                ('v', (1, 1)),
                ('>', (1, 2)),
            ]),
            num_levels,
        }
    }
}

fn get_pos(ch: char, lvl: u8, ctx: &Ctx) -> &(isize, isize) {
    if lvl == 0 {
        ctx.numpad.get(&ch).unwrap()
    } else {
        ctx.dirpad.get(&ch).unwrap()
    }
}

fn mov(from: char, to: char, lvl: u8, ctx: &Ctx, memo: &mut HashMap<(char, char, u8), u64>) -> u64 {
    if lvl == ctx.num_levels - 1 {
        return 1;
    }
    if memo.contains_key(&(from, to, lvl)) {
        return memo[&(from, to, lvl)];
    }
    let from_pos = get_pos(from, lvl, ctx);
    let to_pos = get_pos(to, lvl, ctx);
    let paths = get_paths(from_pos, to_pos, if lvl == 0 { (3, 0) } else { (0, 0) });

    let seq_len = paths
        .iter()
        .map(|path| {
            path.chars()
                .fold(('A', 0), |(prev, acc), ch| {
                    (ch, acc + mov(prev, ch, lvl + 1, &ctx, memo))
                })
                .1
        })
        .min()
        .unwrap();
    memo.insert((from, to, lvl), seq_len);
    seq_len
}

fn get_paths(
    (from_r, from_c): &(isize, isize),
    (to_r, to_c): &(isize, isize),
    (dead_r, dead_c): (isize, isize),
) -> Vec<String> {
    let mut paths = vec![];
    let row_diff = to_r - from_r;
    let col_diff = to_c - from_c;
    let row_mov = vec![if row_diff < 0 { '^' } else { 'v' }; row_diff.unsigned_abs()];
    let col_mov = vec![if col_diff < 0 { '<' } else { '>' }; col_diff.unsigned_abs()];
    if *from_c != dead_c || from_r + row_diff != dead_r {
        paths.push(row_mov.iter().collect::<String>() + &col_mov.iter().collect::<String>() + "A");
    }
    if *from_r != dead_r || from_c + col_diff != dead_c {
        paths.push(col_mov.iter().collect::<String>() + &row_mov.iter().collect::<String>() + "A");
    }
    paths
}

fn p1_p2(num_levels: u8) {
    let codes = include_str!("../../input/day21")
        .lines()
        .collect::<Vec<_>>();

    let ctx = Ctx::init(num_levels);
    let mut memo: HashMap<(char, char, u8), u64> = HashMap::new();
    let ans = codes
        .iter()
        .map(|&code| {
            let seq_len = code
                .chars()
                .fold(('A', 0), |(prev, acc), ch| {
                    (ch, acc + mov(prev, ch, 0, &ctx, &mut memo))
                })
                .1;
            code[..code.len() - 1].parse::<u64>().unwrap() * seq_len
        })
        .sum::<u64>();

    println!("P1/2: {ans}");
}

fn main() {
    p1_p2(4);
    p1_p2(27);
}
