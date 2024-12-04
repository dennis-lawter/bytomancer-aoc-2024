use std::collections::HashMap;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 01;
const SOL: u8 = 2;

async fn input(example: bool) -> Vec<(u64, u64)> {
    let mut output = vec![];
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    for line in lines {
        let stripped: Vec<String> = line
            .split_ascii_whitespace()
            .map(|item| item.to_owned())
            .collect();
        let mut res = (0u64, 0u64);
        res.0 = stripped[0].parse().unwrap();
        res.1 = stripped[1].parse().unwrap();
        output.push(res);
    }

    output
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    for line in input {
        left.push(line.0);
        right.push(line.1);
    }
    let mut r_occ: HashMap<u64, u64> = HashMap::new();
    for r in right {
        if !r_occ.contains_key(&r) {
            r_occ.insert(r, 1);
        } else {
            let old = r_occ.get(&r).unwrap();
            r_occ.insert(r, old + 1);
        }
    }
    let mut ans = 0;
    for l in left {
        let occ = r_occ.get(&l);
        match occ {
            Some(o) => {
                ans += o * l;
            }
            None => {}
        }
    }
    final_answer(ans, submit, DAY, SOL).await;
}
