use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 04;
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
    let mut ans = 0;
    let h = input.len();
    let w = input[0].len();
    for y in 0..h {
        for x in 0..w {
            if input[y][x] == 'X' {
                let ans_b4 = ans;
                // up
                if y >= 3 && input[y - 1][x] == 'M' {
                    if input[y - 2][x] == 'A' {
                        if input[y - 3][x] == 'S' {
                            ans += 1;
                            println!("{x} {y}");
                        }
                    }
                }
                // down
                if y + 3 < h && input[y + 1][x] == 'M' {
                    if input[y + 2][x] == 'A' {
                        if input[y + 3][x] == 'S' {
                            ans += 1;
                            println!("{x} {y}");
                        }
                    }
                }
                // left
                if x >= 3 && input[y][x - 1] == 'M' {
                    if input[y][x - 2] == 'A' {
                        if input[y][x - 3] == 'S' {
                            ans += 1;
                            println!("{x} {y}");
                        }
                    }
                }
                // right
                if x + 3 < w && input[y][x + 1] == 'M' {
                    if input[y][x + 2] == 'A' {
                        if input[y][x + 3] == 'S' {
                            ans += 1;
                            println!("{x} {y}");
                        }
                    }
                }
                // ul
                if y >= 3 && x >= 3 && input[y - 1][x - 1] == 'M' {
                    if input[y - 2][x - 2] == 'A' {
                        if input[y - 3][x - 3] == 'S' {
                            ans += 1;
                            println!("{x} {y}");
                        }
                    }
                }
                // ur
                if y >= 3 && x + 3 < h && input[y - 1][x + 1] == 'M' {
                    if input[y - 2][x + 2] == 'A' {
                        if input[y - 3][x + 3] == 'S' {
                            ans += 1;
                            println!("{x} {y}");
                        }
                    }
                }
                // dl
                if y + 3 < h && x >= 3 && input[y + 1][x - 1] == 'M' {
                    if input[y + 2][x - 2] == 'A' {
                        if input[y + 3][x - 3] == 'S' {
                            ans += 1;
                            println!("{x} {y}");
                        }
                    }
                }
                // dr
                if y + 3 < h && x + 3 < w && input[y + 1][x + 1] == 'M' {
                    if input[y + 2][x + 2] == 'A' {
                        if input[y + 3][x + 3] == 'S' {
                            ans += 1;
                            println!("{x} {y}");
                        }
                    }
                }
                if ans == ans_b4 {
                    println!("DIDNOTFIND {x} {y}");
                }
            }
        }
    }
    final_answer(ans, submit, DAY, SOL).await;
}
