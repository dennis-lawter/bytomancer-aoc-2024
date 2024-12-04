use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 02;
const SOL: u8 = 1;

async fn input(example: bool) -> Vec<Vec<u64>> {
    let raw = input_raw(DAY, example).await;
    raw.lines()
        .map(|i| i.to_owned())
        .filter(|item| item.len() > 0)
        .map(|i| {
            i.split_whitespace()
                .map(|j| j.parse::<u64>().expect("Failed to parse during input"))
                .collect()
        })
        .collect()
}

pub fn test_safe(line: Vec<u64>) -> bool {
    let allowed_dist = [1, 2, 3];
    let increasing = line[1] > line[0];
    if increasing {
        for i in 1..line.len() {
            if line[i] <= line[i - 1] || !allowed_dist.contains(&(line[i] - line[i - 1])) {
                return false;
            }
        }
    } else {
        for i in 1..line.len() {
            if line[i] >= line[i - 1] || !allowed_dist.contains(&(line[i - 1] - line[i])) {
                return false;
            }
        }
    }

    true
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut ans = 0;
    for line in input {
        if test_safe(line) {
            ans += 1;
        }
    }
    final_answer(ans, submit, DAY, SOL).await;
}
