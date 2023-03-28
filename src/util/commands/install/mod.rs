use std::{fs, collections::HashMap};
use colored::Colorize;

use serde::{Deserialize, Serialize};
use reqwest::blocking::get;
use serde_json::Value;

use std::env;

pub fn run(package_name: String, package_paramater: String) {
    let env_path = env::current_dir().unwrap().to_owned();
    
    let dmut_dir_path: String          = env_path.clone().display().to_string();
    let nmut_dir_path         = env_path.display();
    let nmut_pkg_main: &str           = "/node_modules/";

    let uri = format!("{}/{}", "https://registry.npmjs.org/", package_name).to_string();
    let res = get(String::from(uri)).unwrap();
    println!("{}: {}", "registry.npmjs.org".bold().bright_cyan(), res.status());
    if res.status().to_string() == "200 OK" {
        let registry_data_f = format!("{}", res.text().unwrap().to_string());

        #[derive(Debug, Deserialize, Serialize)]
        struct NPMRegistryStruct {
            _id: String,
            _rev: String,
            versions: Value
        }

        #[derive(Debug, Deserialize, Serialize)]
        struct  NPMRegistryVersions {
            version: String,
        }

        let registry_data_s: NPMRegistryStruct          = serde_json::from_str(registry_data_f.as_str()).unwrap();
        let registry_data_latest: (String, Value)       = registry_data_s.versions.as_object().unwrap().clone().into_iter().nth(registry_data_s.versions.as_object().unwrap().len() - 1).unwrap();
        let registry_data_latest_ver: String            = registry_data_latest.0;
        let registry_data_latest_dist   = registry_data_s.versions.get(registry_data_latest_ver.clone()).unwrap().get("dist");
        let registry_data_latest_tarq: String           = registry_data_latest_dist.unwrap().get("tarball").unwrap().to_string();
        let registry_data_latest_tar: String            = registry_data_latest_tarq[1..registry_data_latest_tarq.len() - 1].to_string();
        
        println!("{}: {} {}{}{}", "registry.npmjs.org".bold().bright_cyan(), "Configuring".bold().bright_black(), package_name.bold().bright_magenta(), "@".bold(), registry_data_latest_ver.clone().to_string().bold().bright_magenta());
        println!("{}: {} {} {}", "registry.npmjs.org".bold().bright_cyan(), "Tarball".bold().bright_black(), "#".bold(), registry_data_latest_tar.bold().bright_magenta());
        
        // Please continue working on functionality!
        // Create node_modules if not found
        // Download data from registry_data_latest_tar
        // Create a folder inside the node_modules on the name of the package 
        // Donwload the package in node_modules/folder_of_the_package and extract it 
        // Get the insider files of node_modules/folder_of_the_package/package and put it in node_modules/folder_of_the_package
        // Remove node_modules/folder_of_the_package/package
        // Install dependencies of node_modules/folder_of_the_package
        // And cache it in the %temp%/.dvn/cached/folder_of_the_package
        // Compress node_modules/folder_of_the_package
        // When using "dvn run script --unpack-cached" or "dvn script -u-c" it 
        // will unpack the compressed folder then it will run node process in the background

    }
}