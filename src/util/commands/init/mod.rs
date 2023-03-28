use std::{fs};
use colored::Colorize;
use std::env;

pub fn run(_argument: String) {
    let env_path = env::current_dir().unwrap().to_owned();
    
    let dmut_dir_path: String          = env_path.clone().display().to_string();
    let nmut_dir_path         = env_path.display();
    let nmut_file_name: &str           = "/package.json";
    let nmut_pkg_name         = String::from(nmut_dir_path.to_string()).split("\\").last().unwrap().to_string();

    fn skip_terminal_io(dmut_dir_path: String, nmut_file_name: &str, nmut_pkg_name: String, nmut_dir_path: std::path::Display) {
        fn default_file_content(package_name: &str) -> String {
            return format!("{}\n   \"name\": \"{}\",\n   \"version\": \"1.0.0\",\n   \"author\": \"Unspecified\"\n{}", "{", package_name, "}").to_string();
        }
    
        fs::write(dmut_dir_path + nmut_file_name, default_file_content(nmut_pkg_name.as_str()).to_string()).ok();
        println!("{} \"{}\" {}", "Done initializing your project".bright_black().bold(), nmut_dir_path.to_string().bright_yellow().bold(), ".");
    }

    match _argument.as_str() {
        "-y"       => skip_terminal_io(dmut_dir_path, nmut_file_name, nmut_pkg_name, nmut_dir_path),
        "-i"       => skip_terminal_io(dmut_dir_path, nmut_file_name, nmut_pkg_name, nmut_dir_path),
        "--yes"    => skip_terminal_io(dmut_dir_path, nmut_file_name, nmut_pkg_name, nmut_dir_path),
        "--ignore" => skip_terminal_io(dmut_dir_path, nmut_file_name, nmut_pkg_name, nmut_dir_path),
        _ => {
            println!("{}", "See you later baby!".bold().red());
            // Command line questions like Name: blablabla
            // Version: 1.0.0
            // Author: Unspecified
        }
    }
}