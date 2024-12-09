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
    // let mut num_of_empty = 0;
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
                // num_of_empty += 1;
            }
        }
        block = !block;
    }

    println!("{:?}", ids);

    let mut defragged: Vec<i64> = Vec::with_capacity(ids.len());
    // let ids_copy = ids.clone();
    let mut tail_i = ids.len();

    for (_i, id) in ids.iter().enumerate() {
        // if num_of_empty == 0 {
        //     break;
        // }
        if defragged.len() >= num_of_values {
            break;
        }
        if *id == -1 {
            tail_i -= 1;
            while ids[tail_i] == -1 {
                tail_i -= 1;
            }
            defragged.push(ids[tail_i]);
            // num_of_empty -= 1;
        } else {
            defragged.push(*id);
        }
    }

    // let mut

    // println!("==========");

    // while num_of_empty > 0 {
    //     let left_empty_i = find_first_empty(&ids);
    //     let right_most_val_i = find_last_value(&ids);
    //     ids[left_empty_i] = ids[right_most_val_i];
    //     ids[right_most_val_i] = -1;
    //     num_of_empty -= 1;
    //     println!("\n\n{:?}", ids);
    // }

    // println!("==========");

    println!("\n\n{:?}", defragged);

    for (i, id) in defragged.iter().enumerate() {
        checksum += i as u64 * *id as u64;
    }

    final_answer(checksum, submit, DAY, SOL).await;
}

// pub fn find_first_empty(ids: &Vec<i64>) -> usize {
//     for (i, id) in ids.iter().enumerate() {
//         if *id == -1 {
//             return i;
//         }
//     }
//     usize::MAX
// }

// pub fn find_last_value(ids: &Vec<i64>) -> usize {
//     for (i, id) in ids.iter().rev().enumerate() {
//         if *id != -1 {
//             return i;
//         }
//     }
//     usize::MAX
// }
