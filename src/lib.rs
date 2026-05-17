#[macro_use]
extern crate gmod;

mod handlers;
mod parser;

use gmod::lua::State;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub(crate) static KEYS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    lua.new_table();

    macro_rules! reg {
        ($name:expr, $fn:expr) => {
            lua.push_function($fn);
            lua.set_field(-2, lua_string!($name));
        };
    }

    reg!("getString",  handlers::get_string);
    reg!("getNumber",  handlers::get_number);
    reg!("getInteger", handlers::get_integer);
    reg!("getBoolean", handlers::get_boolean);
    reg!("getKeys",    handlers::get_keys);
    reg!("parse",      handlers::parse);
    reg!("load",       handlers::load);

    lua.new_table();
    lua.push_function(handlers::env_call);
    lua.set_field(-2, lua_string!("__call"));
    lua.set_metatable(-2);

    lua.set_global(lua_string!("env"));
    0
}

#[gmod13_close]
unsafe fn gmod13_close(_lua: State) -> i32 {
    KEYS.lock().unwrap().clear();
    0
}
