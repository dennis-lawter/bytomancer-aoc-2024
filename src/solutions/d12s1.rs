use std::collections::HashSet;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 12;
const SOL: u8 = 1;

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

    let mut calculated: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let coord = (x, y);
            if calculated.contains(&coord) {
                continue;
            }

            let (area, perimeter) = calc_stats(coord, &input, &mut calculated);
            let species = input[y][x];
            println!("{species} A: {area}\tP: {perimeter}");
            // panic!("STOP FOR DEBUGGING");
            answer += area * perimeter;
        }
    }

    final_answer(answer, submit, DAY, SOL).await;
}

pub fn calc_stats(
    coord: (usize, usize),
    input: &Vec<Vec<char>>,
    calculated: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut answer = (0, 0);

    let all_coords = find_all_coords(coord, input, calculated);
    // uncomment if needed, then fix
    // let filtered_coords: HashSet<(usize, usize)> = all_coords.iter().collect();

    answer.0 = all_coords.len();
    // answer.1 = find_outer_perimeter(&all_coords);
    answer.1 = find_all_perimeter(&all_coords, input);

    answer
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

pub fn find_all_perimeter(all_coords: &Vec<(usize, usize)>, input: &Vec<Vec<char>>) -> usize {
    let start = all_coords[0];
    let species = safe_input_check(input, start).unwrap();
    // // let (start_x, start_y) = start;
    // match safe_input_check(input, start) {
    //     Some(ch) => {}
    //     None => {}
    // }
    let mut edges = 0;
    for coord in all_coords {
        let (x, y) = coord;
        match safe_input_check(input, (*x - 1, *y)) {
            Some(test_species) => {
                if test_species != species {
                    edges += 1;
                }
            }
            None => {
                edges += 1;
            }
        }
        match safe_input_check(input, (*x + 1, *y)) {
            Some(test_species) => {
                if test_species != species {
                    edges += 1;
                }
            }
            None => {
                edges += 1;
            }
        }
        match safe_input_check(input, (*x, *y - 1)) {
            Some(test_species) => {
                if test_species != species {
                    edges += 1;
                }
            }
            None => {
                edges += 1;
            }
        }
        match safe_input_check(input, (*x, *y + 1)) {
            Some(test_species) => {
                if test_species != species {
                    edges += 1;
                }
            }
            None => {
                edges += 1;
            }
        }
    }

    edges
}

pub fn safe_input_check(input: &Vec<Vec<char>>, coord: (usize, usize)) -> Option<char> {
    let (x, y) = coord;
    let x = input.get(y)?.get(x)?;
    Some(*x)
}

pub fn find_outer_perimeter(all_coords: &Vec<(usize, usize)>) -> usize {
    let all_coords: Vec<(i32, i32)> = all_coords
        .iter()
        .map(|(x, y)| (*x as i32, *y as i32))
        .collect();
    let mut steps = 0;

    // thanks to a little domain knowledge,
    // we know that the first inserted point is guaranteed
    // to be the top-most (first found) point.
    // xoo
    // ooo
    // ooo
    // we use the cursor to track the top left of each square
    // so we can SURELY march east over the edge of the starting point
    let start = all_coords[0];
    // println!("\tPERIMETER CALC START");
    // println!("\t{:?}", start);
    let mut cursor = (start.0 + 1, start.1); // march E once
    steps += 1;
    // println!("\t{:?}", cursor);
    // let mut points_visited = HashSet::new();
    let mut dir = Dir::E;
    // points_visited.insert(start);
    // points_visited.insert(cursor);

    while cursor != start {
        // we're tracing the edge, keeping the contents to our "right"
        // A   B
        //   ^
        // C   D
        // So we KNOW D must contain content
        // C cannot contain content unless we screwed up and wandered inside
        // can I turn left? If A is content, do so
        // can I go straight? if B is content, do so
        // else I must turn right...
        let x = cursor.0;
        let y = cursor.1;
        match dir {
            Dir::N => {
                // ?   ?
                //   ^
                // N   Y
                assert!(!all_coords.contains(&(x - 1, y)));
                assert!(all_coords.contains(&(x, y)));

                if all_coords.contains(&(x - 1, y - 1)) {
                    // turn left
                    dir = Dir::W;
                } else if all_coords.contains(&(x, y - 1)) {
                    // go straight
                } else {
                    // turn right
                    dir = Dir::E;
                }
            }
            Dir::E => {
                // N   ?
                //   >
                // Y   ?
                assert!(!all_coords.contains(&(x - 1, y - 1)));
                assert!(all_coords.contains(&(x - 1, y)));

                if all_coords.contains(&(x, y - 1)) {
                    // turn left
                    dir = Dir::N;
                } else if all_coords.contains(&(x, y)) {
                    // go straight
                } else {
                    // turn right
                    dir = Dir::S;
                }
            }
            Dir::S => {
                // Y   N
                //   v
                // ?   ?
                assert!(!all_coords.contains(&(x, y - 1)));
                assert!(all_coords.contains(&(x - 1, y - 1)));

                if all_coords.contains(&(x, y)) {
                    // turn left
                    dir = Dir::E;
                } else if all_coords.contains(&(x - 1, y)) {
                    // go straight
                } else {
                    // turn right
                    dir = Dir::W;
                }
            }
            Dir::W => {
                // ?   Y
                //   <
                // ?   N
                assert!(!all_coords.contains(&(x, y)));
                assert!(all_coords.contains(&(x, y - 1)));

                if all_coords.contains(&(x - 1, y)) {
                    // turn left
                    dir = Dir::S;
                } else if all_coords.contains(&(x - 1, y - 1)) {
                    // go straight
                } else {
                    // turn right
                    dir = Dir::N;
                }
            }
        }
        // finally do a march
        match dir {
            Dir::N => cursor.1 -= 1,
            Dir::E => cursor.0 += 1,
            Dir::S => cursor.1 += 1,
            Dir::W => cursor.0 -= 1,
        }
        // println!("\t{:?}", cursor);
        steps += 1;
    }
    // println!("\tPERM CALCULATED: {steps}");

    // points_visited.len()

    steps
}

pub fn find_all_coords(
    coord: (usize, usize),
    input: &Vec<Vec<char>>,
    calculated: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    if calculated.contains(&coord) {
        return vec![];
    }
    let mut answer = vec![coord];
    calculated.insert(coord);
    let (x, y) = coord;
    let h = input.len();
    let w = input[0].len();
    let species = input[y][x];
    if x > 0 {
        if input[y][x - 1] == species {
            answer.append(&mut find_all_coords((x - 1, y), input, calculated));
        }
    }
    if x < w - 1 {
        if input[y][x + 1] == species {
            answer.append(&mut find_all_coords((x + 1, y), input, calculated));
        }
    }
    if y > 0 {
        if input[y - 1][x] == species {
            answer.append(&mut find_all_coords((x, y - 1), input, calculated));
        }
    }
    if y < h - 1 {
        if input[y + 1][x] == species {
            answer.append(&mut find_all_coords((x, y + 1), input, calculated));
        }
    }
    answer
}
