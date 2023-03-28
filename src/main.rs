use std::{env};

mod errlog;
mod util;

fn main() {
    let args: Vec<_> = env::args().collect();

    let command: Option<String> = args.get(1).and_then(|e| {
        e.parse().ok()
    });
    let options: Option<String> = args.get(2).and_then(|a| {
        a.parse().ok()
    });

    if command.is_none() {
        return errlog::log_unknown_command();
    }

    if !options.is_none() {
        return util::parser(command.unwrap().to_string(), options);
    }
        
    return util::parser(command.unwrap().to_string(), Some("--skip-option-string".to_string()));
}