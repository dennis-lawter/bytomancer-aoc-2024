use super::final_answer;
use super::input_raw;

const DAY: u8 = 02;

async fn input(example: bool) -> Vec<Vec<u64>> {
    let raw = input_raw(DAY, example).await;
    raw.lines()
        .map(|i| i.to_owned())
        .filter(|item| item.len() > 0)
        .map(|i| {
            i.split_whitespace()
                .map(|j| j.parse::<u64>().expect("Failed to parse during input"))
                .collect()
        })
        .collect()
}

pub async fn d02s1(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, 1, input);

    final_answer(answer, submit, DAY, 1).await;
}

pub async fn d02s2(submit: bool, example: bool) {
    let input = input(example).await;

    let answer: u64 = super::lua_runner::run_lua_script(DAY, 2, input);

    final_answer(answer, submit, DAY, 2).await;
}
