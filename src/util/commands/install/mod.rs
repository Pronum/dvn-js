use std::{fs};
use colored::Colorize;
use std::env;

pub fn run(_argument: String) {
    let env_path = env::current_dir().unwrap().to_owned();
    
    let dmut_dir_path: String          = env_path.clone().display().to_string();
    let nmut_dir_path         = env_path.display();
    let nmut_pkg_main: &str           = "/node_modules/";

    // https://registry.npmjs.org/[PACKAGE WITHOUT ARRAYS]
    // Please continue working on functionality!
}