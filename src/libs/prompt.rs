use std::env;
use std::error::Error;
use libs;
use tools;

pub fn get_prompt(status: i32) -> String {
    let home = tools::get_user_home();
    let user;
    match env::var("USER") {
        Ok(x) => user = x,
        Err(e) => {
            println!("cicada: env USER error: {:?}", e);
            return String::from("cicada >> ");
        }
    }
    let hostname = tools::get_hostname();
    let _current_dir;
    match env::current_dir() {
        Ok(x) => _current_dir = x,
        Err(e) => {
            println!("cicada: env current_dir error: {}", e.description());
            return format!("({})$ ", libs::colored::red("no current dir"));
        }
    }
    let current_dir;
    match _current_dir.to_str() {
        Some(x) => current_dir = x,
        None => {
            println!("cicada: to_str error");
            return String::from("cicada >> ");
        }
    }
    let _tokens: Vec<&str> = current_dir.split('/').collect();

    let last;
    match _tokens.last() {
        Some(x) => last = x,
        None => {
            println!("cicada: prompt token last error");
            return String::from("cicada >> ");
        }
    }
    let pwd: String;
    if last.is_empty() {
        pwd = String::from("/");
    } else if current_dir == home {
        pwd = String::from("~");
    } else {
        pwd = last.to_string();
    }

    let mut prompt = if status == 0 {
        format!(
            "{}@{}: {}$ ",
            libs::colored::green(user.as_str()),
            libs::colored::green(hostname.as_str()),
            libs::colored::green(pwd.as_str())
        )
    } else {
        format!(
            "{}@{}: {}$ ",
            libs::colored::red(user.as_str()),
            libs::colored::red(hostname.as_str()),
            libs::colored::red(pwd.as_str())
        )
    };
    if let Ok(x) = env::var("VIRTUAL_ENV") {
        if x != "" {
            let _tokens: Vec<&str> = x.split('/').collect();
            let env_name;
            match _tokens.last() {
                Some(x) => env_name = x,
                None => {
                    println!("prompt token last error");
                    return String::from("cicada >> ");
                }
            }
            prompt = format!("({}){}", libs::colored::green(env_name), prompt);
        }
    }
    prompt
}
