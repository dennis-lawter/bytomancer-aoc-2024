use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 01;
const SOL: u8 = 1;

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

    left.sort();
    right.sort();

    let mut ans = 0;

    for i in 0..left.len() {
        if left[i] < right[i] {
            ans += right[i] - left[i];
        } else {
            ans += left[i] - right[i];
        }
    }

    final_answer(ans, submit, DAY, SOL).await;
}
