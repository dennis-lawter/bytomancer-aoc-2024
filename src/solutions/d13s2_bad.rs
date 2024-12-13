use regex::Regex;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 13;
const SOL: u8 = 2;

// println!("GOAL:{:?}", self.prize);
// let mut solutions: Vec<(usize, usize)> = vec![];

// let mut b_presses_needed_on_x_axis = self.prize.0 / self.b.0;
// if self.b.0 * b_presses_needed_on_x_axis < self.prize.0 {
//     b_presses_needed_on_x_axis += 1;
// }

// let mut b_presses_needed_on_y_axis = self.prize.1 / self.b.1;
// if self.b.1 * b_presses_needed_on_y_axis < self.prize.1 {
//     b_presses_needed_on_y_axis += 1;
// }

// let min_of_b_press_limit = [b_presses_needed_on_x_axis, b_presses_needed_on_y_axis]
//     .iter()
//     .min()
//     .unwrap()
//     .clone();

// let mut b_presses = min_of_b_press_limit;
// let mut a_presses = 0;
// let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
// let claw_y = a_presses * self.a.1 + b_presses * self.b.1;
// let mut claw = (claw_x, claw_y);
// println!(
//     "Starting at\t\tA:{a_presses}, B:{b_presses}, CLAW: {:?}",
//     claw
// );
// while b_presses > 0 {
//     // let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
//     // let claw_y = a_presses * self.a.1 + b_presses * self.b.1;
//     // let mut claw = (claw_x, claw_y);
//     if b_presses % 10_000_000_000 == 0 {
//         println!("Now at\t\tA:{a_presses}, B:{b_presses}, CLAW: {:?}", claw);
//     }
//     if claw == self.prize {
//         solutions.push((a_presses, b_presses));
//         let cost = b_presses + (a_presses * 3);
//         println!("FOUND ONE!  A: {a_presses}\tB: {b_presses}\tcost: {cost}");
//     }
//     b_presses -= 1;
//     claw.0 -= self.b.0;
//     claw.1 -= self.b.1;
//     while claw.0 < self.prize.0 && claw.1 < self.prize.1 {
//         a_presses += 1;
//         claw.0 += self.a.0;
//         claw.1 += self.a.1;
//         // let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
//         // let claw_y = a_presses * self.a.1 + b_presses * self.b.1;
//         // let claw = (claw_x, claw_y);
//         // if claw.0 >= self.prize.0 || claw.1 >= self.prize.1 {
//         //     break;
//         // }
//     }
// }

// let token_cost: Vec<usize> = solutions.iter().map(|item| item.0 * 3 + item.1).collect();

// token_cost.iter().min().copied()

#[derive(Debug, Clone)]
pub struct Machine {
    pub a: (usize, usize),
    pub b: (usize, usize),
    pub prize: (usize, usize),
}
impl Machine {
    pub fn solve_for_straight_lines(&self) -> Option<usize> {
        panic!("Dammit, Eric!!!");
    }

    pub fn solve_for_angles(&self) -> Option<usize> {
        // I don't have a mathematical proof,
        // but I believe it must be true that
        // there exists only 1 solution
        // It looks something like this:
        // Imagine the prize is at 100,100 making its slope 45 degrees
        // If A is 30 degrees and B is 60 degrees,
        // there exists at most 1 combination of A and B segments to touch the prize.
        // Remember that these line segments are communicative!
        //
        // General algorithm:
        //
        // Start the claw off at 0,0
        // Find the A slope and B slope
        // Whichever slope is less, apply that press first.
        // Keep track of the slope of the claw
        // When claw slope < prize slope,

        let a_slope = self.a.0 as f32 / self.a.1 as f32;
        let b_slope = self.b.0 as f32 / self.b.1 as f32;
        let p_slope = self.prize.0 as f32 / self.prize.1 as f32;
        let mut claw = (0, 0);
        let mut a_presses = 0;
        let mut b_presses = 0;
        if a_slope < b_slope {
            a_presses += 1;
            claw.0 += self.a.0;
            claw.1 += self.a.1;
        } else {
            b_presses += 1;
            claw.0 += self.b.0;
            claw.1 += self.b.1;
        }
        // let mut claw_slope = claw.0 as f32 / claw.1 as f32;
        while claw.0 < self.prize.0 && claw.1 < self.prize.1 {
            let claw_slope = claw.0 as f32 / claw.1 as f32;
            if claw_slope < p_slope {
                // need the greater slope applied
                if a_slope > b_slope {
                    a_presses += 1;
                    claw.0 += self.a.0;
                    claw.1 += self.a.1;
                } else {
                    b_presses += 1;
                    claw.0 += self.b.0;
                    claw.1 += self.b.1;
                }
            } else {
                // need the lesser slope applied
                if a_slope < b_slope {
                    a_presses += 1;
                    claw.0 += self.a.0;
                    claw.1 += self.a.1;
                } else {
                    b_presses += 1;
                    claw.0 += self.b.0;
                    claw.1 += self.b.1;
                }
            }
        }

        if claw == self.prize {
            let cost = b_presses + (a_presses * 3);
            Some(cost)
        } else {
            None
        }
    }
    pub fn least_tokens_to_win(&self) -> Option<usize> {
        // In this scenario, 0 travels north, infinity travels east.
        // 1.0 travels exactly north-east.
        // No zeros appeared in any of my input, so /0 is safe.
        let a_slope = self.a.0 as f32 / self.a.1 as f32;
        let b_slope = self.b.0 as f32 / self.b.1 as f32;
        let p_slope = self.prize.0 as f32 / self.prize.1 as f32;
        if a_slope == b_slope {
            if a_slope == p_slope {
                self.solve_for_straight_lines()
            } else {
                None
            }
        // the 4 following cases check against impossible slopes
        // for example, if the target is at (1000, 1000)
        // but A is (1, 100)
        // and B is (2, 100)
        // we'll never reach it
        // the y will go beyond the target bounds before the x is near
        } else if a_slope > b_slope && b_slope > p_slope {
            None
        } else if b_slope > a_slope && a_slope > p_slope {
            None
        } else if a_slope < b_slope && b_slope < p_slope {
            None
        } else if b_slope < a_slope && a_slope < p_slope {
            None
        } else {
            self.solve_for_angles()
        }
    }
}

pub async fn input(example: bool) -> Vec<Machine> {
    let raw = input_raw(DAY, example).await;
    let groups: Vec<String> = raw
        .split("\n\n")
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    let mut machines = vec![];
    let button_a_re = Regex::new(r#"Button A: X\+(\d+), Y\+(\d+)"#).unwrap();
    let button_b_re = Regex::new(r#"Button B: X\+(\d+), Y\+(\d+)"#).unwrap();
    let prize_re = Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap();
    for group in groups {
        let lines: Vec<String> = group.lines().map(|item| item.to_owned()).collect();

        // println!("A: {}", lines[0]);
        // println!("B: {}", lines[1]);
        // println!("C: {}", lines[2]);
        let a_matches = button_a_re.captures(&lines[0]).expect("A cap failed");
        let a = (
            a_matches[1].parse::<usize>().unwrap(),
            a_matches[2].parse::<usize>().unwrap(),
        );

        let b_matches = button_b_re.captures(&lines[1]).expect("B cap failed");
        let b = (
            b_matches[1].parse::<usize>().unwrap(),
            b_matches[2].parse::<usize>().unwrap(),
        );

        let prize_matches = prize_re.captures(&lines[2]).expect("Prize cap failed");
        let prize = (
            prize_matches[1].parse::<usize>().unwrap(),
            prize_matches[2].parse::<usize>().unwrap(),
        );

        machines.push(Machine { a, b, prize });
    }

    machines
}

pub async fn solve(submit: bool, example: bool) {
    let mut input = input(example).await;
    for machine in input.iter_mut() {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
    }
    let tokens_to_win: Vec<Option<usize>> = input.iter().map(|m| m.least_tokens_to_win()).collect();
    println!("{:#?}", tokens_to_win);
    let answer: usize = tokens_to_win
        .iter()
        .filter(|item| **item != None)
        .map(|item| item.unwrap())
        .sum();

    final_answer(answer, submit, DAY, SOL).await;
}
