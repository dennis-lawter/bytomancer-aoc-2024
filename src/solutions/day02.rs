use super::final_answer;
use super::input_raw;

const DAY: u8 = 02;

async fn input(example: bool) -> Vec<Vec<u64>> {
    let raw = input_raw(DAY, example).await;
    let mut grid = vec![];
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();
    for line in lines {
        let grid_line: Vec<u64> = line
            .split_whitespace()
            .map(|item| item.parse::<u64>().expect("Failed to parse during input"))
            .collect();
        grid.push(grid_line);
    }

    grid
}

fn test_safe(line: Vec<u64>) -> bool {
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

pub async fn d02s1(submit: bool, example: bool) {
    let input = input(example).await;
    let mut ans = 0;
    for line in input {
        if test_safe(line) {
            ans += 1;
        }
    }
    final_answer(ans, submit, DAY, 1).await;
}

pub async fn d02s2(submit: bool, example: bool) {
    let input = input(example).await;
    let mut ans = 0;
    for line in input {
        if test_safe(line.clone()) {
            ans += 1;
        } else if test_safe_perms(line) {
            ans += 1;
        }
    }
    final_answer(ans, submit, DAY, 2).await;
}

fn test_safe_perms(line: Vec<u64>) -> bool {
    for i in 0..line.len() {
        let mut perm = line.clone();
        perm.remove(i);
        if test_safe(perm) {
            return true;
        }
    }

    false
}
