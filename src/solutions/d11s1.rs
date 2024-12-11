use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 11;
const SOL: u8 = 1;

async fn input(example: bool) -> Vec<u64> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .replace("\n", "")
        .split(" ")
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .map(|item| item.parse::<u64>().expect("Parse failure"))
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let mut input = input(example).await;
    let num_of_blinks = 25;

    for _blink in 0..num_of_blinks {
        let mut i = 0;
        while i < input.len() {
            let test = input[i];
            let teststr = format!("{}", test);
            let dcnt = teststr.len();
            if test == 0 {
                input[i] = 1;
            } else if dcnt % 2 == 0 {
                let pow = 10_u64.pow(dcnt as u32 / 2);
                let left = test / pow;
                let right = test % pow;
                input[i] = left;
                input.insert(i + 1, right);
                i += 1;
            } else {
                input[i] *= 2024;
            }
            i += 1;
        }
    }

    let answer = input.len();

    final_answer(answer, submit, DAY, SOL).await;
}
