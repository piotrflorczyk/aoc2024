use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Robot {
    pos: (isize, isize),
    velocity: (isize, isize),
}

fn simulate(robot: &Robot, sec: isize, w: isize, h: isize) -> (isize, isize) {
    let mut new_x = (robot.pos.0 + robot.velocity.0 * sec) % w;
    let mut new_y = (robot.pos.1 + robot.velocity.1 * sec) % h;

    if new_x < 0 {
        new_x += w;
    }
    if new_y < 0 {
        new_y += h;
    }
    (new_x, new_y)
}

fn p1() {
    let robots = include_str!("../../input/day14")
        .lines()
        .map(|line| {
            let (p_str, v_str) = line.split_once(' ').unwrap();
            let pos = p_str[2..].split_once(',').unwrap();
            let v = v_str[2..].split_once(',').unwrap();
            Robot {
                pos: (
                    pos.0.parse::<isize>().unwrap(),
                    pos.1.parse::<isize>().unwrap(),
                ),
                velocity: (v.0.parse::<isize>().unwrap(), v.1.parse::<isize>().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    let height = 103;
    let width = 101;

    let transformed_robots = robots
        .iter()
        .map(|robot| simulate(robot, 100, width, height))
        .collect::<Vec<_>>();

    let mut counts = [0; 4];
    transformed_robots.iter().for_each(|&(x, y)| {
        if x != width / 2 && y != height / 2 {
            let q1 = x / ((width / 2) + 1);
            let q2 = y / ((height / 2) + 1);
            counts[(q1 + 2 * q2) as usize] += 1;
        }
    });

    let p1 = counts.iter().product::<i32>();
    println!("P1: {p1:?}");
}

fn p2() {
    let robots = include_str!("../../input/day14")
        .lines()
        .map(|line| {
            let (p_str, v_str) = line.split_once(' ').unwrap();
            let pos = p_str[2..].split_once(',').unwrap();
            let v = v_str[2..].split_once(',').unwrap();
            Robot {
                pos: (
                    pos.0.parse::<isize>().unwrap(),
                    pos.1.parse::<isize>().unwrap(),
                ),
                velocity: (v.0.parse::<isize>().unwrap(), v.1.parse::<isize>().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    let height = 103;
    let width = 101;

    for i in 0..10000 {
        let transformed_robots = robots
            .iter()
            .map(|robot| simulate(robot, i, width, height))
            .collect::<HashSet<_>>();

        if transformed_robots.len() == robots.len() {
            println!("P2: {i}");
            break;
        }
    }
}

fn main() {
    p1();
    p2();
}
