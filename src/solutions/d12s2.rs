use std::collections::HashSet;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 12;
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
                    // does the tile S of us match?
                    if safe_input_check(input, (*x, *y + 1)) == Some(species) {
                        // if the tile SW doesn't match species, can't steal its edge
                        if safe_input_check(input, (*x - 1, *y + 1)) == Some(species) {
                            edges += 1;
                        }
                    } else {
                        edges += 1;
                    }
                }
            }
            None => {
                // If tile S doesn't match, can't steal edge
                if safe_input_check(input, (*x, *y + 1)) != Some(species) {
                    edges += 1;
                }
            }
        }
        match safe_input_check(input, (*x + 1, *y)) {
            Some(test_species) => {
                if test_species != species {
                    // does the tile S of us match?
                    if safe_input_check(input, (*x, *y + 1)) == Some(species) {
                        // if the tile SE doesn't match species, can't steal its edge
                        if safe_input_check(input, (*x + 1, *y + 1)) == Some(species) {
                            edges += 1;
                        }
                    } else {
                        edges += 1;
                    }
                }
            }
            None => {
                // If tile S doesn't match, can't steal edge
                if safe_input_check(input, (*x, *y + 1)) != Some(species) {
                    edges += 1;
                }
            }
        }
        match safe_input_check(input, (*x, *y - 1)) {
            Some(test_species) => {
                if test_species != species {
                    // does the tile E of us match?
                    if safe_input_check(input, (*x + 1, *y)) == Some(species) {
                        // if the tile NE doesn't match species, can't steal its edge
                        if safe_input_check(input, (*x + 1, *y - 1)) == Some(species) {
                            edges += 1;
                        }
                    } else {
                        edges += 1;
                    }
                }
            }
            None => {
                // If tile E doesn't match, can't steal edge
                if safe_input_check(input, (*x + 1, *y)) != Some(species) {
                    edges += 1;
                }
            }
        }
        match safe_input_check(input, (*x, *y + 1)) {
            Some(test_species) => {
                if test_species != species {
                    // does the tile E of us match?
                    if safe_input_check(input, (*x + 1, *y)) == Some(species) {
                        // if the tile SE doesn't match species, can't steal its edge
                        if safe_input_check(input, (*x + 1, *y + 1)) == Some(species) {
                            edges += 1;
                        }
                    } else {
                        edges += 1;
                    }
                }
            }
            None => {
                // If tile E doesn't match, can't steal edge
                if safe_input_check(input, (*x + 1, *y)) != Some(species) {
                    edges += 1;
                }
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
