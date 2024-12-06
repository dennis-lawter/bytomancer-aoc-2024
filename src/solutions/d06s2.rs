use std::collections::HashSet;

use super::d06s1::Dir;
use super::d06s1::Dungeon;
use super::d06s1::Guard;
use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 06;
const SOL: u8 = 2;

async fn input(example: bool) -> Vec<Vec<char>> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .map(|i| i.chars().collect())
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut answer = 0;

    let dun = Dungeon { map: input.clone() };
    let guard_start_pos = dun.start_guard_pos();
    let guard = Guard {
        x: guard_start_pos.0,
        y: guard_start_pos.1,
        dir: Dir::N,
        dun,
    };

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            // can't spawn the obstacle on the guard
            if guard_start_pos.0 == x && guard_start_pos.1 == y {
                continue;
            }
            if !guard.dun.is_obs(x, y) {
                if test_perm_causes_loop(guard.clone(), x, y) {
                    answer += 1;
                }
            }
        }
    }

    final_answer(answer, submit, DAY, SOL).await;
}

fn test_perm_causes_loop(guard: Guard, x: usize, y: usize) -> bool {
    let mut guard = guard;
    guard.dun.map[y][x] = 'O';
    let mut visited: HashSet<(usize, usize, Dir)> = HashSet::new();
    let vis_status = (guard.x, guard.y, guard.dir);
    visited.insert(vis_status);
    while guard.march() {
        let vis_status = (guard.x, guard.y, guard.dir);
        if visited.contains(&vis_status) {
            return true;
        }
        visited.insert(vis_status);
    }
    false
}
