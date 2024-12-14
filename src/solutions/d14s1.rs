use colored::Colorize;
use regex::Regex;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 14;
const SOL: u8 = 1;

pub const WIDTH: i32 = 101;
pub const HEIGHT: i32 = 103;
// pub const WIDTH: i32 = 11;
// pub const HEIGHT: i32 = 7;
pub const SIMTIME: usize = 100;

#[derive(Clone, Debug)]
pub struct Robot {
    pub pos: (i32, i32),
    pub vel: (i32, i32),
}
impl Robot {
    pub fn new(re: &Regex, input: &str) -> Self {
        // println!("Robot Input:\t\t{input}");
        let caps = re.captures(input).expect("Failed to apply regex");
        let pos = (
            caps[1].parse::<i32>().expect("cap not int"),
            caps[2].parse::<i32>().expect("cap not int"),
        );
        let vel = (
            caps[3].parse::<i32>().expect("cap not int"),
            caps[4].parse::<i32>().expect("cap not int"),
        );
        Self { pos, vel }
    }
    pub fn march(&mut self, w: i32, h: i32) {
        // println!("before\t{:?}", self.pos);
        // println!("applying v={:?}", self.vel);
        self.pos.0 += self.vel.0;
        if self.pos.0 < 0 {
            self.pos.0 += w;
        }
        self.pos.0 = self.pos.0 % (w + 0);
        self.pos.1 += self.vel.1;
        if self.pos.1 < 0 {
            self.pos.1 += h;
        }
        self.pos.1 = self.pos.1 % (h + 0);
        // println!("after \t{:?}", self.pos);
    }
}

pub async fn input(example: bool) -> Vec<Robot> {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    let re = Regex::new(r#"p=(\d+),(\d+) v=([0-9-]+),([0-9-]+)"#).unwrap();

    lines.iter().map(|line| Robot::new(&re, &line)).collect()
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    let mut robots = input.clone();

    for _t in 0..SIMTIME {
        // println!("\nT:{_t}");
        // let _ = robots.iter_mut().map(|robot| robot.march(WIDTH, HEIGHT));
        for robot in robots.iter_mut() {
            robot.march(WIDTH, HEIGHT);
        }
    }

    // let positions: Vec<(i32, i32)> = robots.iter().map(|robot| robot.pos).collect();

    // let mut answer = 0;

    let mut tl = 0;
    // for y in 0..(HEIGHT - 3) / 2 {
    //     for x in 0..=WIDTH / 2 {
    //         let pos = (x, y);
    //         if positions.contains(&pos) {
    //             tl += 1;
    //         }
    //     }
    // }
    let mut tr = 0;
    // for y in 0..(HEIGHT - 3) / 2 {
    //     for x in (WIDTH + 1) / 2..WIDTH {
    //         let pos = (x, y);
    //         if positions.contains(&pos) {
    //             tr += 1;
    //         }
    //     }
    // }
    let mut bl = 0;
    // for y in (HEIGHT + 1) / 2..HEIGHT {
    //     for x in 0..=WIDTH / 2 {
    //         let pos = (x, y);
    //         if positions.contains(&pos) {
    //             bl += 1;
    //         }
    //     }
    // }
    let mut br = 0;
    // for y in (HEIGHT + 1) / 2..HEIGHT {
    //     for x in (WIDTH + 1) / 2..WIDTH {
    //         let pos = (x, y);
    //         if positions.contains(&pos) {
    //             br += 1;
    //         }
    //     }
    // }
    // println!("");
    for robot in &robots {
        // println!("{:?}", robot);
        let x = robot.pos.0;
        let y = robot.pos.1;

        if x <= (WIDTH - 3) / 2 {
            // ex: w11 , 0 thru 4
            if y <= (HEIGHT - 3) / 2 {
                tl += 1;
            } else if y >= (HEIGHT + 1) / 2 {
                bl += 1;
            }
        } else if x >= (WIDTH + 1) / 2 {
            // ex: w11, 6 thru 10
            if y <= (HEIGHT - 3) / 2 {
                tr += 1;
            } else if y >= (HEIGHT + 1) / 2 {
                br += 1;
            }
        }
    }

    // println!("TL{tl} TR{tr} BL{bl} BR{br}");

    let answer = tl * tr * bl * br;

    print_map(&robots, true);

    final_answer(answer, submit, DAY, SOL).await;
}

pub fn print_map(robots: &Vec<Robot>, hide_middle: bool) {
    println!("\n\n");
    let skip_x = (WIDTH - 1) / 2; //11-1/2=5
    let skip_y = (HEIGHT - 1) / 2;
    for y in 0..HEIGHT {
        println!("");
        for x in 0..WIDTH {
            if hide_middle && (x == skip_x || y == skip_y) {
                print!("{}", " ".to_owned().on_red());
            } else {
                let mut count = 0;
                for robot in robots {
                    if robot.pos.0 == x && robot.pos.1 == y {
                        count += 1;
                    }
                }
                if count == 0 {
                    print!(".")
                } else {
                    print!("{}", format!("{count}").on_blue());
                }
            }
        }
    }
}
