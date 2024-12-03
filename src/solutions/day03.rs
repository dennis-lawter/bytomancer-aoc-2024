use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 03;

async fn input(example: bool) -> String {
    input_raw(DAY, example).await
}

pub async fn d03s1(submit: bool, example: bool) {
    let input = input(example).await;
    let mut ans = 0;
    let re = Regex::new(r#"mul\(\d+,\d+\)"#).unwrap();
    let re2 = Regex::new(r#"(\d+),(\d+)"#).unwrap();
    for cap_group in re.captures_iter(&input) {
        for cap in cap_group.iter() {
            match cap {
                Some(c) => {
                    let c_str = c.as_str();
                    if c_str.starts_with("mul") {
                        let inner_caps = re2.captures(c_str).unwrap();
                        let left = inner_caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        let right = inner_caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
                        ans += left * right;
                    }
                }
                None => {}
            }
        }
    }
    final_answer(ans, submit, DAY, 1).await;
}

pub async fn d03s2(submit: bool, example: bool) {
    let input = input(example).await;
    let mut ans = 0;
    let mut en = true;
    let re = Regex::new(r#"mul\(\d+,\d+\)|do\(\)|don't\(\)"#).unwrap();
    let re2 = Regex::new(r#"(\d+),(\d+)"#).unwrap();
    for cap_group in re.captures_iter(&input) {
        for cap in cap_group.iter() {
            match cap {
                Some(c) => {
                    let c_str = c.as_str();
                    if c_str.starts_with("mul") {
                        if en {
                            let inner_caps = re2.captures(c_str).unwrap();
                            let left = inner_caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                            let right = inner_caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
                            ans += left * right;
                        }
                    } else if c_str.starts_with("don't()") {
                        en = false;
                    } else if c_str.starts_with("do()") {
                        en = true;
                    }
                }
                None => {}
            }
        }
    }
    final_answer(ans, submit, DAY, 2).await;
}
