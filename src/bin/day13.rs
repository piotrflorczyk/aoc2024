use regex::Regex;
const DELIMITER: &str = "\r\n\r\n";

#[derive(Debug, Clone, Copy)]
struct Puzzle {
    mov_button_a: (isize, isize),
    mov_button_b: (isize, isize),
    prize: (isize, isize),
}

fn solve(mov: isize) {
    let puzzles = include_str!("../../input/day13")
        .split(DELIMITER)
        .map(|s| {
            let re = Regex::new(r"\d+").unwrap();
            let all_nums = re
                .find_iter(s)
                .map(|m| m.as_str().parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            assert!(all_nums.len() == 6);
            Puzzle {
                mov_button_a: (all_nums[0], all_nums[1]),
                mov_button_b: (all_nums[2], all_nums[3]),
                prize: (mov + all_nums[4], mov + all_nums[5]),
            }
        })
        .collect::<Vec<_>>();

    let p1 = puzzles
        .iter()
        .map(|puzzle| {
            let denominator = puzzle.mov_button_a.0 * puzzle.mov_button_b.1
                - puzzle.mov_button_a.1 * puzzle.mov_button_b.0;
            if denominator != 0 {
                let numerator =
                    puzzle.prize.1 * puzzle.mov_button_a.0 - puzzle.prize.0 * puzzle.mov_button_a.1;
                if numerator % denominator == 0 {
                    let b_count = numerator / denominator;
                    let numerator_a = puzzle.prize.0 - puzzle.mov_button_b.0 * b_count;
                    if numerator_a % puzzle.mov_button_a.0 == 0 {
                        let a_count = numerator_a / puzzle.mov_button_a.0;
                        return 3 * a_count + b_count;
                    }
                }
            }
            0
        })
        .sum::<isize>();

    println!("Res: {p1}");
}

fn main() {
    solve(0);
    solve(10000000000000);
}
