use std::collections::HashSet;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 06;
const SOL: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    E,
    S,
    W,
}
impl Dir {
    pub fn rot_cw(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dungeon {
    pub map: Vec<Vec<char>>,
}
impl Dungeon {
    pub fn get_w(&self) -> usize {
        return self.map[0].len();
    }
    pub fn get_h(&self) -> usize {
        return self.map.len();
    }
    pub fn is_obs(&self, x: usize, y: usize) -> bool {
        self.map[y][x] == '#' || self.map[y][x] == 'O'
    }
    pub fn start_guard_pos(&self) -> (usize, usize) {
        for y in 0..self.get_h() {
            for x in 0..self.get_w() {
                if self.map[y][x] == '^' {
                    return (x, y);
                }
            }
        }
        panic!("No guard");
    }
}

#[derive(Debug, Clone)]
pub struct Guard {
    pub x: usize,
    pub y: usize,
    pub dir: Dir,
    pub dun: Dungeon,
}
impl Guard {
    pub fn march(&mut self) -> bool {
        match self.dir {
            Dir::N => {
                if self.y == 0 {
                    return false;
                }
                self.y -= 1;
                if self.dun.is_obs(self.x, self.y) {
                    self.y += 1;
                    self.dir = self.dir.rot_cw();
                    return self.march();
                }
            }
            Dir::E => {
                if self.x + 1 >= self.dun.get_w() {
                    return false;
                }
                self.x += 1;
                if self.dun.is_obs(self.x, self.y) {
                    self.x -= 1;
                    self.dir = self.dir.rot_cw();
                    return self.march();
                }
            }
            Dir::S => {
                if self.y + 1 >= self.dun.get_h() {
                    return false;
                }
                self.y += 1;
                if self.dun.is_obs(self.x, self.y) {
                    self.y -= 1;
                    self.dir = self.dir.rot_cw();
                    return self.march();
                }
            }
            Dir::W => {
                if self.x == 0 {
                    return false;
                }
                self.x -= 1;
                if self.dun.is_obs(self.x, self.y) {
                    self.x += 1;
                    self.dir = self.dir.rot_cw();
                    return self.march();
                }
            }
        }

        true
    }
}

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
    // let mut answer = 0;

    let dun = Dungeon { map: input };
    let guard_pos = dun.start_guard_pos();
    let mut guard = Guard {
        x: guard_pos.0,
        y: guard_pos.1,
        dir: Dir::N,
        dun,
    };
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(guard_pos);
    while guard.march() {
        let pos = (guard.x, guard.y);
        visited.insert(pos);
    }

    final_answer(visited.len(), submit, DAY, SOL).await;
}
