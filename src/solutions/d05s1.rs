use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 05;
const SOL: u8 = 1;

#[derive(Default, Debug)]
struct Input {
    ord: Vec<(u64, u64)>,
    upd: Vec<Vec<u64>>,
}

async fn input(example: bool) -> Input {
    let raw = input_raw(DAY, example).await;
    let mut out = Input::default();

    let groups: Vec<String> = raw
        .split("\n\n")
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    let g1 = &groups[0];
    for g1l in g1.lines() {
        let g1ls: Vec<&str> = g1l.split("|").collect();
        let g1ls1 = g1ls[0].parse::<u64>().expect("not int");
        let g1ls2 = g1ls[1].parse::<u64>().expect("not int");
        out.ord.push((g1ls1, g1ls2));
    }
    let g2 = &groups[1];
    for g2l in g2.lines() {
        let mut g2v = vec![];
        for g in g2l.split(",") {
            g2v.push(g.parse().unwrap());
        }
        out.upd.push(g2v);
    }

    out
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    // println!("{:?}", input);
    // panic!("EARLY TERMINATION FOR TESTING");
    let mut answer = 0;
    for upd in input.upd {
        let mut valid = true;
        for (left, right) in input.ord.iter() {
            match upd.iter().position(|item| item == left) {
                Some(upd_left_pos) => match upd.iter().position(|item| item == right) {
                    Some(upd_right_pos) => {
                        println!("testing: {left} {right} {upd_left_pos} {upd_right_pos}");
                        if upd_right_pos < upd_left_pos {
                            valid = false;
                        }
                    }
                    None => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }
        }
        if valid {
            let middle_idx = upd.len() / 2;
            let middle = upd[middle_idx];
            answer += middle;
            println!("{:?}", upd);
        }
    }
    final_answer(answer, submit, DAY, SOL).await;
}
