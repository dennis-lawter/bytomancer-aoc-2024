use mlua::FromLuaMulti;
use mlua::Function;

pub fn run_lua_script<T: std::fmt::Display + FromLuaMulti>(
    day: u8,
    solution: u8,
    input: Vec<String>,
) -> T {
    let lua = unsafe { mlua::Lua::unsafe_new() };
    let solver = lua
        .load(include_str!("../../lua_src/runner.lua"))
        .eval::<Function>()
        .expect("Failed to load solver");
    solver
        .call::<T>((day, solution, input))
        .expect("Solver failed")
}
