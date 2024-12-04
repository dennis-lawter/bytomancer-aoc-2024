use std::fs;

pub async fn generate_new_functions(func: &str) -> () {
    let placeholder = r#"// AUTOMATED EXPANSION PLACEHOLDER"#;

    let day = func[1..=2]
        .parse::<u8>()
        .expect("The day could not be parsed; did you provide the format d00s0?");

    let sol = func[4..=4]
        .parse::<u8>()
        .expect("The day could not be parsed; did you provide the format d00s0?");

    {
        let func_map_path = "src/func_map.rs";
        let func_map_replacement = format!(
            r#""{func}" => crate::solutions::{func}::solve(submit, example).await,
        {placeholder}"#
        );
        let current_func_map =
            fs::read_to_string(func_map_path).expect(&build_file_err(func_map_path));
        let new_func_map = current_func_map.replace(placeholder, &func_map_replacement);
        fs::write(func_map_path, new_func_map).expect(&write_file_err(func_map_path));
    }

    {
        let mod_path = "src/solutions/mod.rs";
        let mod_path_replacement = format!(
            r#"pub mod {func};
{placeholder}"#
        );
        let current_mod = fs::read_to_string(mod_path).expect(&build_file_err(mod_path));
        let new_mod = current_mod.replace(placeholder, &mod_path_replacement);
        fs::write(mod_path, new_mod).expect(&write_file_err(mod_path));
    }

    if func.ends_with("lua") {
        // Custom fennel automation
        let short_func = func.replace("lua", "");
        let in_path = format!("lua_src/solutions/d00.fnl");
        let out_path = format!("lua_src/solutions/{short_func}.fnl");
        let template = fs::read_to_string(&in_path).expect(&build_file_err(&in_path));
        fs::write(&out_path, template).expect(&write_file_err(&out_path));

        {
            let in_path = format!("src/solutions/d00s{sol}lua.rs");
            let out_path = format!("src/solutions/{func}.rs");
            let template = fs::read_to_string(&in_path).expect(&build_file_err(&in_path));
            let new_template = template.replace(
                "const DAY: u8 = 00;",
                &format!("const DAY: u8 = {day:0>2};"),
            );
            fs::write(&out_path, new_template).expect(&write_file_err(&out_path));
        }
    } else {
        let in_path = format!("src/solutions/d00s{sol}.rs");
        let out_path = format!("src/solutions/{func}.rs");
        let template = fs::read_to_string(&in_path).expect(&build_file_err(&in_path));
        let new_template = template.replace(
            "const DAY: u8 = 00;",
            &format!("const DAY: u8 = {day:0>2};"),
        );
        fs::write(&out_path, new_template).expect(&write_file_err(&out_path));
    }
}

fn build_file_err(path: &str) -> String {
    format!("{path} cannot be found; are you in the project directory?")
}

fn write_file_err(path: &str) -> String {
    format!("{path} could not be written.")
}
