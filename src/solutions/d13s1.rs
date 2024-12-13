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
        // let mut claw = (0usize, 0usize);
        // try_buttons(self.a, self.b, self.prize, claw, 0, 0)
        // let mut claw = (0usize, 0usize);
        // let mut max_a_presses = 0;
        // while claw.0 < self.prize.0 && claw.1 < self.prize.1 {
        //     claw.0 += self.a.0;
        //     claw.1 += self.a.1;
        //     max_a_presses += 1;
        //     if max_a_presses > 100 {
        //         break;
        //     }
        // }
        // let mut claw = (0usize, 0usize);
        // let mut max_b_presses = 0;
        // while claw.0 < self.prize.0 && claw.1 < self.prize.1 {
        //     claw.0 += self.b.0;
        //     claw.1 += self.b.1;
        //     max_b_presses += 1;
        //     if max_b_presses > 100 {
        //         break;
        //     }
        // }
        // println!("Maxes:{} {}", max_a_presses, max_b_presses);
        println!("GOAL:{:?}", self.prize);
        let mut solutions: Vec<(usize, usize)> = vec![];
        // for a_presses in 0..max_a_presses {
        //     for b_presses in 0..max_b_presses {
        for a_presses in 0..=100 {
            for b_presses in 0..=100 {
                let mut claw = (0usize, 0usize);
                claw.0 += a_presses * self.a.0;
                claw.1 += a_presses * self.a.1;
                claw.0 += b_presses * self.b.0;
                claw.1 += b_presses * self.b.1;
                if claw == self.prize {
                    solutions.push((a_presses, b_presses));
                    println!("FOUND ONE!  A: {a_presses} B: {b_presses}");
                } else if claw.0 > self.prize.0 || claw.1 > self.prize.1 {
                    break;
                }
                // else {
                //     if a_presses == 80 {
                //         println!("Nope  A: {a_presses} B: {b_presses}\t\t{:?}", claw);
                //     }
                // }
            }
        }

        let token_cost: Vec<usize> = solutions.iter().map(|item| item.0 * 3 + item.1).collect();

        token_cost.iter().min().copied()
    }
}

// pub enum Btn {
//     A,
//     B,
// }

// pub fn try_buttons(
//     a: (usize, usize),
//     b: (usize, usize),
//     prize: (usize, usize),
//     claw: (usize, usize),
//     tokens_so_far: usize,
//     futility: usize,
// ) -> Option<usize> {
//     let vec![]
//     None
// }

// pub fn try_buttons(
//     a: (usize, usize),
//     b: (usize, usize),
//     prize: (usize, usize),
//     claw: (usize, usize),
//     tokens_so_far: usize,
//     futility: usize,
// ) -> Option<usize> {
//     if claw == prize {
//         return Some(tokens_so_far);
//     } else if claw.0 > prize.0 {
//         return None;
//     } else if claw.1 > prize.1 {
//         return None;
//     } else if futility > 100 {
//         return None;
//     }

//     let press_a_claw = (claw.0 + a.0, claw.1 + a.1);
//     let press_a_tried = try_buttons(a, b, prize, press_a_claw, tokens_so_far + 3, futility + 1);
//     let press_b_claw = (claw.0 + b.0, claw.1 + b.1);
//     let press_b_tried = try_buttons(a, b, prize, press_b_claw, tokens_so_far + 1, futility + 1);

//     if press_a_tried == None && press_b_tried == None {
//         None
//     } else if press_a_tried == None {
//         press_b_tried
//     } else if press_b_tried == None {
//         press_a_tried
//     } else if press_a_tried < press_b_tried {
//         press_a_tried
//     } else {
//         press_b_tried
//     }
// }

async fn input(example: bool) -> Vec<Machine> {
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
    // let answer = 0;
    let tokens_to_win: Vec<Option<usize>> = input.iter().map(|m| m.least_tokens_to_win()).collect();
    println!("{:#?}", tokens_to_win);
    let answer: usize = tokens_to_win
        .iter()
        .filter(|item| **item != None)
        .map(|item| item.unwrap())
        .sum();

    final_answer(answer, submit, DAY, SOL).await;
}
