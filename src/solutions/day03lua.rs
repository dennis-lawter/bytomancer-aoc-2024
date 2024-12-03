use super::final_answer;
use super::input_raw;

const DAY: u8 = 03;

async fn input(example: bool) -> String {
    input_raw(DAY, example).await
}

pub async fn d03s1(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, 1, input);

    final_answer(answer, submit, DAY, 1).await;
}

pub async fn d03s2(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, 2, input);

    final_answer(answer, submit, DAY, 2).await;
}
