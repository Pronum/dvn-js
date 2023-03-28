use colored::Colorize;
pub fn log_unknown_command_no_name() {
    return println!("{} \"{}\" {}", "Unknown command, Please specifiy a command to use".red(), "dvn".bold().bright_blue(), "Type --help to get commands list.".red());
}
pub fn log_unknown_command(command_name: String) {
    return println!("\"{}\" {} \"{}\" {}", command_name, "is an unknown command, Please specifiy a command to use".red(), "dvn".bold().bright_blue(), "Type --help to get commands list.".red());
}
pub fn log_red_message(message: &str) {
    return println!("{}", message);
}