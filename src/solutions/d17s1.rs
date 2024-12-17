use regex::Regex;

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 17;
const SOL: u8 = 1;

#[derive(Debug)]
pub struct Computer {
    pub reg_a: i64,
    pub reg_b: i64,
    pub reg_c: i64,
    pub prog: Vec<u8>,
    pub ip: usize,
}
impl Computer {
    pub fn new(reg_a: i64, reg_b: i64, reg_c: i64, prog: Vec<u8>) -> Self {
        Computer {
            reg_a,
            reg_b,
            reg_c,
            prog,
            ip: 0,
        }
    }

    pub fn execute(&mut self) -> Vec<i64> {
        let mut output: Vec<i64> = vec![];
        while self.ip < self.prog.len() {
            let inst_raw = self.prog[self.ip];
            let op_raw = self.prog[self.ip + 1];
            let inst = Inst::from_u8(inst_raw);
            let operand = if inst.is_combo() {
                match op_raw {
                    0..=3 => op_raw as i64,
                    4 => self.reg_a,
                    5 => self.reg_b,
                    6 => self.reg_c,
                    7 => panic!("There is no seven"),
                    _ => panic!("What operand is {op_raw}??"),
                }
            } else {
                op_raw as i64
            };

            let mut jumped = false;
            match inst {
                Inst::Adv => {
                    let numerator = self.reg_a;
                    let divisor = i64::pow(2, operand as u32);
                    let ans = numerator / divisor; // int division
                    self.reg_a = ans;
                }
                Inst::Bxl => {
                    println!("{} ^ {}", self.reg_b, operand);
                    let ans = self.reg_b ^ operand;
                    self.reg_b = ans;
                }
                Inst::Bst => {
                    let ans = operand % 8;
                    self.reg_b = ans;
                }
                Inst::Jnz => {
                    if self.reg_a != 0 {
                        jumped = true;
                        self.ip = operand as usize;
                    }
                }
                Inst::Bxc => {
                    let ans = self.reg_b ^ self.reg_c;
                    self.reg_b = ans;
                }
                Inst::Out => {
                    let ans = operand % 8;
                    output.push(ans);
                }
                Inst::Bdv => {
                    let numerator = self.reg_a;
                    let divisor = i64::pow(2, operand as u32);
                    let ans = numerator / divisor; // int division
                    self.reg_b = ans;
                }
                Inst::Cdv => {
                    let numerator = self.reg_a;
                    let divisor = i64::pow(2, operand as u32);
                    let ans = numerator / divisor; // int division
                    self.reg_c = ans;
                }
            }

            if !jumped {
                // standard increment
                self.ip += 2;
            }
        }

        output
    }
}

pub enum Inst {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl Inst {
    pub fn from_u8(raw: u8) -> Self {
        match raw {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid Inst: {raw}"),
        }
    }
    pub fn is_combo(&self) -> bool {
        match self {
            Inst::Adv => true,
            Inst::Bxl => false,
            Inst::Bst => true,
            Inst::Jnz => false,
            Inst::Bxc => false, // weird?
            Inst::Out => true,
            Inst::Bdv => true,
            Inst::Cdv => true,
        }
    }
}

async fn input(example: bool) -> Computer {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw.lines().map(|i| i.to_owned()).collect();
    let re_a = Regex::new(r#"Register A: (\d+)"#).expect("Invalid RE");
    let re_b = Regex::new(r#"Register B: (\d+)"#).expect("Invalid RE");
    let re_c = Regex::new(r#"Register C: (\d+)"#).expect("Invalid RE");
    let re_prog = Regex::new(r#"Program: (.+)"#).expect("Invalid RE");
    let cap_a = re_a.captures(&lines[0]).expect("No cap");
    let cap_b = re_b.captures(&lines[1]).expect("No cap");
    let cap_c = re_c.captures(&lines[2]).expect("No cap");
    // skip empty line
    let cap_prog = re_prog.captures(&lines[4]).expect("No cap");

    let reg_a = cap_a[1].parse::<i64>().expect("Invalid num");
    let reg_b = cap_b[1].parse::<i64>().expect("Invalid num");
    let reg_c = cap_c[1].parse::<i64>().expect("Invalid num");

    let prog: Vec<u8> = cap_prog[1]
        .split(",")
        .map(|i| i.parse().expect("invalid u8"))
        .collect();

    Computer::new(reg_a, reg_b, reg_c, prog)
}

pub async fn solve(submit: bool, example: bool) {
    let mut computer = input(example).await;
    println!("{:#?}", computer);
    let answer_raw = computer.execute();
    println!("{:#?}", computer);
    let answer_as_strings: Vec<String> = answer_raw
        .iter()
        .map(|i| format!("{i}").to_owned())
        .collect();
    let answer_final = answer_as_strings.join(",");
    final_answer(answer_final, submit, DAY, SOL).await;
}
