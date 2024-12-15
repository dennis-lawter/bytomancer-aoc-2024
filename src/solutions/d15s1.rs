use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 15;
const SOL: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    E,
    S,
    W,
}
impl Dir {
    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Self::N,
            '>' => Self::E,
            'v' => Self::S,
            '<' => Self::W,
            invalid => panic!("Robot chars had a {invalid} in it!"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rob {
    pub pos: (i32, i32),
    pub mov_list: Vec<char>,
    pub mov_i: usize,
}
impl Rob {
    pub fn march(&mut self, dun: &mut Dun) {
        let c = self.mov_list[self.mov_i];
        let dir = Dir::from_char(c);
        let new_pos = match dir {
            Dir::N => (self.pos.0, self.pos.1 - 1),
            Dir::E => (self.pos.0 + 1, self.pos.1),
            Dir::S => (self.pos.0, self.pos.1 + 1),
            Dir::W => (self.pos.0 - 1, self.pos.1),
        };
        if dun.is_wall(new_pos) {
            return;
        } else if dun.is_box(new_pos) {
            dun.push_box(new_pos, dir);
            // let b = dun.get_box_mut_ref(new_pos).unwrap();
            // if b.push(dir, dun) {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct Box {
    pub pos: (i32, i32),
}
impl Box {
    // pub fn push(&mut self, dir: Dir, dun: &Dun) -> bool {
    //     let mut new_pos = match dir {
    //         Dir::N => (self.pos.0, self.pos.1 - 1),
    //         Dir::E => (self.pos.0 + 1, self.pos.1),
    //         Dir::S => (self.pos.0, self.pos.1 + 1),
    //         Dir::W => (self.pos.0 - 1, self.pos.1),
    //     };
    //     if dun.is_wall(new_pos) {
    //         return false;
    //     // } else if let Some(b) = dun.get_box_mut_ref(new_pos) {
    //     //     b.push(dir, dun);
    //     } else if dun.is_box(new_pos) {
    //         // we're going to attempt to teleport the box to the end of the line
    //         while !dun.is_wall(new_pos) {
    //             if dun.is_box(new_pos) {
    //                 new_pos = match dir {
    //                     Dir::N => (self.pos.0, self.pos.1 - 1),
    //                     Dir::E => (self.pos.0 + 1, self.pos.1),
    //                     Dir::S => (self.pos.0, self.pos.1 + 1),
    //                     Dir::W => (self.pos.0 - 1, self.pos.1),
    //                 };
    //             } else {
    //                 self.pos = new_pos;
    //                 return true;
    //             }
    //         }
    //         return false;
    //     } else {
    //         self.pos = new_pos;
    //         return true;
    //     }
    // }
}

#[derive(Clone, Debug)]
pub struct Dun {
    pub walls: Vec<(i32, i32)>,
    pub boxes: Vec<Box>,
}
impl Dun {
    pub fn is_wall(&mut self, pos: (i32, i32)) -> bool {
        self.walls.contains(&pos)
    }
    pub fn is_box(&mut self, pos: (i32, i32)) -> bool {
        for b in self.boxes.iter() {
            if b.pos == pos {
                return true;
            }
        }

        false
    }
    pub fn get_box_mut_ref(&mut self, pos: (i32, i32)) -> Option<&mut Box> {
        for b in self.boxes.iter_mut() {
            if b.pos == pos {
                return Some(b);
            }
        }

        None
    }
    pub fn push_box(&mut self, pos: (i32, i32), dir: Dir) -> bool {
        if let Some(b) = self.get_box_mut_ref(pos) {
            let mut new_pos = match dir {
                Dir::N => (b.pos.0, b.pos.1 - 1),
                Dir::E => (b.pos.0 + 1, b.pos.1),
                Dir::S => (b.pos.0, b.pos.1 + 1),
                Dir::W => (b.pos.0 - 1, b.pos.1),
            };
            if self.is_wall(new_pos) {
                return false;
            // } else if let Some(b) = self.get_box_mut_ref(new_pos) {
            //     b.push(dir, self);
            } else if self.is_box(new_pos) {
                // we're going to attempt to teleport the box to the end of the line
                while !self.is_wall(new_pos) {
                    if self.is_box(new_pos) {
                        new_pos = match dir {
                            Dir::N => (b.pos.0, b.pos.1 - 1),
                            Dir::E => (b.pos.0 + 1, b.pos.1),
                            Dir::S => (b.pos.0, b.pos.1 + 1),
                            Dir::W => (b.pos.0 - 1, b.pos.1),
                        };
                    } else {
                        b.pos = new_pos;
                        return true;
                    }
                }
                return false;
            } else {
                b.pos = new_pos;
                return true;
            }
        }

        false
    }
}

async fn input(example: bool) -> (Rob, Dun) {
    let raw = input_raw(DAY, example).await;
    let groups: Vec<String> = raw
        .split("\n\n")
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    let mov_list: Vec<char> = groups[1].replace("\n", "").chars().collect();
    let mut walls: Vec<(i32, i32)> = vec![];
    let mut boxes: Vec<Box> = vec![];

    let mut rob_opt: Option<Rob> = None;

    let map_lines: Vec<String> = groups[0].lines().map(|item| item.to_owned()).collect();
    for y in 0..map_lines.len() {
        let map_vals: Vec<char> = map_lines[y].chars().collect();
        for x in 0..map_vals.len() {
            let pos = (x as i32, y as i32);
            match map_vals[x] {
                '#' => {
                    walls.push(pos);
                }
                'O' => {
                    boxes.push(Box { pos });
                }
                '@' => {
                    rob_opt = Some(Rob {
                        pos,
                        mov_list: mov_list.clone(),
                        mov_i: 0,
                    });
                }
                _ => {}
            }
        }
    }

    let rob = rob_opt.expect("No robot constructed");
    let dun = Dun { walls, boxes };

    (rob, dun)
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    println!("{:?}", input);
    final_answer(0, submit, DAY, SOL).await;
}
