use raylib::color::Color;
use raylib::prelude::RaylibDraw;

use crate::solutions::d14s1::*;

use super::solutions::final_answer;

const DAY: u8 = 14;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    let mut robots = input.clone();

    let mut answer = 0;

    let stdin = std::io::stdin();
    // loop {
    //     for robot in robots.iter_mut() {
    //         robot.march(WIDTH, HEIGHT);
    //     }
    //     answer += 1;
    //     if answer > 6500 {
    //         print_map(&robots, false);
    //         println!("");
    //         println!("time: {answer}");
    //         println!("type c then enter to confirm!  Enter to skip!");

    //         let mut iterator = std::io::BufRead::lines(stdin.lock());
    //         let line1 = iterator.next().unwrap().unwrap();
    //         if line1 == "c" {
    //             break;
    //         }
    //     }
    // }

    let scale = 20;
    let fps = 1200;
    let (mut rl, thread) = raylib::init()
        .size(WIDTH * scale, HEIGHT * scale)
        .title("d06s1vis")
        .build();
    rl.set_target_fps(fps);

    let mut frame_time_start = fps * 5;
    let mut frame_time_end = fps * 5;
    let final_frame = 6577;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for robot in robots.iter() {
            let x = robot.pos.0 * scale;
            let y = robot.pos.1 * scale;
            d.draw_rectangle(x, y, scale, scale, Color::GREEN);
        }
        if frame_time_start > 0 {
            frame_time_start -= 1;
            continue;
        }
        if answer < final_frame {
            for robot in robots.iter_mut() {
                robot.march(WIDTH, HEIGHT);
            }
            answer += 1;
        } else if frame_time_end > 0 {
            frame_time_end -= 1;
        } else {
            break;
        }
    }
    final_answer(answer, submit, DAY, SOL).await;
}
