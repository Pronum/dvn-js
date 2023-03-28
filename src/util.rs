mod errlog;
mod commands;

pub fn parser(command_name: String, command_paramater: Option<String>) {
    match command_name.as_str() {
        "init" => {
            if command_paramater == Some("--skip-option-string".to_string()) {
                return commands::init::run(Some("--argument-is-null").to_owned().unwrap().to_string());
            }

            commands::init::run(command_paramater.to_owned().unwrap().to_string());
        },
        "install" => {
            if command_paramater == Some("--skip-option-string".to_string()) {
                return commands::init::run(Some("--argument-is-null").to_owned().unwrap().to_string());
            }

            commands::init::run(command_paramater.to_owned().unwrap().to_string());
        },
        _ => {
            errlog::log_unknown_command();
        }
    }
}