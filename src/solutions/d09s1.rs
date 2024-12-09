use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 09;
const SOL: u8 = 1;

async fn input(example: bool) -> Vec<char> {
    input_raw(DAY, example)
        .await
        .replace("\n", "")
        .chars()
        .collect()
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut checksum = 0;
    let mut block = true;

    let mut ids: Vec<i64> = Vec::with_capacity(input.len() * 10);
    let mut id = 0;
    let mut num_of_values = 0;

    for c in input {
        let cv = String::from(c)
            .parse::<i64>()
            .expect(&format!("Parse failure for {c}"));
        if block {
            for _ in 0..cv {
                ids.push(id);
                num_of_values += 1;
            }
            id += 1;
        } else {
            for _ in 0..cv {
                ids.push(-1);
            }
        }
        block = !block;
    }

    println!("{:?}", ids);

    let mut defragged: Vec<i64> = Vec::with_capacity(ids.len());
    let mut tail_i = ids.len();

    for (_i, id) in ids.iter().enumerate() {
        if defragged.len() >= num_of_values {
            break;
        }
        if *id == -1 {
            tail_i -= 1;
            while ids[tail_i] == -1 {
                tail_i -= 1;
            }
            defragged.push(ids[tail_i]);
        } else {
            defragged.push(*id);
        }
    }

    println!("\n\n{:?}", defragged);

    for (i, id) in defragged.iter().enumerate() {
        checksum += i as u64 * *id as u64;
    }

    final_answer(checksum, submit, DAY, SOL).await;
}
