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
                .help("Specify the function to run (in the format d00s0")
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
    match &func[..] {
        // INITIAL SOLUTIONS
        "d00s1" => solutions::day00::d00s1(submit, example).await,
        "d00s2" => solutions::day00::d00s2(submit, example).await,

        "d01s1" => solutions::day01::d01s1(submit, example).await,
        "d01s2" => solutions::day01::d01s2(submit, example).await,

        // REVISED APPROACHES
        "d00s1rev" => solutions::day00rev::d00s1(submit, example).await,
        "d00s2rev" => solutions::day00rev::d00s2(submit, example).await,

        // LUA
        "d00s1lua" => solutions::day00lua::d00s1(submit, example).await,
        "d00s2lua" => solutions::day00lua::d00s2(submit, example).await,

        "d01s1lua" => solutions::day01lua::d01s1(submit, example).await,
        "d01s2lua" => solutions::day01lua::d01s2(submit, example).await,

        // VISUALIZATIONS

        // ERR
        invalid => {
            println!(
                "{}\n",
                format!("Unrecognized function: {}", invalid.bold()).on_red()
            )
        }
    }
    println!(
        "{}\n",
        format!("Execution time: {:.2?}", now.elapsed())
            .black()
            .on_white()
    );
}
