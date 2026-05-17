use crate::{parser, KEYS};
use gmod::lua::State;

pub unsafe extern "C-unwind" fn get_string(lua: State) -> i32 {
    let key = lua.check_string(1).to_string();
    let guard = KEYS.lock().unwrap();

    if let Some(value) = guard.get(&key) {
        lua.push_string(value);
    } else if lua.get_top() >= 2 && !lua.is_nil(2) {
        lua.push_value(2);
    } else {
        lua.push_nil();
    }
    1
}

pub unsafe extern "C-unwind" fn get_number(lua: State) -> i32 {
    let key = lua.check_string(1).to_string();
    let guard = KEYS.lock().unwrap();
    let parsed = guard.get(&key).and_then(|v| v.parse::<f64>().ok());
    drop(guard);

    if let Some(n) = parsed {
        lua.push_number(n);
    } else if lua.get_top() >= 2 && !lua.is_nil(2) {
        lua.push_value(2);
    } else {
        lua.push_nil();
    }
    1
}

pub unsafe extern "C-unwind" fn get_integer(lua: State) -> i32 {
    let key = lua.check_string(1).to_string();
    let guard = KEYS.lock().unwrap();
    let parsed = guard.get(&key).and_then(|v| v.parse::<f64>().ok());
    drop(guard);

    if let Some(n) = parsed {
        lua.push_number(n.floor());
    } else if lua.get_top() >= 2 && !lua.is_nil(2) {
        lua.push_number(lua.check_number(2).floor());
    } else {
        lua.push_nil();
    }
    1
}

pub unsafe extern "C-unwind" fn get_boolean(lua: State) -> i32 {
    let key = lua.check_string(1).to_string();
    let guard = KEYS.lock().unwrap();
    let result = guard.get(&key).and_then(|v| match v.to_lowercase().as_str() {
        "true"  => Some(true),
        "false" => Some(false),
        _       => None,
    });
    drop(guard);

    if let Some(b) = result {
        lua.push_boolean(b);
    } else if lua.get_top() >= 2 && !lua.is_nil(2) {
        lua.push_value(2);
    } else {
        lua.push_nil();
    }
    1
}

pub unsafe extern "C-unwind" fn get_keys(lua: State) -> i32 {
    let guard = KEYS.lock().unwrap();
    lua.new_table();
    for (i, key) in guard.keys().enumerate() {
        lua.push_integer((i + 1) as isize);
        lua.push_string(key);
        lua.set_table(-3);
    }
    1
}

pub unsafe extern "C-unwind" fn parse(lua: State) -> i32 {
    let body = lua.check_string(1).to_string();
    let (parsed, errors) = parser::parse(&body);

    lua.new_table();
    for (key, value) in &parsed {
        lua.push_string(key.as_str());
        lua.push_string(value.as_str());
        lua.set_table(-3);
    }

    if errors.is_empty() {
        lua.push_nil();
    } else {
        lua.new_table();
        for (i, err) in errors.iter().enumerate() {
            lua.push_integer((i + 1) as isize);
            lua.push_string(err.as_str());
            lua.set_table(-3);
        }
    }
    2
}

pub unsafe extern "C-unwind" fn load(lua: State) -> i32 {
    let path = lua.check_string(1).to_string();

    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let (parsed, _) = parser::parse(&content);
            *KEYS.lock().unwrap() = parsed;
        }
        Err(_) => {
            lua.get_global(lua_string!("print"));
            lua.push_string("Attempted to load non-existent dotenv file at:");
            lua.push_string(path.as_str());
            lua.call(2, 0);
        }
    }
    0
}

pub unsafe extern "C-unwind" fn env_call(lua: State) -> i32 {
    lua.remove(1);
    get_string(lua)
}
