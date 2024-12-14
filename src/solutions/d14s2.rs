use crate::solutions::d14s1::*;

use super::solutions::final_answer;

const DAY: u8 = 14;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    let mut robots = input.clone();

    let mut answer = 0;

    let stdin = std::io::stdin();
    loop {
        for robot in robots.iter_mut() {
            robot.march(WIDTH, HEIGHT);
        }
        answer += 1;
        if answer > 6500 {
            print_map(&robots, false);
            println!("");
            println!("time: {answer}");
            println!("type c then enter to confirm!  Enter to skip!");

            let mut iterator = std::io::BufRead::lines(stdin.lock());
            let line1 = iterator.next().unwrap().unwrap();
            if line1 == "c" {
                break;
            }
        }
    }

    final_answer(answer, submit, DAY, SOL).await;
}
