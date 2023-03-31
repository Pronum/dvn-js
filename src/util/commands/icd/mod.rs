use std::{fs::{self, File}, path::Path, collections::HashMap, hash::Hash, io::{Write}};
use flate2::read::GzDecoder;
use tar::Archive;
use colored::Colorize;

use serde::{Deserialize, Serialize};
use reqwest::blocking::get;
use serde_json::{Value};

use std::env;

pub fn run(installation_type: String) {
    let env_path = env::current_dir().unwrap().to_owned();
    
    let dmut_dir_path: String          = env_path.clone().display().to_string();
    let _nmut_pkg_main: &str           = "/node_modules/";

    if !Path::new(&String::from(dmut_dir_path.clone() + "/package.json".to_string().as_str())).exists() {
        return println!("{} {}", "Project is not initialized!".bright_black().bold(), "Please use \"init\" command before installing a package.".bright_cyan().bold());
    }

    let node_modules_strp = String::from(dmut_dir_path.clone() + _nmut_pkg_main.clone());
    let node_modules_path  = Path::new(node_modules_strp.as_str());
    let node_modules_clone = node_modules_path.clone();

    if !node_modules_clone.exists() {
        fs::create_dir(node_modules_clone).unwrap_or_default();
        println!("{}: {} {} {}", "dvn.base.dir".bold().bright_cyan(), "Created".bold().bright_black(), "#".bold(), "/node_modules/".bold().bright_yellow());
    }

    println!("{}: {} {}{}{} {}", "registry.npmjs.org".bold().bright_cyan(), "Adding module".bold().bright_black(), package_name.bold().bright_magenta(), "@".bold(), registry_data_latest_ver.clone().to_string().bold().bright_magenta(), "to package.json".bold().bright_black());

    let parsed_file_path    = String::from(dmut_dir_path.clone() + "/package.json".to_string().as_str());
    let parsed_file_content = fs::read_to_string(parsed_file_path.clone());

    match package_paramater.as_str() {
        "--only-dep"     => module_parse_odep(package_name.clone(), registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap()),
        "-od"            => module_parse_odep(package_name.clone(), registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap()),
        "--only-dev-dep" => module_parse_ovdp(package_name.clone(), registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap()),
        "-odd"           => module_parse_ovdp(package_name.clone(), registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap()),
        _                => module_parse_both(package_name.clone(), registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap()),
    }

    // Download data from registry_data_latest_tar
    let tgz_package_name        = String::from(node_modules_clone.clone().to_str().unwrap().to_string() + "tmp-pkg-".to_string().as_str() + format!("{}.{}", package_name.clone().to_lowercase().to_string().chars()
    .map(|x| match x { 
        '/' => '-',
        _ => x
    }).collect::<String>(), "tgz").as_str()).to_string();
    fn data_from_registry(registry_data_latest_tar: String, tgz_package_name: String, package_name: String, node_modules_clone: String) {
        let tgz_package_in_stream = get(registry_data_latest_tar.clone()).expect("Request failed while contacting with registry.npmjs.org tarball.");
        let tgz_package_ostream_path = Path::new(&tgz_package_name);
        let mut tgz_package_out_stream= File::create(tgz_package_ostream_path).expect("Could not create the file");
        print!("[{}] | {}: {} {} {}", "/".bold().bright_green(), "registry.npmjs.org".bold().bright_cyan(), "Recieving data of".bold().bright_black(), "#".bold(), registry_data_latest_tar.clone().to_string().bold().bright_purple());
        #[allow(unused_must_use)] {
            tgz_package_out_stream.write_all(&tgz_package_in_stream.bytes().unwrap()).and(Ok(|| {
            })).and(Ok(|| {
                print!("\r");
                print!("[{}] | {}: {} {} {}", "-".bold().bright_green(), "registry.npmjs.org".bold().bright_cyan(), "Recieving data of".bold().bright_black(), "#".bold(), registry_data_latest_tar.clone().to_string().bold().bright_purple())
            })).and(Ok(|| {
                print!("\r");
                print!("[{}] | {}: {} {} {}", "\\".bold().bright_green(), "registry.npmjs.org".bold().bright_cyan(), "Recieving data of".bold().bright_black(), "#".bold(), registry_data_latest_tar.clone().to_string().bold().bright_purple())
            })).and_then(|_| Ok({
                print!("\r");
                print!("[{}] | {}: {} {} {}", "DONE".bold().bright_green(), "registry.npmjs.org".bold().bright_cyan(), "Recieved all data of".bold().bright_black(), "#".bold(), registry_data_latest_tar.clone().to_string().bold().bright_purple())
            })).and_then(|_| Ok({
                print!("\n");
                println!("[{}] | {}: {} {} {}", "DONWLOADED".bold().bright_green(), "registry.npmjs.org".bold().bright_cyan(), "Package".bold().bright_black(), "#".bold(), package_name.clone().to_string().bold().bright_purple());
                println!("[{}]  | {}: {} {} {}", "UNPACKING".bold().red(), "registry.npmjs.org".bold().bright_cyan(), "Package".bold().bright_black(), "#".bold(), package_name.clone().to_string().bold().bright_blue());

                let unpack_to_dist = String::from(node_modules_clone.clone());
                let unpack_to_pkgd = String::from(node_modules_clone.clone() + package_name.as_str());

                let path = tgz_package_ostream_path;

                let tar_gz = File::open(path)?;
                let tar = GzDecoder::new(tar_gz);
                let mut archive = Archive::new(tar);
                archive.unpack(String::from(unpack_to_dist.clone()));
                println!("[{}]   | {}{}", "UNPACKED".bold().bright_green(), "To Path: /node_modules/".bold().bright_cyan(), package_name.clone().to_string().bold().bright_green());
                if Path::new(&String::from(unpack_to_pkgd.clone())).exists() {
                    fs::remove_dir_all(unpack_to_pkgd.clone());
                    println!("[{}]    | {} {}{}", "REMOVED".bold().bright_green(), "Old Package Directory".bold().bright_cyan(), "/node_modules/".bold().bright_black(), package_name.clone().to_string().bold().bright_green());
                }
                fs::rename(unpack_to_dist + "package", unpack_to_pkgd.clone());
                fs::remove_file(path);
                println!("[{}] | {}", "CLEANED_UP".bold().bright_green(), "/node_modules/".bold().bright_cyan());
            }));
        }
    }

    #[allow(unused_must_use)] {
        data_from_registry(registry_data_latest_tar, tgz_package_name, package_name, node_modules_clone.clone().to_string_lossy().to_string());
    }
}