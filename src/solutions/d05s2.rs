use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 05;
const SOL: u8 = 2;
#[derive(Default, Debug)]
struct Input {
    ord: Vec<(u64, u64)>,
    upd: Vec<Vec<u64>>,
}

async fn input(example: bool) -> Input {
    let raw = input_raw(DAY, example).await;
    let mut out = Input::default();

    let groups: Vec<String> = raw
        .split("\n\n")
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    let g1 = &groups[0];
    for g1l in g1.lines() {
        let g1ls: Vec<&str> = g1l.split("|").collect();
        let g1ls1 = g1ls[0].parse::<u64>().expect("not int");
        let g1ls2 = g1ls[1].parse::<u64>().expect("not int");
        out.ord.push((g1ls1, g1ls2));
    }
    let g2 = &groups[1];
    for g2l in g2.lines() {
        let mut g2v = vec![];
        for g in g2l.split(",") {
            g2v.push(g.parse().unwrap());
        }
        out.upd.push(g2v);
    }

    out
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    // println!("{:?}", input);
    // panic!("EARLY TERMINATION FOR TESTING");
    let mut answer = 0;
    for upd in input.upd {
        let mut valid = true;
        for (left, right) in input.ord.iter() {
            match upd.iter().position(|item| item == left) {
                Some(upd_left_pos) => match upd.iter().position(|item| item == right) {
                    Some(upd_right_pos) => {
                        // println!("testing: {left} {right} {upd_left_pos} {upd_right_pos}");
                        if upd_right_pos < upd_left_pos {
                            valid = false;
                        }
                    }
                    None => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }
        }
        if !valid {
            answer += fixed_middle(&upd, &input.ord);
        }
    }
    final_answer(answer, submit, DAY, SOL).await;
}

fn fixed_middle(upd: &Vec<u64>, ord: &Vec<(u64, u64)>) -> u64 {
    let mut old_upd = upd.clone();
    let mut new_upd: Vec<u64> = vec![old_upd.pop().unwrap()];

    while !old_upd.is_empty() {
        let val = old_upd.pop().unwrap();
        let nums_gt_val: Vec<u64> = {
            let mut ans = vec![];
            for (left, right) in ord {
                if *left == val {
                    ans.push(*right);
                }
            }
            ans
        };

        // so now we have nums_gt_val
        // find the first i where we find a num in nums_gt_val
        // insert it at i
        // if not found, throw it at the back?

        let tmp = new_upd.clone();
        println!("nums gt {} are: {:?}", val, nums_gt_val);
        let mut inserted = false;
        for (i, v) in tmp.iter().enumerate() {
            if nums_gt_val.contains(v) {
                new_upd.insert(i, val);
                println!("new_upd: {:?}", new_upd);
                inserted = true;
                break;
            }
        }
        if !inserted {
            new_upd.push(val);
        }
    }

    let middle_idx = new_upd.len() / 2;
    let middle = new_upd[middle_idx];
    println!("{:?}", new_upd);
    middle
}
