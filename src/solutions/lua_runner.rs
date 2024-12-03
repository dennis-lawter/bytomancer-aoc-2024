use mlua::FromLuaMulti;
use mlua::Function;
use mlua::IntoLuaMulti;

pub fn run_lua_script<T: std::fmt::Display + FromLuaMulti, I: IntoLuaMulti>(
    day: u8,
    solution: u8,
    input: I,
) -> T
where
{
    let lua = unsafe { mlua::Lua::unsafe_new() };
    let solver = lua
        .load(include_str!("../../lua_src/runner.lua"))
        .eval::<Function>()
        .expect("Failed to load solver");
    match solver.call::<T>((day, solution, input)) {
        Ok(answer) => answer,
        Err(e) => {
            let msg = e.to_string().replace("\\n", "\n").replace("\\t", "\t");
            panic!("{}", msg);
        }
    }
}
