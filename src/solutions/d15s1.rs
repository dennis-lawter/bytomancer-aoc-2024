use colored::Colorize;

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
        self.mov_i += 1;
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
            if !dun.push_box(new_pos, dir) {
                return;
            }
        }

        self.pos = new_pos;
    }
}

#[derive(Clone, Debug)]
pub struct Box {
    pub pos: (i32, i32),
}
impl Box {
    // pub fn get_gps(&self) -> usize {
    //     self.pos.0 as usize * 100 + self.pos.1 as usize
    // }
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
pub struct DunRaw {
    pub dim: (usize, usize),
    pub walls: Vec<(i32, i32)>,
    pub boxes: Vec<Box>,
}
impl DunRaw {
    pub fn is_wall(&self, pos: (i32, i32)) -> bool {
        self.walls.contains(&pos)
    }
    pub fn is_box(&self, pos: (i32, i32)) -> bool {
        for b in self.boxes.iter() {
            if b.pos == pos {
                return true;
            }
        }

        false
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DunTile {
    Wall,
    Box,
    Empty,
}
impl DunTile {
    pub fn to_char(&self) -> char {
        match self {
            DunTile::Wall => '#',
            DunTile::Box => 'O',
            DunTile::Empty => '.',
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dun {
    pub data: Vec<Vec<DunTile>>,
}
impl Dun {
    pub fn from_raw(raw: &DunRaw) -> Self {
        let mut data: Vec<Vec<DunTile>> = Vec::with_capacity(raw.dim.1);
        for y in 0..raw.dim.1 {
            let mut row: Vec<DunTile> = Vec::with_capacity(raw.dim.0);
            for x in 0..raw.dim.0 {
                let pos = (x as i32, y as i32);
                let tile = if raw.is_wall(pos) {
                    DunTile::Wall
                } else if raw.is_box(pos) {
                    DunTile::Box
                } else {
                    DunTile::Empty
                };
                row.push(tile);
            }
            data.push(row);
        }
        Self { data }
    }

    fn print(&self, rob: &Rob) {
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                let pos = (x as i32, y as i32);
                if pos == rob.pos {
                    print!("{}", "@".on_red());
                } else {
                    print!("{}", self.data[y][x].to_char());
                }
            }
            println!("");
        }
    }

    fn is_wall(&self, pos: (i32, i32)) -> bool {
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        self.data[y][x] == DunTile::Wall
    }

    fn is_box(&self, pos: (i32, i32)) -> bool {
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        self.data[y][x] == DunTile::Box
    }

    fn is_empty(&self, pos: (i32, i32)) -> bool {
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        self.data[y][x] == DunTile::Empty
    }

    fn push_box(&mut self, pos: (i32, i32), dir: Dir) -> bool {
        let mut new_pos = pos.clone();
        while !self.is_wall(new_pos) {
            new_pos = match dir {
                Dir::N => (new_pos.0, new_pos.1 - 1),
                Dir::E => (new_pos.0 + 1, new_pos.1),
                Dir::S => (new_pos.0, new_pos.1 + 1),
                Dir::W => (new_pos.0 - 1, new_pos.1),
            };
            if self.is_empty(new_pos) {
                let x = pos.0 as usize;
                let y = pos.1 as usize;
                let new_x = new_pos.0 as usize;
                let new_y = new_pos.1 as usize;
                self.data[new_y][new_x] = DunTile::Box;
                self.data[y][x] = DunTile::Empty;
                return true;
            }
        }

        false
    }

    fn get_gps_sum(&self) -> usize {
        let mut ans = 0;
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.is_box((x as i32, y as i32)) {
                    ans += y * 100 + x
                }
            }
        }
        ans
    }
}

// impl DunRaw {
//     pub fn is_wall(&mut self, pos: (i32, i32)) -> bool {
//         self.walls.contains(&pos)
//     }
//     pub fn is_box(&mut self, pos: (i32, i32)) -> bool {
//         for b in self.boxes.iter() {
//             if b.pos == pos {
//                 return true;
//             }
//         }

//         false
//     }
//     pub fn get_box_mut_ref(&mut self, pos: (i32, i32)) -> Option<&mut Box> {
//         for b in self.boxes.iter_mut() {
//             if b.pos == pos {
//                 return Some(b);
//             }
//         }

//         None
//     }
//     pub fn push_box(&mut self, pos: (i32, i32), dir: Dir) -> bool {
//         if let Some(b) = self.get_box_mut_ref(pos) {
//             let mut new_pos = match dir {
//                 Dir::N => (b.pos.0, b.pos.1 - 1),
//                 Dir::E => (b.pos.0 + 1, b.pos.1),
//                 Dir::S => (b.pos.0, b.pos.1 + 1),
//                 Dir::W => (b.pos.0 - 1, b.pos.1),
//             };
//             if self.is_wall(new_pos) {
//                 return false;
//             // } else if let Some(b) = self.get_box_mut_ref(new_pos) {
//             //     b.push(dir, self);
//             } else if self.is_box(new_pos) {
//                 // we're going to attempt to teleport the box to the end of the line
//                 while !self.is_wall(new_pos) {
//                     if self.is_box(new_pos) {
//                         new_pos = match dir {
//                             Dir::N => (b.pos.0, b.pos.1 - 1),
//                             Dir::E => (b.pos.0 + 1, b.pos.1),
//                             Dir::S => (b.pos.0, b.pos.1 + 1),
//                             Dir::W => (b.pos.0 - 1, b.pos.1),
//                         };
//                     } else {
//                         b.pos = new_pos;
//                         return true;
//                     }
//                 }
//                 return false;
//             } else {
//                 b.pos = new_pos;
//                 return true;
//             }
//         }

//         false
//     }
// }

async fn input(example: bool) -> (Rob, DunRaw) {
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
    let h = map_lines.len();
    let mut w = 0;
    for y in 0..map_lines.len() {
        let map_vals: Vec<char> = map_lines[y].chars().collect();
        if w == 0 {
            w = map_vals.len();
        }
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

    let dim = (w, h);
    let rob = rob_opt.expect("No robot constructed");
    let dun = DunRaw { walls, boxes, dim };

    (rob, dun)
}

pub async fn solve(submit: bool, example: bool) {
    let (mut rob, raw_dun) = input(example).await;
    let mut dun = Dun::from_raw(&raw_dun);
    // println!("{:?}", dun);
    dun.print(&rob);

    for _i in 0..rob.mov_list.len() {
        rob.march(&mut dun);

        println!("");
        dun.print(&rob);
    }

    let answer = dun.get_gps_sum();

    final_answer(answer, submit, DAY, SOL).await;
}
