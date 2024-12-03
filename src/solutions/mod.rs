use colored::Colorize;
use regex::Regex;
use reqwest::Url;

use crate::input::get_example_as_string;
use crate::input::get_input_as_string;

pub mod lua_runner;

pub mod day00;
pub mod day00lua;
pub mod day00rev;

pub mod day01;
pub mod day01lua;

pub mod day02;
pub mod day02lua;

pub async fn input_raw(day: u8, example: bool) -> String {
    if example {
        get_example_as_string(day).await
    } else {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            crate::prelude::YEAR,
            day
        )
        .to_string();
        get_input_as_string(&url).await
    }
}

pub async fn final_answer<T: std::fmt::Display>(answer: T, submit: bool, day: u8, level: u8) {
    println!(
        "\n{}",
        format!(
            "   Solution {}",
            format!(" {} ", answer).black().on_yellow().bold()
        )
        .bold()
        .on_blue()
    );

    if submit {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/answer",
            crate::prelude::YEAR,
            day
        );
        let request = format!("level={}&answer={}", level, answer);
        let response = perform_submit(&url, request).await;

        if response.contains("day-success") {
            println!("{}", "Accepted!".bold().on_blue());
        } else if response.contains("Did you already complete it?") {
            println!("{}", "Solution already accepted...".bold().on_white());
        } else if response.contains("left to wait.") {
            // You have 13s left to wait.
            let time_capture_regex = Regex::new(r"You have (.+) left to wait.").unwrap();
            let captures_result = time_capture_regex.captures(&response);
            println!("{}", "    SLOW DOWN    ".bold().on_red());
            match captures_result {
                Some(captures) => {
                    println!(
                        "Please wait {}.",
                        format!("{}", captures.get(1).unwrap().as_str())
                            .bold()
                            .on_red()
                    );
                }
                None => {
                    println!("Could not determine time before next submission...");
                }
            }
        } else {
            println!("{}", "Innaccurate!".bold().on_bright_red());
        }
    }
    println!();
}

async fn perform_submit(submit_url: &String, body: String) -> String {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    const KEY: &str = "SESSION";
    let session = dotenv::var(KEY).unwrap();
    let cookie = format!("session={}", session);
    let url = submit_url.parse::<Url>().unwrap();

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let response = client
        .post(url)
        .header("cookie", cookie)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();

    body
}
