use std::collections::HashMap;
use std::collections::HashSet;

use color_eyre::owo_colors::OwoColorize;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 08;
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}
#[derive(Debug)]
pub struct Node {
    pub loc: Point,
    pub id: char,
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    let mut all_nodes: Vec<Node> = vec![];

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '.' => {}
                v => all_nodes.push(Node {
                    loc: Point {
                        x: x as u64,
                        y: y as u64,
                    },
                    id: *v,
                }),
            }
        }
    }

    let mut grouped: HashMap<char, Vec<Node>> = HashMap::new();
    for node in all_nodes {
        let id = node.id;
        if !grouped.contains_key(&id) {
            grouped.insert(id, vec![]);
        }
        let list = grouped.get_mut(&id).unwrap();
        list.push(node);
    }

    let mut antinodes: Vec<Point> = vec![];
    let h = input.len();
    let w = input[0].len();

    for (_k, nodes) in grouped {
        for n1 in nodes.iter() {
            for n2 in nodes.iter() {
                if n1.loc == n2.loc {
                    continue;
                }
                let y_diff = n1.loc.y as i64 - n2.loc.y as i64;
                let x_diff = n1.loc.x as i64 - n2.loc.x as i64;
                let mut new_y = n1.loc.y as i64;
                let mut new_x = n1.loc.x as i64;
                while new_y >= 0 && new_y < w as i64 && new_x >= 0 && new_x < h as i64 {
                    let a = Point {
                        x: new_x as u64,
                        y: new_y as u64,
                    };

                    // println!("{:?} for {k}", a);

                    antinodes.push(a);

                    new_y += y_diff;
                    new_x += x_diff;
                }
            }
        }
    }

    // for a in &antinodes {
    //     println!("{:?}", a);
    // }

    for (y, line) in input.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val == '.' {
                let mut a_found = false;
                for a in &antinodes {
                    if a.x == x as u64 && a.y == y as u64 {
                        a_found = true;
                    }
                }
                if a_found {
                    print!("{}", format!(".").on_red());
                } else {
                    print!("{}", val);
                }
            } else {
                // print!("{}", val);
                let mut a_found = false;
                for a in &antinodes {
                    if a.x == x as u64 && a.y == y as u64 {
                        a_found = true;
                    }
                }
                if a_found {
                    print!("{}", val.on_red());
                } else {
                    print!("{}", val);
                }
            }
        }
        println!("");
    }

    let mut unique_list: HashSet<(u64, u64)> = HashSet::new();
    for a in &antinodes {
        unique_list.insert((a.x, a.y));
    }

    final_answer(unique_list.len(), submit, DAY, SOL).await;
}
