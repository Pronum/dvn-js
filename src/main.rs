use std::{env};

use colored::Colorize;

mod errlog;
mod util;

fn main() {
    let args: Vec<_> = env::args().collect();
    
    // Default commands
    let command: Option<String> = args.get(1).and_then(|e| {
        e.parse().ok()
    });
    let options: Option<String> = args.get(2).and_then(|a| {
        a.parse().ok()
    });

    // Install command
    let package_name: Option<String> = args.get(2).and_then(|s| {
        s.parse().ok()
    });
    let package_paramater: Option<String> = args.get(3).and_then(|p| {
        p.parse().ok()
    });

    // Check if command is install and use package, paramater and if paramater is none we use --save-dev for default.
    // We will possibly need to make functions inside util::parser::functioname(it's paramater) so we can use custom paramaters to each command 
    // Need more work OwO.

    if command.is_none() {
        return errlog::log_unknown_command_no_name();
    }

    let command_name = command.clone().unwrap().to_string();

    match command.clone().unwrap().as_str() {
        "init" => {

            if !options.is_none() {
                return util::parser::init(options);
            }
                
            return util::parser::init(Some("--skip-option-string".to_string()));
        },
        "install" => {
            if package_name.is_none() {
                return errlog::log_red_message(format!("{}, {}", "Could not find the package name in arguments".bold().bright_black(), "Possibly forgot to add the package name?".bold().bright_cyan()).as_str());
            }

            if package_paramater.is_none() {
                return util::parser::install(package_name.clone().unwrap().as_str(), "--save-dev".to_string());
            }

            return util::parser::install(package_name.clone().unwrap().as_str(), package_paramater.clone().unwrap());
        },
        _ => {
            return errlog::log_unknown_command(command_name);
        }
    }
}