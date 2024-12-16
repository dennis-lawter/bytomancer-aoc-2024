use std::collections::HashMap;
use std::usize;
// use std::collections::HashSet;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 16;
const SOL: u8 = 1;

pub async fn input(example: bool) -> Vec<Vec<char>> {
    let raw = input_raw(DAY, example).await;
    raw.lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .map(|i| i.chars().collect())
        .collect()
}

#[derive(Clone, Debug)]
pub struct Dun {
    pub w: usize,
    pub h: usize,
    pub data: Vec<Vec<DunTile>>,
}
impl Dun {
    pub fn from_input(input: &Vec<Vec<char>>) -> Self {
        let h = input.len();
        let w = input[0].len();
        let mut data: Vec<Vec<DunTile>> = Vec::with_capacity(h);
        for y in 0..h {
            let mut row: Vec<DunTile> = Vec::with_capacity(w);
            for x in 0..w {
                row.push(DunTile::from_char(input[y][x]));
            }
            data.push(row);
        }
        Self { w, h, data }
    }
    pub fn get_start(&self) -> (usize, usize) {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.data[y][x] == DunTile::Start {
                    return (x, y);
                }
            }
        }
        panic!("No start found");
    }
    pub fn get_end(&self) -> (usize, usize) {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.data[y][x] == DunTile::End {
                    return (x, y);
                }
            }
        }
        panic!("No start found");
    }
    pub fn is_wall(&self, pos: (usize, usize)) -> bool {
        let (x, y) = pos;
        self.data[y][x] == DunTile::Wall
    }
    pub fn print(&self) {
        println!("=====DUNGEON=====");
        for y in 0..self.h {
            for x in 0..self.w {
                print!("{}", self.data[y][x].to_char());
            }
            println!("");
        }
        println!("=================");
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DunTile {
    Start,
    End,
    Wall,
    Empty,
}
impl DunTile {
    pub fn from_char(c: char) -> Self {
        match c {
            'S' => Self::Start,
            'E' => Self::End,
            '#' => Self::Wall,
            '.' => Self::Empty,
            _ => panic!("Invalid dun tile"),
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            DunTile::Start => 'S',
            DunTile::End => 'E',
            DunTile::Wall => '#',
            DunTile::Empty => '.',
        }
    }
}

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
    pub fn rot_ccw(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }
    pub fn to_usize(&self) -> usize {
        match self {
            Dir::N => 0,
            Dir::E => 1,
            Dir::S => 2,
            Dir::W => 3,
        }
    }
    // pub fn rot_180(&self) -> Self {
    //     match self {
    //         Dir::N => Dir::S,
    //         Dir::E => Dir::W,
    //         Dir::S => Dir::N,
    //         Dir::W => Dir::E,
    //     }
    // }
}

#[derive(Clone, Debug)]
pub struct Robot {
    pub pos: (usize, usize),
    pub dir: Dir,
    pub score: usize,
    // pub memory: HashSet<(usize, usize, Dir)>,
}
// I want to implement Ord/PartialOrd but guh
impl Robot {
    pub fn new(pos: (usize, usize)) -> Self {
        Robot {
            pos,
            dir: Dir::E, // from p1 problem text
            score: 0usize,
            // memory: HashSet::new(),
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
        // } else if self.memory.contains(&(new_pos.0, new_pos.1, self.dir)) {
        //     false
        } else {
            self.pos = new_pos;
            self.score += 1;
            // self.memory.insert((new_pos.0, new_pos.1, self.dir))
            true
        }
    }
    pub fn turn_right(&mut self) -> bool {
        let new_dir = self.dir.rot_cw();
        self.dir = new_dir;
        self.score += 1000;
        true
        // if self.memory.contains(&(self.pos.0, self.pos.1, new_dir)) {
        //     false
        // } else {
        // self.memory.insert((self.pos.0, self.pos.1, new_dir));
        // self.dir = new_dir;
        // true
        // }
    }
    pub fn turn_left(&mut self) -> bool {
        let new_dir = self.dir.rot_ccw();
        self.dir = new_dir;
        self.score += 1000;
        true
        // if self.memory.contains(&(self.pos.0, self.pos.1, new_dir)) {
        //     false
        // } else {
        // self.memory.insert((self.pos.0, self.pos.1, new_dir));
        // self.dir = new_dir;
        // true
        // }
    }
    pub fn score(&self) -> usize {
        self.score
    }
}

pub fn good_deer(deer: &Robot, memory: &HashMap<(usize, usize, Dir), usize>) -> bool {
    let key = (deer.pos.0, deer.pos.1, deer.dir);
    let score = deer.score();
    if memory.contains_key(&key) {
        memory[&key] > score
    } else {
        true
    }
}

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
            // panic!("No deers left...");
            break;
        }
    }
    println!("\n\n\n{:#?}", finished_deer);

    let mut min_score = usize::MAX;
    for deer in finished_deer {
        if deer.score < min_score {
            min_score = deer.score;
        }
    }

    final_answer(min_score, submit, DAY, SOL).await;
}
