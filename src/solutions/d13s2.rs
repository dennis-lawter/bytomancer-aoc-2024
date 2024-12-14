use futures::future::join_all;

use regex::Regex;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 13;
const SOL: u8 = 2;

#[derive(Debug, Clone)]
pub struct Machine {
    pub a: (usize, usize),
    pub b: (usize, usize),
    pub prize: (usize, usize),
}
impl Machine {
    pub async fn least_tokens_to_win(&self) -> Option<usize> {
        println!("GOAL:{:?}", self.prize);
        // let mut solutions: Vec<(usize, usize)> = vec![];

        let a_slope = self.a.0 as f32 / self.a.1 as f32;
        let b_slope = self.b.0 as f32 / self.b.1 as f32;
        let _p_slope = self.prize.0 as f32 / self.prize.1 as f32;

        let mut claw = (0usize, 0usize);
        let mut a_presses = 0usize;
        let mut b_presses = 0usize;

        let a_tends_east = a_slope > b_slope;
        let prize_x = self.prize.0;
        let prize_y = self.prize.1;
        // I only want to go "east"
        if a_tends_east {
            println!("A tends east");
            let mut mult = prize_x / self.a.0;
            if prize_x % self.a.0 != 0 {
                // just add an extra because we want to hit the mark
                // going beyond is fine
                mult += 1;
                println!("we didn't hit it first try");
            }
            a_presses = mult;
            claw.0 = self.a.0 * mult;
            claw.1 = self.a.1 * mult;
            println!("We start at {:?}\tA:{a_presses}\tB:{b_presses}", claw);
            while claw.0 != prize_x {
                if claw.1 > prize_y {
                    // well this is worst case
                    // we have a non-convergent pattern...
                    // this will loop for a WHILE.
                    // consider tracking the starting x,
                    // then if we cycle back to that x,
                    // we can terminate then...
                    println!("???");
                    return None;
                }
                a_presses -= 1;
                claw.0 -= self.a.0;
                claw.1 -= self.a.1;
                while claw.0 < prize_x {
                    b_presses += 1;
                    claw.0 += self.b.0;
                    claw.1 += self.b.1;
                }
            }
            println!(
                "We aligned to the prize X {:?}\tA:{a_presses}\tB:{b_presses}",
                claw
            );
            // after the first series, just in case...
            if claw == self.prize {
                let cost = b_presses + (a_presses * 3);
                return Some(cost);
            }
            let pattern_start = claw.clone();
            let pattern_start_a_presses = a_presses.clone();
            let pattern_start_b_presses = b_presses.clone();

            while claw == pattern_start || claw.0 != prize_x {
                if claw.1 > prize_y {
                    // well this is worst case
                    // we have a non-convergent pattern...
                    // this will loop for a WHILE.
                    // consider tracking the starting x,
                    // then if we cycle back to that x,
                    // we can terminate then...
                    println!("??????");
                    return None;
                }
                a_presses -= 1;
                claw.0 -= self.a.0;
                claw.1 -= self.a.1;
                while claw.0 < prize_x {
                    b_presses += 1;
                    claw.0 += self.b.0;
                    claw.1 += self.b.1;
                }
            }

            println!(
                "Second align to the prize X {:?}\tA:{a_presses}\tB:{b_presses}",
                claw
            );
            // after the second series, just in case...
            if claw == self.prize {
                let cost = b_presses + (a_presses * 3);
                return Some(cost);
            }
            let pattern_end = claw.clone();
            let pattern_end_a_presses = a_presses.clone();
            let pattern_end_b_presses = b_presses.clone();

            // OKAY
            // now we have two claw positions that are axis-aligned to prize
            let gained_y = pattern_end.1 - pattern_start.1;
            let y_left = prize_y - pattern_end.1;
            if y_left % gained_y != 0 {
                // there is no path to land on the prize
                // we will weave around it instead
                return None;
            }
            let y_mult = y_left / gained_y;
            // simulate the pattern * y_mult
            let pattern_a_presses_lost = pattern_start_a_presses - pattern_end_a_presses;
            let pattern_b_presses_gained = pattern_end_b_presses - pattern_start_b_presses;
            a_presses -= pattern_a_presses_lost * y_mult;
            b_presses += pattern_b_presses_gained * y_mult;
            // sanity check?
            // assert(...)

            let cost = b_presses + (a_presses * 3);
            return Some(cost);
        } else {
            println!("B tends east");
            // oh god the copy pasting
            // forgive me
            // we will now be pressing B a lot
            // then removing 1 B for some amount of As
            let mut mult = prize_x / self.b.0;
            if prize_x % self.b.0 != 0 {
                // just add an extra because we want to hit the mark
                mult += 1;
            }
            b_presses = mult;
            claw.0 = self.b.0 * mult;
            claw.1 = self.b.1 * mult;
            while claw.0 != prize_x {
                if claw.1 > prize_y {
                    // well this is worst case
                    // we have a non-convergent pattern...
                    // this will loop for a WHILE.
                    // consider tracking the starting x,
                    // then if we cycle back to that x,
                    // we can terminate then...
                    return None;
                }
                b_presses -= 1;
                claw.0 -= self.b.0;
                claw.1 -= self.b.1;
                while claw.0 < prize_x {
                    a_presses += 1;
                    claw.0 += self.a.0;
                    claw.1 += self.a.1;
                }
            }
            // after the first series, just in case...
            if claw == self.prize {
                let cost = b_presses + (a_presses * 3);
                return Some(cost);
            }
            let pattern_start = claw.clone();
            let pattern_start_a_presses = a_presses.clone();
            let pattern_start_b_presses = b_presses.clone();
            while claw == pattern_start || claw.0 != prize_x {
                if claw.1 > prize_y {
                    // well this is worst case
                    // we have a non-convergent pattern...
                    // this will loop for a WHILE.
                    // consider tracking the starting x,
                    // then if we cycle back to that x,
                    // we can terminate then...
                    return None;
                }
                b_presses -= 1;
                claw.0 -= self.b.0;
                claw.1 -= self.b.1;
                while claw.0 < prize_x {
                    a_presses += 1;
                    claw.0 += self.a.0;
                    claw.1 += self.a.1;
                }
            }
            // after the second series, just in case...
            if claw == self.prize {
                let cost = b_presses + (a_presses * 3);
                return Some(cost);
            }
            let pattern_end = claw.clone();
            let pattern_end_a_presses = a_presses.clone();
            let pattern_end_b_presses = b_presses.clone();

            // OKAY
            // now we have two claw positions that are axis-aligned to prize
            let gained_y = pattern_end.1 - pattern_start.1;
            let y_left = prize_y - pattern_end.1;
            if y_left % gained_y != 0 {
                // there is no path to land on the prize
                // we will weave around it instead
                return None;
            }
            let y_mult = y_left / gained_y;
            // simulate the pattern * y_mult
            let pattern_b_presses_lost = pattern_start_b_presses - pattern_end_b_presses;
            let pattern_a_presses_gained = pattern_end_a_presses - pattern_start_a_presses;
            b_presses -= pattern_b_presses_lost * y_mult;
            a_presses += pattern_a_presses_gained * y_mult;
            // sanity check?
            // assert(...)

            let cost = b_presses + (a_presses * 3);
            return Some(cost);
        }

        // None
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

    let mut futures = vec![];
    for machine in input.iter() {
        let f = machine.least_tokens_to_win();
        futures.push(f);
    }

    // let mut tokens_to_win: Vec<Option<usize>> = vec![];
    let tokens_to_win = join_all(futures).await;
    // for f in futures {
    //     tokens_to_win.push(f.await);
    // }
    // let tokens_to_win: Vec<Option<usize>> = input.iter().map(|m| m.least_tokens_to_win()).collect();
    println!("\n\n{:#?}", tokens_to_win);
    let answer: usize = tokens_to_win
        .iter()
        .filter(|item| **item != None)
        .map(|item| item.unwrap())
        .sum();

    final_answer(answer, submit, DAY, SOL).await;
}
