use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 01;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw.lines().map(|item| item.to_owned()).collect();

    lines
}

pub async fn d01s1(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, 1, input);

    final_answer(answer, submit, DAY, 1).await;
}

pub async fn d01s2(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, 2, input);

    final_answer(answer, submit, DAY, 2).await;
}
