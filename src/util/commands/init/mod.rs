use std::{fs, io::{stdin, stdout, Write}, path::Path};
use colored::Colorize;
use std::env;

pub fn run(_argument: String) {
    let env_path = env::current_dir().unwrap().to_owned();
    
    let dmut_dir_path: String          = env_path.clone().display().to_string();
    let nmut_dir_path         = env_path.display();
    let nmut_file_name: &str           = "/package.json";
    let nmut_pkg_name         = String::from(nmut_dir_path.to_string()).split("\\").last().unwrap().to_string();

    fn skip_terminal_io(dmut_dir_path: String, nmut_file_name: &str, nmut_pkg_name: String, nmut_dir_path: std::path::Display) {
        if Path::new(&String::from(dmut_dir_path.clone() + nmut_file_name)).exists() {
            return println!("{} {}", "Project is already initialized!".bright_black().bold(), "If you still want to create new package.json remove the old one and rerun the command!".red().bold());
        }

        fn default_file_content(package_name: &str) -> String {
            return format!("{}\n  \"name\": \"{}\",\n  \"version\": \"1.0.0\",\n  \"author\": \"Unspecified\"\n{}", "{", package_name, "}").to_string();
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
            if Path::new(&String::from(dmut_dir_path.clone() + nmut_file_name)).exists() {
                return println!("{} {}", "Project is already initialized!".bright_black().bold(), "If you still want to create new package.json remove the old one and rerun the command!".red().bold());
            }

            print!("{} ({}) {} ", "Enter Package Name".bold().black(), nmut_pkg_name.bold().red(), ":".bold().bright_black());
            let _first_flush   = stdout().flush();
            let mut package_name_input    = String::new();
            stdin().read_line(&mut package_name_input).expect("Using default directory name.");

            print!("{} ({}) {} ", "Enter Package Version".bold().black(), "1.0.0".bold().red(), ":".bold().bright_black());
            let _middle_flush   = stdout().flush();
            let mut package_version_input = String::new();
            stdin().read_line(&mut package_version_input).expect("Using default version.");

            print!("{} ({}) {} ", "Enter Package Author".bold().black(), "Unspecified".bold().red(), ":".bold().bright_black());
            let _last_flush   = stdout().flush();
            let mut package_author_input = String::new();
            stdin().read_line(&mut package_author_input).expect("Using default author.");

            if package_name_input.is_empty() || package_name_input.trim().len() < 1 {
                package_name_input = nmut_pkg_name;
            }

            if package_version_input.is_empty() || package_version_input.trim().len() < 1 {
                package_version_input = "1.0.0".to_string();
            }

            if package_author_input.is_empty() || package_author_input.trim().len() < 1 {
                package_author_input = "Unspecified".to_string();
            }

            fn default_file_content(package_name_input: String, package_version_input: String, package_author_input: String) -> String {
                return format!("{}\n  \"name\": \"{}\",\n  \"version\": \"{}\",\n  \"author\": \"{}\"\n{}", "{",  package_name_input.trim().to_lowercase().to_string().chars()
                .map(|x| match x { 
                    ' ' => '-',
                    _ => x
                }).collect::<String>(), package_version_input.trim(), package_author_input.trim(), "}").to_string();
            }
        
            fs::write(dmut_dir_path + nmut_file_name, default_file_content(package_name_input, package_version_input, package_author_input).to_string()).ok();
            println!("{} \"{}\" {}", "Done initializing your project".bright_black().bold(), nmut_dir_path.to_string().bright_yellow().bold(), ".");
        }
    }
}