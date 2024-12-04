use colored::Colorize;

pub async fn run(func: &str, submit: bool, example: bool) {
    match func {
        "d00s1" => crate::solutions::day00::d00s1(submit, example).await,
        "d00s2" => crate::solutions::day00::d00s2(submit, example).await,
        "d01s1" => crate::solutions::day01::d01s1(submit, example).await,
        "d01s2" => crate::solutions::day01::d01s2(submit, example).await,
        "d02s1" => crate::solutions::day02::d02s1(submit, example).await,
        "d02s2" => crate::solutions::day02::d02s2(submit, example).await,
        "d03s1" => crate::solutions::day03::d03s1(submit, example).await,
        "d03s2" => crate::solutions::day03::d03s2(submit, example).await,
        "d00s1lua" => crate::solutions::day00lua::d00s1(submit, example).await,
        "d00s2lua" => crate::solutions::day00lua::d00s2(submit, example).await,
        "d01s1lua" => crate::solutions::day01lua::d01s1(submit, example).await,
        "d01s2lua" => crate::solutions::day01lua::d01s2(submit, example).await,
        "d02s1lua" => crate::solutions::day02lua::d02s1(submit, example).await,
        "d02s2lua" => crate::solutions::day02lua::d02s2(submit, example).await,
        "d03s1lua" => crate::solutions::day03lua::d03s1(submit, example).await,
        "d03s2lua" => crate::solutions::day03lua::d03s2(submit, example).await,
        // EXPANSION HERE
        invalid => {
            println!(
                "{}\n",
                format!("Unrecognized function: {}", invalid.bold()).on_red()
            )
        }
    }
}
