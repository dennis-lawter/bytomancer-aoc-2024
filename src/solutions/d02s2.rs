use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 02;
const SOL: u8 = 2;

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

fn test_safe_perms(line: Vec<u64>) -> bool {
    for i in 0..line.len() {
        let mut perm = line.clone();
        perm.remove(i);
        if super::d02s1::test_safe(perm) {
            return true;
        }
    }

    false
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut ans = 0;
    for line in input {
        if super::d02s1::test_safe(line.clone()) {
            ans += 1;
        } else if test_safe_perms(line) {
            ans += 1;
        }
    }
    final_answer(ans, submit, DAY, SOL).await;
}
