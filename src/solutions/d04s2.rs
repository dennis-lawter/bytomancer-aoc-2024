use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 04;
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
    let mut ans = 0;
    let h = input.len();
    let w = input[0].len();
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if input[y][x] == 'A' {
                let tl = input[y - 1][x - 1];
                let tr = input[y - 1][x + 1];
                let bl = input[y + 1][x - 1];
                let br = input[y + 1][x + 1];
                let tlbr = format!("{tl}A{br}");
                let trbl = format!("{tr}A{bl}");
                if tlbr == "MAS" || tlbr == "SAM" {
                    if trbl == "MAS" || trbl == "SAM" {
                        println!("{x} {y}");
                        ans += 1;
                    }
                }
            }
        }
    }
    final_answer(ans, submit, DAY, SOL).await;
}
