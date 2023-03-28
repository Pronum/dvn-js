mod errlog;
mod commands;

pub mod parser {
    use crate::util::commands;

    pub fn init(command_paramater: Option<String>) {
        if command_paramater == Some("--skip-option-string".to_string()) {
            return commands::init::run(Some("--argument-is-null").to_owned().unwrap().to_string());
        }

        return commands::init::run(command_paramater.to_owned().unwrap().to_string());
    }

    pub fn install(_package_name: &str, command_paramater: String) {
        return commands::install::run(_package_name.to_owned(), command_paramater.to_owned());
    }
}