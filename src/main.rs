mod func_map;
mod generator;
mod input;
mod prelude;
mod solutions;

use clap::App;
use clap::Arg;
use colored::Colorize;
use std::io::Write;

#[tokio::main]
async fn main() {
    let matches = App::new("Bytomancer's Advent of Code Solver")
        .version("1.0")
        .author("Bytomancer")
        .about("Once properly configured, this repository can solve Advent of Code problems automatically")
        .arg(
            Arg::with_name("FUNCTION")
                .help("Specify the function to run (in the format d00s0)")
                .required(true),
        )
        .arg(
            Arg::with_name("submit")
                .short("s")
                .long("submit")
                .help("Submit option, automatically pushes your answer to AOC"),
        )
        .arg(
            Arg::with_name("example")
                .short("e")
                .long("example")
                .help("Attempt to find and use an example provided on the problem page"),
        )
        .arg(
            Arg::with_name("generate")
                .short("g")
                .long("generate")
                .help("Generate a new rust file for the solution, may create a fennel file as well if the command ends in lua"),
        )
        .get_matches();

    let func = match matches.value_of("FUNCTION") {
        Some(val) => val.to_owned(),
        None => {
            println!();
            let prompt = String::from("Enter the function you'd like to run").on_green();
            print!("{}", prompt);
            print!(" ");
            std::io::stdout().flush().unwrap();
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            buffer.trim().to_owned()
        }
    };

    let submit = matches.is_present("submit");
    let example = matches.is_present("example");
    let generate = matches.is_present("generate");

    if generate {
        generator::generate_new_functions(&func).await;
    } else {
        solve(&func, submit, example).await;
    }
}

async fn solve(func: &str, submit: bool, example: bool) -> () {
    println!(
        "\n{}\n",
        format!(
            "    Solving {}",
            format!(" {} ", func).black().on_yellow().bold()
        )
        .bold()
        .on_blue()
    );

    use std::time::Instant;
    let now = Instant::now();

    func_map::run(&func, submit, example).await;

    println!(
        "{}\n",
        format!("Execution time: {:.2?}", now.elapsed())
            .black()
            .on_white()
    );
}
