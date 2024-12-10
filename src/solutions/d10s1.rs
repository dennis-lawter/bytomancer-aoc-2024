use std::collections::HashSet;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 10;
const SOL: u8 = 1;

async fn input(example: bool) -> Vec<Vec<u8>> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .map(|i| {
            i.chars()
                .map(|item| String::from(item).parse::<u8>().expect("Failed to parse"))
                .collect()
        })
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut answer = 0;

    let mut th_idxs: Vec<(usize, usize)> = vec![];

    let h = input.len();
    let w = input[0].len();

    for y in 0..h {
        for x in 0..w {
            if input[y][x] == 0 {
                th_idxs.push((x, y));
            }
        }
    }

    for th_idx in th_idxs {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        count_trails(th_idx, &input, &mut set);
        answer += set.len();
    }

    final_answer(answer, submit, DAY, SOL).await;
}

pub fn count_trails(
    th_idx: (usize, usize),
    input: &Vec<Vec<u8>>,
    set: &mut HashSet<(usize, usize)>,
) {
    let h = input.len();
    let w = input[0].len();
    let (x, y) = th_idx;
    let curr_step = input[y][x];
    if curr_step == 9 {
        set.insert((x, y));
    }
    if x > 0 {
        let new_step_idx = (x - 1, y);
        if input[new_step_idx.1][new_step_idx.0] == curr_step + 1 {
            count_trails(new_step_idx, input, set);
        }
    }
    if x < w - 1 {
        let new_step_idx = (x + 1, y);
        if input[new_step_idx.1][new_step_idx.0] == curr_step + 1 {
            count_trails(new_step_idx, input, set);
        }
    }
    if y > 0 {
        let new_step_idx = (x, y - 1);
        if input[new_step_idx.1][new_step_idx.0] == curr_step + 1 {
            count_trails(new_step_idx, input, set);
        }
    }
    if y < h - 1 {
        let new_step_idx = (x, y + 1);
        if input[new_step_idx.1][new_step_idx.0] == curr_step + 1 {
            count_trails(new_step_idx, input, set);
        }
    }
}
