use std::{fs, path::Path, collections::HashMap};
use colored::Colorize;

use serde::{Deserialize, Serialize};
use reqwest::blocking::get;
use serde_json::{Value};

use std::env;

pub fn run(package_name: String, package_paramater: String) {
    let env_path = env::current_dir().unwrap().to_owned();
    
    let dmut_dir_path: String          = env_path.clone().display().to_string();
    let _nmut_pkg_main: &str           = "/node_modules/";

    if !Path::new(&String::from(dmut_dir_path.clone() + "/package.json".to_string().as_str())).exists() {
        return println!("{} {}", "Project is not initialized!".bright_black().bold(), "Please use \"init\" command before installing a package.".bright_cyan().bold());
    }

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
        
        // Create node_modules if not found

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

        pub fn merge(v: &Value, fields: &HashMap<String, Value>) -> Value {
            match v {
                Value::Object(m) => {
                    let mut m = m.clone();
                    for (k, v) in fields {
                        m.insert(k.clone(), Value::Object(v.as_object().unwrap().clone()));
                    }
                    Value::Object(m)
                }
                v => v.clone(),
            }
        }

        #[allow(unused_variables)]
        pub fn module_save(package_name: String, package_version: String, parsed_file_path: String, parsed_file_content: String) {
            #[derive(Debug, Serialize, Deserialize)]
            struct PackageJsonFileStruct {
                dependencies: Value
            }
            
            let package_json: serde_json::Value = serde_json::from_str(&parsed_file_content).expect("package.json file was not formatted correctly!");
            let mut dependencie = HashMap::new();
            let mut package = serde_json::Map::new();
            package.insert(package_name.clone(), Value::String("^".to_string() + package_version.as_str()));
            dependencie.insert("dependencies".to_string(), Value::Object(package));

            let merged_data = merge(&package_json, &dependencie);

            fs::write(parsed_file_path, serde_json::to_string_pretty(&merged_data).unwrap().to_string()).unwrap();

            println!("{}: {} {}{}{} {}", "registry.npmjs.org".bold().bright_cyan(), "Added module".bold().bright_black(), package_name.clone().bold().bright_magenta(), "@".bold(), package_version.clone().to_string().bold().bright_magenta(), "to package.json".bold().bright_black());
        }

        #[allow(unused_variables)]
        pub fn module_save_dev(package_name: String, package_version: String, parsed_file_path: String, parsed_file_content: String) {
            let package_json: serde_json::Value = serde_json::from_str(&parsed_file_content).expect("package.json file was not formatted correctly!");

        }

        match package_paramater.as_str() {
            "--save"     => module_save    (package_name, registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap()),
            "--save-dev" => module_save_dev(package_name, registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap()),
            _            => module_save    (package_name, registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap()),
        }

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
        // Then add it as a (dev or not) dependencie(s) in package.json
        // If no package.json then just alert the user 🤷‍♂️

    }
}