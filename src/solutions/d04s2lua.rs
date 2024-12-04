use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 04;
const SOL: u8 = 2;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw.lines().map(|item| item.to_owned()).collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, SOL, input);

    final_answer(answer, submit, DAY, SOL).await;
}
