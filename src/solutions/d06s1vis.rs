use std::collections::HashSet;

use raylib::prelude::*;

use super::d06s1::Dir;
use super::d06s1::Dungeon;
use super::d06s1::Guard;
use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 06;
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
    // let mut answer = 0;

    let dun = Dungeon { map: input };
    let w = dun.get_w();
    let h = dun.get_h();
    let guard_pos = dun.start_guard_pos();
    let mut guard = Guard {
        x: guard_pos.0,
        y: guard_pos.1,
        dir: Dir::N,
        dun,
    };
    let mut visited: HashSet<(usize, usize, Dir)> = HashSet::new();
    visited.insert((guard_pos.0, guard_pos.1, Dir::N));
    let scale = 16;
    let fps = 120;
    let (mut rl, thread) = raylib::init()
        .size(w as i32 * scale, h as i32 * scale)
        .title("d06s1vis")
        .build();
    rl.set_target_fps(fps);

    let mut frame_time_start = fps * 5;
    let mut frame_time_end = fps * 5;
    let mut finished = false;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        // d.draw_pixel(x, y, color);
        for y in 0..h {
            for x in 0..w {
                let adj_x = x as i32 * scale;
                let adj_y = y as i32 * scale;
                if guard.x == x && guard.y == y {
                    // d.draw_rectangle(adj_x, adj_y, scale, scale, Color::TOMATO);
                    d.draw_circle(
                        adj_x + (scale / 2),
                        adj_y + (scale / 2),
                        scale as f32 / 2.0,
                        Color::TOMATO,
                    );
                    match guard.dir {
                        Dir::N => {
                            d.draw_rectangle(
                                adj_x,
                                adj_y + (scale / 2),
                                scale,
                                scale / 2,
                                Color::TOMATO,
                            );
                        }
                        Dir::E => {
                            d.draw_rectangle(adj_x, adj_y, scale / 2, scale, Color::TOMATO);
                        }
                        Dir::S => {
                            d.draw_rectangle(adj_x, adj_y, scale, scale / 2, Color::TOMATO);
                        }
                        Dir::W => {
                            d.draw_rectangle(
                                adj_x + (scale / 2),
                                adj_y,
                                scale / 2,
                                scale,
                                Color::TOMATO,
                            );
                        }
                    }
                } else if guard.dun.is_obs(x, y) {
                    d.draw_rectangle(adj_x, adj_y, scale, scale, Color::BLACK);
                } else {
                    let mut found = false;
                    if visited.contains(&(x, y, Dir::N)) {
                        found = true;
                        let center = Vector2 {
                            x: (adj_x + scale / 2) as f32,
                            y: (adj_y + scale / 4) as f32,
                        };
                        d.draw_poly(center, 3, scale as f32 / 4.0, 270.0, Color::BLUE);
                    }
                    if visited.contains(&(x, y, Dir::E)) {
                        found = true;
                        let center = Vector2 {
                            x: adj_x as f32 + scale as f32 * 0.75,
                            y: (adj_y + scale / 2) as f32,
                        };
                        d.draw_poly(center, 3, scale as f32 / 4.0, 0.0, Color::BLUE);
                    }
                    if visited.contains(&(x, y, Dir::S)) {
                        found = true;
                        let center = Vector2 {
                            x: (adj_x + scale / 2) as f32,
                            y: adj_y as f32 + scale as f32 * 0.75,
                        };
                        d.draw_poly(center, 3, scale as f32 / 4.0, 90.0, Color::BLUE);
                    }
                    if visited.contains(&(x, y, Dir::W)) {
                        found = true;
                        let center = Vector2 {
                            x: (adj_x + scale / 4) as f32,
                            y: (adj_y + scale / 2) as f32,
                        };
                        d.draw_poly(center, 3, scale as f32 / 4.0, 180.0, Color::BLUE);
                    }

                    if finished && found {
                        d.draw_circle(
                            adj_x + scale / 2,
                            adj_y + scale / 2,
                            scale as f32 / 4.0,
                            Color::TOMATO,
                        );
                    }
                }
            }
        }

        if frame_time_start > 0 {
            frame_time_start -= 1;
        } else if guard.march() {
            let pos = (guard.x, guard.y, guard.dir);
            visited.insert(pos);
        } else if frame_time_end > 0 {
            finished = true;
            frame_time_end -= 1;
        } else {
            break;
        }
    }

    let visited_locations: HashSet<(usize, usize)> =
        visited.iter().map(|item| (item.0, item.1)).collect();

    final_answer(visited_locations.len(), submit, DAY, SOL).await;
}
