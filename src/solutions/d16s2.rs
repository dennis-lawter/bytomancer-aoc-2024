use std::collections::HashMap;
use std::collections::HashSet;

use super::d16s1::*;
use super::solutions::final_answer;

const DAY: u8 = 16;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let dun = Dun::from_input(&input);
    let mut memory: HashMap<(usize, usize, Dir), usize> = HashMap::new();
    let start = dun.get_start();
    let end = dun.get_end();
    let deer = Robot::new(start);
    let mut active_deer = vec![deer];
    let mut finished_deer: Vec<Robot> = vec![];

    dun.print();

    loop {
        if let Some(first_deer) = active_deer.pop() {
            if first_deer.pos == end {
                finished_deer.push(first_deer);
                continue;
            }
            {
                // march
                let mut deer = first_deer.clone();
                if deer.march_forward(&dun) {
                    if good_deer(&deer, &memory) {
                        let key = (deer.pos.0, deer.pos.1, deer.dir);
                        memory.insert(key, deer.score);
                        active_deer.push(deer);
                    }
                }
            }
            {
                // turn left
                let mut deer = first_deer.clone();
                if deer.turn_left() {
                    if good_deer(&deer, &memory) {
                        let key = (deer.pos.0, deer.pos.1, deer.dir);
                        memory.insert(key, deer.score);
                        active_deer.push(deer);
                    }
                }
            }
            {
                // turn left
                let mut deer = first_deer.clone();
                if deer.turn_right() {
                    if good_deer(&deer, &memory) {
                        let key = (deer.pos.0, deer.pos.1, deer.dir);
                        memory.insert(key, deer.score);
                        active_deer.push(deer);
                    }
                }
            }
        } else {
            break;
        }
    }
    // println!("\n\n\n{:#?}", finished_deer);

    let mut min_score = usize::MAX;
    for deer in finished_deer {
        if deer.score < min_score {
            min_score = deer.score;
        }
    }

    // now that we have min_score, we'll redo the algorithm with deer tracking
    let deer = RobotWithTracking::new(start);
    let mut active_deer = vec![deer];
    let mut finished_deer: Vec<RobotWithTracking> = vec![];
    loop {
        if let Some(first_deer) = active_deer.pop() {
            if first_deer.pos == end {
                finished_deer.push(first_deer);
                continue;
            }

            {
                // march
                let mut deer = first_deer.clone();
                if deer.march_forward(&dun) {
                    if deer_within_score(&deer, min_score) {
                        active_deer.push(deer);
                    }
                }
            }
            {
                // turn left
                let mut deer = first_deer.clone();
                if deer.turn_left() {
                    if deer_within_score(&deer, min_score) {
                        active_deer.push(deer);
                    }
                }
            }
            {
                // turn left
                let mut deer = first_deer.clone();
                if deer.turn_right() {
                    if deer_within_score(&deer, min_score) {
                        active_deer.push(deer);
                    }
                }
            }
        } else {
            break;
        }
    }

    let mut tiles_on_any_optimal_path: HashSet<(usize, usize)> = HashSet::new();
    for deer in finished_deer {
        for visited in deer.memory {
            let visited_tile = (visited.0, visited.1);
            tiles_on_any_optimal_path.insert(visited_tile);
        }
    }

    final_answer(tiles_on_any_optimal_path.len(), submit, DAY, SOL).await;
}

pub fn deer_within_score(deer: &RobotWithTracking, min_score: usize) -> bool {
    deer.score() <= min_score
}

#[derive(Clone, Debug)]
pub struct RobotWithTracking {
    pub pos: (usize, usize),
    pub dir: Dir,
    pub score: usize,
    pub memory: HashSet<(usize, usize, Dir)>,
}
impl RobotWithTracking {
    pub fn new(pos: (usize, usize)) -> Self {
        let mut memory = HashSet::new();
        memory.insert((pos.0, pos.1, Dir::E));
        RobotWithTracking {
            pos,
            dir: Dir::E, // from p1 problem text
            score: 0usize,
            memory,
        }
    }
    pub fn march_forward(&mut self, dun: &Dun) -> bool {
        let new_pos = match self.dir {
            Dir::N => (self.pos.0, self.pos.1 - 1),
            Dir::E => (self.pos.0 + 1, self.pos.1),
            Dir::S => (self.pos.0, self.pos.1 + 1),
            Dir::W => (self.pos.0 - 1, self.pos.1),
        };
        if dun.is_wall(new_pos) {
            false
        } else if self.memory.contains(&(new_pos.0, new_pos.1, self.dir)) {
            false
        } else {
            self.pos = new_pos;
            self.score += 1;
            self.memory.insert((new_pos.0, new_pos.1, self.dir));
            true
        }
    }
    pub fn turn_right(&mut self) -> bool {
        let new_dir = self.dir.rot_cw();
        if self.memory.contains(&(self.pos.0, self.pos.1, new_dir)) {
            false
        } else {
            self.memory.insert((self.pos.0, self.pos.1, new_dir));
            self.dir = new_dir;
            self.score += 1000;
            true
        }
    }
    pub fn turn_left(&mut self) -> bool {
        let new_dir = self.dir.rot_ccw();
        if self.memory.contains(&(self.pos.0, self.pos.1, new_dir)) {
            false
        } else {
            self.memory.insert((self.pos.0, self.pos.1, new_dir));
            self.dir = new_dir;
            self.score += 1000;
            true
        }
    }
    pub fn score(&self) -> usize {
        self.score
    }
}
