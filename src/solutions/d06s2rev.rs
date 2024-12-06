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

    let dun = Dungeon { map: input };
    let guard_start_pos = dun.start_guard_pos();
    let mut guard = Guard {
        x: guard_start_pos.0,
        y: guard_start_pos.1,
        dir: Dir::N,
        dun,
    };

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    // don't insert this so we don't parse it as an obstacle point
    // visited.insert(guard_start_pos);
    while guard.march() {
        let pos = (guard.x, guard.y);
        visited.insert(pos);
    }

    let opt_capacity = 2 * visited.len();

    for (x, y) in visited {
        // reset the guard
        guard.x = guard_start_pos.0;
        guard.y = guard_start_pos.1;
        guard.dir = Dir::N;
        if test_perm_causes_loop(&mut guard, x, y, opt_capacity) {
            answer += 1;
        }
    }

    final_answer(answer, submit, DAY, SOL).await;
}

fn test_perm_causes_loop(guard: &mut Guard, x: usize, y: usize, opt_capacity: usize) -> bool {
    // set the obstacle, it will be reset before return
    guard.dun.map[y][x] = 'O';
    let mut visited: HashSet<(usize, usize, Dir)> = HashSet::with_capacity(opt_capacity);
    let vis_status = (guard.x, guard.y, guard.dir);
    visited.insert(vis_status);
    let mut last_dir = guard.dir;
    while guard.march() {
        if guard.dir != last_dir {
            last_dir = guard.dir;
            let vis_status = (guard.x, guard.y, guard.dir);
            if visited.contains(&vis_status) {
                guard.dun.map[y][x] = '.';
                return true;
            }
            visited.insert(vis_status);
        }
    }
    guard.dun.map[y][x] = '.';
    false
}
