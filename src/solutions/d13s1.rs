use regex::Regex;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 13;
const SOL: u8 = 1;

#[derive(Debug, Clone)]
pub struct Machine {
    pub a: (usize, usize),
    pub b: (usize, usize),
    pub prize: (usize, usize),
}
impl Machine {
    pub fn least_tokens_to_win(&self) -> Option<usize> {
        println!("GOAL:{:?}", self.prize);
        let mut solutions: Vec<(usize, usize)> = vec![];
        // for a_presses in 0..=100 {
        //     for b_presses in 0..=100 {
        // let x_simplification = self.a.0 * self.b.0;
        // let y_simplification = self.a.1 * self.b.1;
        // for a_presses in 0..=100 * 10000000000000 {
        //     for b_presses in 0..=100 * 10000000000000 {
        //         let mut claw = (0usize, 0usize);
        //         claw.0 += a_presses * self.a.0;
        //         claw.1 += a_presses * self.a.1;
        //         claw.0 += b_presses * self.b.0;
        //         claw.1 += b_presses * self.b.1;
        //         if claw == self.prize {
        //             solutions.push((a_presses, b_presses));
        //             println!("FOUND ONE!  A: {a_presses} B: {b_presses}");
        //         } else if claw.0 > self.prize.0 || claw.1 > self.prize.1 {
        //             break;
        //         }
        //     }
        //     let mut claw = (0usize, 0usize);
        //     claw.0 += a_presses * self.a.0;
        //     claw.1 += a_presses * self.a.1;
        //     if claw.0 > self.prize.0 || claw.1 > self.prize.1 {
        //         break;
        //     }
        // }

        // x goal = 100, x progress = 2:  100/2=50 right
        // x goal = 100, x progress = 3: 100/3 = 33 but that's within the bounds of unsolved values
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
        let mut claw = (claw_x, claw_y);
        println!(
            "Starting at\t\tA:{a_presses}, B:{b_presses}, CLAW: {:?}",
            claw
        );
        while b_presses > 0 {
            // let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
            // let claw_y = a_presses * self.a.1 + b_presses * self.b.1;
            // let mut claw = (claw_x, claw_y);
            if b_presses % 10_000_000_000 == 0 {
                println!("Now at\t\tA:{a_presses}, B:{b_presses}, CLAW: {:?}", claw);
            }
            if claw == self.prize {
                solutions.push((a_presses, b_presses));
                let cost = b_presses + (a_presses * 3);
                println!("FOUND ONE!  A: {a_presses}\tB: {b_presses}\tcost: {cost}");
            }
            b_presses -= 1;
            claw.0 -= self.b.0;
            claw.1 -= self.b.1;
            while claw.0 < self.prize.0 && claw.1 < self.prize.1 {
                a_presses += 1;
                claw.0 += self.a.0;
                claw.1 += self.a.1;
                // let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
                // let claw_y = a_presses * self.a.1 + b_presses * self.b.1;
                // let claw = (claw_x, claw_y);
                // if claw.0 >= self.prize.0 || claw.1 >= self.prize.1 {
                //     break;
                // }
            }
        }

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
    let input = input(example).await;
    let tokens_to_win: Vec<Option<usize>> = input.iter().map(|m| m.least_tokens_to_win()).collect();
    println!("{:#?}", tokens_to_win);
    let answer: usize = tokens_to_win
        .iter()
        .filter(|item| **item != None)
        .map(|item| item.unwrap())
        .sum();

    final_answer(answer, submit, DAY, SOL).await;
}
