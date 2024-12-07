use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 07;
const SOL: u8 = 1;

async fn input(example: bool) -> Vec<(u64, Vec<u64>)> {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();
    let mut out: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in lines {
        let line_split: Vec<String> = line.split(": ").map(|i| i.to_owned()).collect();
        let k = line_split[0].parse().expect("Failed to parse int");
        let v: Vec<u64> = line_split[1]
            .split(" ")
            .map(|i| i.parse::<u64>().expect("Failed to parse int"))
            .collect();
        out.push((k, v));
    }

    out
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut answer = 0;
    for (k, v) in input {
        if test_equation(k, v) {
            answer += k;
        }
    }
    final_answer(answer, submit, DAY, SOL).await;
}

pub fn perform_single_test(k: u64, v: &Vec<u64>, test_id: usize) -> bool {
    // let org_test_id = test_id;
    let mut test_id = test_id;
    let mut accum = 0;
    // let mut op_list = "".to_owned();
    for vv in v {
        if accum == 0 {
            accum = vv.clone().into();
            continue;
        }
        let t = test_id % 3;
        test_id /= 3;
        if t == 0 {
            accum *= vv;
            // op_list = format!("{}{}", op_list, "*");
        } else if t == 1 {
            accum += vv;
            // op_list = format!("{}{}", op_list, "+");
        } else {
            let mut vvc = vv.clone();
            while vvc > 0 {
                accum *= 10;
                vvc /= 10;
            }
            accum += vv;
        }
    }

    // println!("{} {k}: {:?} {}", accum == k, v, op_list);

    accum == k
}

pub fn test_equation(k: u64, v: Vec<u64>) -> bool {
    let num_of_tests = usize::pow(3, (v.len() - 1) as u32) - 1;
    for test_id in 0..=num_of_tests {
        if perform_single_test(k, &v, test_id) {
            // println!("{k}: {:?}", v);
            return true;
        }
    }

    false
}
