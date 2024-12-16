use std::collections::HashSet;

use super::d16s1::*;
use super::solutions::final_answer;

const DAY: u8 = 16;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let dun = Dun::from_input(&input);
    // shoe horning in Dijkstra-like solution
    let mut best_tile_score: Vec<Vec<Vec<usize>>> = vec![];
    for _y in 0..dun.h {
        let mut row: Vec<Vec<usize>> = vec![];
        for _x in 0..dun.w {
            let mut row2: Vec<usize> = vec![];
            for _dir in 0..4 {
                row2.push(usize::MAX);
            }
            row.push(row2);
        }
        best_tile_score.push(row);
    }

    let start = dun.get_start();
    let end = dun.get_end();
    let deer = Robot::new(start);
    let mut active_deer = vec![deer];
    let mut finished_deer: Vec<Robot> = vec![];

    dun.print();

    loop {
        if let Some(first_deer) = active_deer.pop() {
            if first_deer.pos == end {
                let x = first_deer.pos.0;
                let y = first_deer.pos.1;
                let z = first_deer.dir.to_usize();
                if first_deer.score <= best_tile_score[y][x][z] {
                    best_tile_score[y][x][z] = first_deer.score;
                    finished_deer.push(first_deer);
                    continue;
                }
            }
            {
                // march
                let mut deer = first_deer.clone();
                if deer.march_forward(&dun) {
                    let x = deer.pos.0;
                    let y = deer.pos.1;
                    let z = deer.dir.to_usize();
                    if deer.score() <= best_tile_score[y][x][z] {
                        best_tile_score[y][x][z] = deer.score();
                        active_deer.push(deer);
                    }
                }
            }
            {
                // turn left
                let mut deer = first_deer.clone();
                if deer.turn_left() {
                    let x = deer.pos.0;
                    let y = deer.pos.1;
                    let z = deer.dir.to_usize();
                    if deer.score() <= best_tile_score[y][x][z] {
                        best_tile_score[y][x][z] = deer.score();
                        active_deer.push(deer);
                    }
                }
            }
            {
                // turn left
                let mut deer = first_deer.clone();
                if deer.turn_right() {
                    let x = deer.pos.0;
                    let y = deer.pos.1;
                    let z = deer.dir.to_usize();
                    if deer.score() <= best_tile_score[y][x][z] {
                        best_tile_score[y][x][z] = deer.score();
                        active_deer.push(deer);
                    }
                }
            }
        } else {
            break;
        }
    }

    let mut min_score = usize::MAX;
    for deer in &finished_deer {
        if deer.score() < min_score {
            min_score = deer.score();
        }
    }

    // println!("{:#?}", best_tile_score);
    // panic!("{min_score}");

    // now that we have min_score, we'll redo the algorithm with deer tracking
    let deer = RobotWithTracking::new(start);
    let mut active_deer = vec![deer];
    let mut best_deers: Vec<RobotWithTracking> = vec![];
    loop {
        if let Some(first_deer) = active_deer.pop() {
            // println!("{:?}", first_deer);
            if first_deer.robot.pos == end {
                if first_deer.score() == min_score {
                    best_deers.push(first_deer);
                }
                continue;
            }

            {
                // march
                let mut deer = first_deer.clone();
                if deer.march_forward(&dun) {
                    // println!("March!");
                    let x = deer.robot.pos.0;
                    let y = deer.robot.pos.1;
                    let z = deer.robot.dir.to_usize();
                    if best_tile_score[y][x][z] == deer.score() {
                        active_deer.push(deer);
                    }
                }
            }
            {
                // turn left
                let mut deer = first_deer.clone();
                if deer.turn_left() {
                    // println!("Left!");
                    let x = deer.robot.pos.0;
                    let y = deer.robot.pos.1;
                    let z = deer.robot.dir.to_usize();
                    if best_tile_score[y][x][z] == deer.score() {
                        active_deer.push(deer);
                    }
                }
            }
            {
                // turn left
                let mut deer = first_deer.clone();
                if deer.turn_right() {
                    // println!("Right!");
                    // if deer_within_score(&deer, min_score) {
                    let x = deer.robot.pos.0;
                    let y = deer.robot.pos.1;
                    let z = deer.robot.dir.to_usize();
                    if best_tile_score[y][x][z] == deer.score() {
                        active_deer.push(deer);
                    }
                }
            }
        } else {
            break;
        }
    }

    let mut tiles_on_any_optimal_path: HashSet<(usize, usize)> = HashSet::new();
    for deer in best_deers {
        for visited in deer.memory {
            let visited_tile = (visited.0, visited.1);
            tiles_on_any_optimal_path.insert(visited_tile);
        }
    }

    final_answer(tiles_on_any_optimal_path.len(), submit, DAY, SOL).await;
}

#[derive(Clone, Debug)]
pub struct RobotWithTracking {
    pub robot: Robot,
    pub memory: HashSet<(usize, usize)>,
}
impl RobotWithTracking {
    pub fn new(pos: (usize, usize)) -> Self {
        let mut memory = HashSet::new();
        memory.insert((pos.0, pos.1));
        let robot = Robot {
            pos,
            dir: Dir::E, // from p1 problem text
            score: 0usize,
        };
        Self { robot, memory }
    }
    pub fn march_forward(&mut self, dun: &Dun) -> bool {
        if self.robot.march_forward(dun) {
            self.memory.insert(self.robot.pos);
            return true;
        } else {
            return false;
        }
    }
    pub fn turn_right(&mut self) -> bool {
        self.robot.turn_right()
    }
    pub fn turn_left(&mut self) -> bool {
        self.robot.turn_left()
    }
    pub fn score(&self) -> usize {
        self.robot.score()
    }
}
