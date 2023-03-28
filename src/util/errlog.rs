use colored::Colorize;
#[allow(dead_code)]
pub fn log_unknown_command() {
    return println!("{} \"{}\" {}", "Unknown command, Please specifiy a command to use".red(), "dvn".bold().bright_blue(), "Type --help to get commands list.".red());
}