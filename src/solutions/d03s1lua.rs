use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 03;
const SOL: u8 = 1;

async fn input(example: bool) -> String {
    input_raw(DAY, example).await
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, SOL, input);

    final_answer(answer, submit, DAY, SOL).await;
}
