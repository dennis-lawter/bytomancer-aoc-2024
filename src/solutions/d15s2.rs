use colored::Colorize;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 15;
const SOL: u8 = 2;

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
            if dun.can_push_box(new_pos, dir) {
                dun.push_box(new_pos, dir);
                self.pos = new_pos;
                return;
            } else {
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
impl Box {}

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
    BoxLeft,
    BoxRight,
    Empty,
}
impl DunTile {
    pub fn to_char(&self) -> char {
        match self {
            DunTile::Wall => '#',
            DunTile::BoxLeft => '[',
            DunTile::BoxRight => ']',
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
            let mut row: Vec<DunTile> = Vec::with_capacity(raw.dim.0 * 2);
            for x in 0..raw.dim.0 {
                let pos = (x as i32, y as i32);
                let (left_tile, right_tile) = if raw.is_wall(pos) {
                    (DunTile::Wall, DunTile::Wall)
                } else if raw.is_box(pos) {
                    (DunTile::BoxLeft, DunTile::BoxRight)
                } else {
                    (DunTile::Empty, DunTile::Empty)
                };
                row.push(left_tile);
                row.push(right_tile);
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
        self.is_left_box(pos) || self.is_right_box(pos)
    }

    fn is_left_box(&self, pos: (i32, i32)) -> bool {
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        self.data[y][x] == DunTile::BoxLeft
    }

    fn is_right_box(&self, pos: (i32, i32)) -> bool {
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        self.data[y][x] == DunTile::BoxRight
    }

    fn is_empty(&self, pos: (i32, i32)) -> bool {
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        self.data[y][x] == DunTile::Empty
    }

    // TODO: fix for p2
    // NOTES TO SELF (NTS):
    // The problem with this is that I was thinking too hard about
    // the N/S push case.
    // In the E/W case, it's much simpler.
    // You could even do it with an iterator.
    // Go down the line, until empty|wall
    // if wall, fail
    // if empty, set empty to (left if W push, right if E push)
    // flip every tile as the iterator returns to pos
    // set pos to empty
    // I'm too tired tonight to finish this.
    // I need sleep.
    fn can_push_box(&mut self, pos: (i32, i32), dir: Dir) -> bool {
        let affected_boxes = if self.is_left_box(pos) {
            (pos, (pos.0 + 1, pos.1))
        } else {
            ((pos.0 - 1, pos.1), pos)
        };
        let (lbox, rbox) = affected_boxes;
        let new_lbox = match dir {
            Dir::N => (lbox.0, lbox.1 - 1),
            Dir::E => (lbox.0 + 2, lbox.1),
            Dir::S => (lbox.0, lbox.1 + 1),
            Dir::W => (lbox.0 - 2, lbox.1),
        };
        let new_rbox = match dir {
            Dir::N => (rbox.0, rbox.1 - 1),
            Dir::E => (rbox.0 + 2, rbox.1),
            Dir::S => (rbox.0, rbox.1 + 1),
            Dir::W => (rbox.0 - 2, rbox.1),
        };
        if self.is_wall(new_lbox) || self.is_wall(new_rbox) {
            return false;
        } else if self.is_box(new_lbox) || self.is_box(new_rbox) {
            if self.can_push_box(new_lbox, dir) && self.can_push_box(new_rbox, dir) {
                return true;
            } else {
                return false;
            }
        } else {
            assert!(self.is_empty(new_lbox));
            assert!(self.is_empty(new_rbox));

            return true;

            // let lbox_x = lbox.0 as usize;
            // let lbox_y = lbox.1 as usize;
            // let lbox_new_x = new_lbox.0 as usize;
            // let lbox_new_y = new_lbox.1 as usize;
            // self.data[lbox_new_y][lbox_new_x] = DunTile::BoxLeft;
            // self.data[lbox_y][lbox_x] = DunTile::Empty;

            // let rbox_x = rbox.0 as usize;
            // let rbox_y = rbox.1 as usize;
            // let rbox_new_x = new_rbox.0 as usize;
            // let rbox_new_y = new_rbox.1 as usize;
            // self.data[rbox_new_y][rbox_new_x] = DunTile::BoxLeft;
            // self.data[rbox_y][rbox_x] = DunTile::Empty;
        }

        // false
    }

    fn push_box(&mut self, pos: (i32, i32), dir: Dir) {
        let affected_boxes = if self.is_left_box(pos) {
            (pos, (pos.0 + 1, pos.1))
        } else {
            ((pos.0 - 1, pos.1), pos)
        };
        let (lbox, rbox) = affected_boxes;
        let new_lbox = match dir {
            Dir::N => (lbox.0, lbox.1 - 1),
            Dir::E => (lbox.0 + 2, lbox.1),
            Dir::S => (lbox.0, lbox.1 + 1),
            Dir::W => (lbox.0 - 2, lbox.1),
        };
        let new_rbox = match dir {
            Dir::N => (rbox.0, rbox.1 - 1),
            Dir::E => (rbox.0 + 2, rbox.1),
            Dir::S => (rbox.0, rbox.1 + 1),
            Dir::W => (rbox.0 - 2, rbox.1),
        };

        assert!(self.is_empty(new_lbox));
        assert!(self.is_empty(new_rbox));

        let lbox_x = lbox.0 as usize;
        let lbox_y = lbox.1 as usize;
        let lbox_new_x = new_lbox.0 as usize;
        let lbox_new_y = new_lbox.1 as usize;
        self.data[lbox_new_y][lbox_new_x] = DunTile::BoxLeft;
        self.data[lbox_y][lbox_x] = DunTile::Empty;

        let rbox_x = rbox.0 as usize;
        let rbox_y = rbox.1 as usize;
        let rbox_new_x = new_rbox.0 as usize;
        let rbox_new_y = new_rbox.1 as usize;
        self.data[rbox_new_y][rbox_new_x] = DunTile::BoxLeft;
        self.data[rbox_y][rbox_x] = DunTile::Empty;
    }

    fn get_gps_sum(&self) -> usize {
        let mut ans = 0;
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.is_left_box((x as i32, y as i32)) {
                    ans += y * 100 + x
                }
            }
        }
        ans
    }
}

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
    rob.pos.0 *= 2;
    dun.print(&rob);

    for _i in 0..rob.mov_list.len() {
        rob.march(&mut dun);

        println!("");
        dun.print(&rob);
    }

    let answer = dun.get_gps_sum();

    final_answer(answer, submit, DAY, SOL).await;
}
