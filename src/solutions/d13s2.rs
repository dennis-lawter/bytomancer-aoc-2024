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
    pub fn least_tokens_to_win(&self) -> Option<usize> {
        let a_slope = self.a.1 as f64 / self.a.0 as f64;
        let b_slope = self.b.1 as f64 / self.b.0 as f64;
        let _p_slope = self.prize.1 as f64 / self.prize.0 as f64;

        let slope_diff = a_slope - b_slope;
        assert!(slope_diff.abs() > f64::EPSILON, "Dammit, Eric!!!");

        // find which slope is more "vertical" and which is more "horizontal"
        let (vert_slope, hor_slope) = if a_slope > b_slope {
            (a_slope, b_slope)
        } else {
            (b_slope, a_slope)
        };
        // Consider the function A(x)=mx for the A presses
        // Consider the function B(x)=mx for the B presses
        // Copy these functions to A' and B', and identify which has the greater m
        // The one with the greater m is the "vertical slope"
        // The horizontal slope gains an offset so that it passes through the prize
        // The vertical slope loses an offset so that it passes through the prize
        // The combination of all 4 equations produces a rhombus
        // The bottom left point of the rhombus is origin
        // The top right point of the rhombus is the prize
        // Once we have this rhombus, we need to find the point of intersection
        // We can do A(x) and B'(x) or B(x) and A'(x)
        // It doesn't matter which one

        let px = self.prize.0 as f64;
        let py = self.prize.1 as f64;

        // We will call H the horizontal slope function
        // The horizontal slope must be raised to intersect the prize
        // H(x) = mx; H'(x) = mx + c such that H'(px) = py;
        // solve for c
        // py = m*px + c
        // py = c + m*px
        // py - m*px = c
        let hor_slope_offset = py - hor_slope * px;

        // We will call V the vertical slope function
        // The vertical slope must move right (which is down; just visualize)
        // V(x) = mx; V'(x) = mx - c such that V'(px) = py;
        // solve for c
        // py = m * px - c
        // py + c = m * px
        // c = m * px - py
        let vert_slope_offset = vert_slope * px - py;

        // So now we have
        // H(x)     = mx       where m=hor_slope
        // H'(x)    = mx + c   where m=hor_slope and c=hor_slope_offset
        // V(x)     = mx       where m=vert_slope
        // V'(x)    = mx + c   where m=vert_slope and c=vert_slope_offset

        // we can use simultaneous linear equations to solve for a point of intersection
        // a1x + b1y = c1
        // a2x + b2y = c2
        // we're going to use H(x) and V'(x)
        if hor_slope == a_slope {
            // H(x) is A(x)
            // V'(x) is B(x) with offset
            let hx = self.a.0 as f64;
            let hy = self.a.1 as f64;
            let hc = 0.0f64;

            let vpx = self.b.0 as f64;
            let vpy = self.b.1 as f64;
            let vpc = vert_slope_offset;

            // I hope this is right
            let inter_x = (hy * vpc - vpy * hc) / (hc * vpy - vpx * hy);
            let inter_y = (hc * vpx - vpc * hx) / (hx * vpy - vpx * hy);

            let dist_to_inter = dist((0.0, 0.0), (inter_x, inter_y));
            let prize_f = (self.prize.0 as f64, self.prize.1 as f64);
            let dist_from_inter_to_prize = dist((inter_x, inter_y), prize_f);

            let a_dist_per_press = dist((0.0, 0.0), (self.a.0 as f64, self.a.1 as f64));
            let num_of_as_for_dist_to_inter = dist_to_inter / a_dist_per_press;
            let a_presses: usize = num_of_as_for_dist_to_inter.round() as usize;

            let b_dist_per_press = dist((0.0, 0.0), (self.b.0 as f64, self.b.1 as f64));
            let num_of_bs_for_dist_from_inter_to_prize =
                dist_from_inter_to_prize / b_dist_per_press;
            let b_presses: usize = num_of_bs_for_dist_from_inter_to_prize.round() as usize;

            let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
            let claw_y = a_presses * self.a.1 + b_presses * self.b.1;

            if (claw_x, claw_y) == self.prize {
                let cost = b_presses + (a_presses * 3);
                return Some(cost);
            }
        } else {
            // H(x) is B(x)
            // V'(x) is A(x) with offset
            // let hx = self.b.0 as f64;
            // let hy = self.b.1 as f64;
            // let hc = 0.0f64;

            // let vpx = self.a.0 as f64;
            // let vpy = self.a.1 as f64;
            // let vpc = vert_slope_offset;

            // I hope this is right
            // let inter_x = (hy * vpc - vpy * hc) / (hc * vpy - vpx * hy);
            // let inter_y = (hc * vpx - vpc * hx) / (hx * vpy - vpx * hy);
            let (inter_x, inter_y) =
                match intersection(a_slope, b_slope, -1.0 * vert_slope_offset, 0.0) {
                    Some(valid) => valid,
                    None => {
                        return None;
                    }
                };

            println!("Horizontal slope: {hor_slope}");
            println!("Vertical slope: {vert_slope}");
            println!("");
            println!("Horizontal offset: {hor_slope_offset}");
            println!("Vertical offset: {vert_slope_offset}");
            println!("");
            println!("Intersection: {:?}", (inter_x, inter_y));

            let dist_to_inter = dist((0.0, 0.0), (inter_x, inter_y));
            let prize_f = (self.prize.0 as f64, self.prize.1 as f64);
            let dist_from_inter_to_prize = dist((inter_x, inter_y), prize_f);

            let a_dist_per_press = dist((0.0, 0.0), (self.a.0 as f64, self.a.1 as f64));
            let num_of_as_for_dist_to_inter = dist_to_inter / a_dist_per_press;
            let a_presses: usize = num_of_as_for_dist_to_inter.round() as usize;

            let b_dist_per_press = dist((0.0, 0.0), (self.b.0 as f64, self.b.1 as f64));
            let num_of_bs_for_dist_from_inter_to_prize =
                dist_from_inter_to_prize / b_dist_per_press;
            let b_presses: usize = num_of_bs_for_dist_from_inter_to_prize.round() as usize;

            let claw_x = a_presses * self.a.0 + b_presses * self.b.0;
            let claw_y = a_presses * self.a.1 + b_presses * self.b.1;

            if (claw_x, claw_y) == self.prize {
                let cost = b_presses + (a_presses * 3);
                return Some(cost);
            }
        }

        // IGNORE THE FOLLOWING
        // Attempting to solve via the Law of Cosines,
        // which is hair pulling

        // let a_deg = f64::atan(a_slope);
        // let b_deg = f64::atan(b_slope);
        // let p_deg = f64::atan(p_slope);

        // let deg_to_rad = std::f64::consts::PI / 180.0;

        // let a_rad = a_deg * deg_to_rad;
        // let b_rad = b_deg * deg_to_rad;
        // let p_rad = p_deg * deg_to_rad;

        // let total_dist_to_prize = dist((0, 0), self.prize);

        None
    }
}

fn dist(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let x1 = p1.0;
    let x2 = p2.0;
    let y1 = p1.1;
    let y2 = p2.1;
    let x_cmp = (x2 - x1) * (x2 - x1);
    let y_cmp = (y2 - y1) * (y2 - y1);
    let sqr_dst = x_cmp + y_cmp;
    f64::sqrt(sqr_dst)
}

fn intersection(a_slope: f64, b_slope: f64, a_offset: f64, b_offset: f64) -> Option<(f64, f64)> {
    let slope_diff = a_slope - b_slope;
    if slope_diff.abs() < f64::EPSILON {
        return None;
    }

    let x = (b_offset - a_offset) / (a_slope - b_slope);
    let y = (a_slope * x) + a_offset;

    Some((x, y))
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
