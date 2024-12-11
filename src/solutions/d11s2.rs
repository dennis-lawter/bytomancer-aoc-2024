use std::collections::HashMap;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 11;
const SOL: u8 = 2;

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
    let input = input(example).await;
    let num_of_blinks = 75;

    let mut input_m: HashMap<u64, usize> = HashMap::new();
    for k in input {
        add_to(&mut input_m, k, 1);
    }
    // println!("{:?}\n", input_m);

    for _blink in 0..num_of_blinks {
        let mut new_input_m = HashMap::new();
        for (k, v) in input_m.iter() {
            let teststr = format!("{}", k);
            let dcnt = teststr.len();
            if *k == 0 {
                add_to(&mut new_input_m, 1, *v);
            } else if dcnt % 2 == 0 {
                let pow = 10_u64.pow(dcnt as u32 / 2);
                let left = *k / pow;
                let right = *k % pow;
                add_to(&mut new_input_m, left, *v);
                add_to(&mut new_input_m, right, *v);
            } else {
                add_to(&mut new_input_m, *k * 2024, *v);
            }
        }
        // println!("{:?}\n", new_input_m);
        input_m = new_input_m.clone();
    }

    let mut answer = 0;

    for (_, v) in input_m.iter() {
        answer += *v;
    }

    final_answer(answer, submit, DAY, SOL).await;
}

fn add_to(input_m: &mut HashMap<u64, usize>, key: u64, add: usize) {
    if input_m.contains_key(&key) {
        input_m.insert(key, input_m[&key] + add);
    } else {
        input_m.insert(key, add);
    }
}
