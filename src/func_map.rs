use colored::Colorize;

pub async fn run(func: &str, submit: bool, example: bool) {
    match func {
        "d00s1" => crate::solutions::d00s1::solve(submit, example).await,
        "d00s2" => crate::solutions::d00s2::solve(submit, example).await,
        "d01s1" => crate::solutions::d01s1::solve(submit, example).await,
        "d01s2" => crate::solutions::d01s2::solve(submit, example).await,
        "d02s1" => crate::solutions::d02s1::solve(submit, example).await,
        "d02s2" => crate::solutions::d02s2::solve(submit, example).await,
        "d03s1" => crate::solutions::d03s1::solve(submit, example).await,
        "d03s2" => crate::solutions::d03s2::solve(submit, example).await,
        "d00s1lua" => crate::solutions::d00s1lua::solve(submit, example).await,
        "d00s2lua" => crate::solutions::d00s2lua::solve(submit, example).await,
        "d01s1lua" => crate::solutions::d01s1lua::solve(submit, example).await,
        "d01s2lua" => crate::solutions::d01s2lua::solve(submit, example).await,
        "d02s1lua" => crate::solutions::d02s1lua::solve(submit, example).await,
        "d02s2lua" => crate::solutions::d02s2lua::solve(submit, example).await,
        "d03s1lua" => crate::solutions::d03s1lua::solve(submit, example).await,
        "d03s2lua" => crate::solutions::d03s2lua::solve(submit, example).await,
        "d04s1" => crate::solutions::d04s1::solve(submit, example).await,
        "d04s2" => crate::solutions::d04s2::solve(submit, example).await,
        "d04s1lua" => crate::solutions::d04s1lua::solve(submit, example).await,
        "d04s2lua" => crate::solutions::d04s2lua::solve(submit, example).await,
        // AUTOMATED EXPANSION PLACEHOLDER
        invalid => {
            println!(
                "{}\n",
                format!("Unrecognized function: {}", invalid.bold()).on_red()
            )
        }
    }
}
