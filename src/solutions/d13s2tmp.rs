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
        let mut solutions: Vec<(usize, usize)> = vec![];

        let mut b_presses_needed_on_x_axis = self.prize.0 / self.b.0;
        if self.b.0 * b_presses_needed_on_x_axis < self.prize.0 {
            b_presses_needed_on_x_axis += 1;
        }

        let mut b_presses_needed_on_y_axis = self.prize.1 / self.b.1;
        if self.b.1 * b_presses_needed_on_y_axis < self.prize.1 {
            b_presses_needed_on_y_axis += 1;
        }

        let min_of_b_press_limit = [b_presses_needed_on_x_axis, b_presses_needed_on_y_axis]
            .iter()
            .min()
            .unwrap()
            .clone();

        let mut b_presses = min_of_b_press_limit;
        let mut a_presses = 0;
        let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
        let claw_y = a_presses * self.a.1 + b_presses * self.b.1;
        // let greater_claw = [claw_x, claw_y].iter().max().unwrap().clone();
        let x_gt_y = claw_x > claw_y;
        let mut claw = (claw_x, claw_y);
        println!(
            "Starting at\t\tA:{a_presses}, B:{b_presses}, CLAW: {:?}",
            claw
        );

        let is_claw_on_east_border = claw.0 > claw.1;

        let place_with_all_bs_pressed = claw.clone();

        b_presses -= 1;
        claw.0 -= self.b.0;
        claw.1 -= self.b.1;
        while claw.0 < self.prize.0 && claw.1 < self.prize.1 {
            a_presses += 1;
            claw.0 += self.a.0;
            claw.1 += self.a.1;
        }

        // OKAY
        // DO THIS
        // E border example
        // if claw.x not prize.x
        // trade b presses for a presses, adjusting for the "on the border" rule
        // the "on the border rule" is how we got here

        // repeat the trades until claw.x is prize.x
        // quickly check, are we on the prize?
        // if so, solve
        // if not, continue:
        // perform another series of b trades for a trades until claw.x == prize.x again
        // now we have a rhythm that is on-axis
        // mult the rhythm to get the solution

        // let axis_gained_when_trading_first_b_press = if is_claw_on_east_border {
        //     claw.1 - place_with_all_bs_pressed.1
        // } else {
        //     claw.0 - place_with_all_bs_pressed.0
        // };
        
        // while b_presses > 0 {
            // if x_gt_y {
            //     if claw.0 < self.prize.0 {
            //         // futility; we've passed the slope!
            //         return None;
            //     }
            // } else {
            //     if claw.1 < self.prize.1 {
            //         return None;
            //     }
            // }
            // let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
            // let claw_y = a_presses * self.a.1 + b_presses * self.b.1;
            // let mut claw = (claw_x, claw_y);
            // if b_presses % 10_000_000_000 == 0 {
            //     println!("Now at\t\tA:{a_presses}, B:{b_presses}, CLAW: {:?}", claw);
            // }
            // if claw == self.prize {
            //     solutions.push((a_presses, b_presses));
            //     let cost = b_presses + (a_presses * 3);
            //     println!("FOUND ONE!  A: {a_presses}\tB: {b_presses}\tcost: {cost}");
            //     // we can just return
            //     return Some(cost);
            // }
            // b_presses -= 1;
            // claw.0 -= self.b.0;
            // claw.1 -= self.b.1;
            // while claw.0 < self.prize.0 && claw.1 < self.prize.1 {
            //     a_presses += 1;
            //     claw.0 += self.a.0;
            //     claw.1 += self.a.1;
                // let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
                // let claw_y = a_presses * self.a.1 + b_presses * self.b.1;
                // let claw = (claw_x, claw_y);
                // if claw.0 >= self.prize.0 || claw.1 >= self.prize.1 {
                //     break;
                // }
            }
        // }

        let token_cost: Vec<usize> = solutions.iter().map(|item| item.0 * 3 + item.1).collect();

        token_cost.iter().min().copied()
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
    // println!("{:#?}", tokens_to_win);
    let answer: usize = tokens_to_win
        .iter()
        .filter(|item| **item != None)
        .map(|item| item.unwrap())
        .sum();

    final_answer(answer, submit, DAY, SOL).await;
}
