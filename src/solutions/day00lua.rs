use super::final_answer;
use super::input_raw;

const DAY: u8 = 0;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw.lines().map(|item| item.to_owned()).collect();

    lines
}

pub async fn d00s1(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, 1, input);

    final_answer(answer, submit, DAY, 1).await;
}

pub async fn d00s2(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, 2, input);

    final_answer(answer, submit, DAY, 2).await;
}
