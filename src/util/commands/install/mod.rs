use std::{fs::{self, File}, path::Path, collections::HashMap, hash::Hash, io::{Write}};
use flate2::read::GzDecoder;
use tar::Archive;
use colored::Colorize;

use serde::{Deserialize, Serialize};
use reqwest::blocking::get;
use serde_json::{Value};

use std::env;

use crate::util::commands::icd;

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
        println!("{}: {} {} {}", "registry.npmjs.org".bold().bright_cyan(), "Tarball".bold().bright_black(), "#".bold(), registry_data_latest_tar.clone().bold().bright_magenta());
        
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

        pub fn module_save(package_name: String, package_version: String, parsed_file_path: String, parsed_file_content: String) {
            #[derive(Debug, Serialize, Deserialize)]
            struct PackageJsonFileStruct {
                dependencies: Value
            }
            
            let package_json: serde_json::Value = serde_json::from_str(&parsed_file_content.clone()).expect("package.json file was not formatted correctly!");
            let mut dependencie: HashMap<String, Value> = HashMap::new();
            let mut package_as_hash = HashMap::new();
            let mut package_as_serde = serde_json::Map::new();
            let dependencies = package_json.get("dependencies");
            package_as_hash.insert(package_name.clone(), Value::String("^".to_string() + package_version.as_str()));
            package_as_serde.insert(package_name.clone(), Value::String("^".to_string() + package_version.as_str()));
            if !dependencies.is_none() {
                let dependencies_json = serde_json::to_string_pretty(&dependencies);
                let mut dependencies_hash_map = HashMap::new();
                #[allow(unused_must_use)] {
                    dependencies_json.map(|op| {
                        let dependencie_at_moment: serde_json::Value = serde_json::from_str(&op).expect("An error occurred while iteratoring of dependencies_json.result");
                        let unwraped_dependencie = dependencie_at_moment.as_object().unwrap();
                        unwraped_dependencie.keys().for_each(|key| {
                            if key.to_string() == package_name.clone() {
                                println!("{}: {} {}", "package.json".bold().bright_cyan(), package_name.clone().to_string().bold().bright_purple(), "is already declared.".bold())
                            }
                            let val = unwraped_dependencie.get(key).unwrap().to_string();
                            dependencies_hash_map.insert(key.to_string(), Value::String(val[1..val.len() - 1].to_string()));
                        });
                    });
                }

                fn hash_merge<K: Hash + Eq + Clone, V: Clone>(first_context: &HashMap<K, V>, second_context: &HashMap<K, V>) -> HashMap<K, V> {
                    let mut new_context = HashMap::new();
                    for (key, value) in first_context.iter() {
                        new_context.insert(key.clone(), value.clone());
                    }
                    for (key, value) in second_context.iter() {
                        new_context.insert(key.clone(), value.clone());
                    }
                    new_context
                }

                let merged_hasmap = hash_merge(&dependencies_hash_map, &package_as_hash);
                let mut serde_hash = serde_json::Map::new();
                merged_hasmap.keys().for_each(|key| {
                    let val = merged_hasmap.get(key).unwrap().to_string();
                    serde_hash.insert(key.to_string(), Value::String(val[1..val.len() - 1].to_string()));
                });
                dependencie.insert("dependencies".to_string(), Value::Object(serde_hash));
                let merged_data = merge(&package_json, &dependencie);

                fs::write(parsed_file_path, serde_json::to_string_pretty(&merged_data).unwrap().to_string()).unwrap();
            }
            else {
                dependencie.insert("dependencies".to_string(), Value::Object(package_as_serde));
                let merged_data = merge(&package_json, &dependencie);

                fs::write(parsed_file_path, serde_json::to_string_pretty(&merged_data).unwrap().to_string()).unwrap();
            }

            println!("{}: {} {}{}{} {}", "registry.npmjs.org".bold().bright_cyan(), "Added module".bold().bright_black(), package_name.clone().bold().bright_magenta(), "@".bold(), package_version.clone().to_string().bold().bright_magenta(), "to package.json".bold().bright_black());
        }

        #[allow(unused_variables)]
        pub fn module_save_dev(package_name: String, package_version: String, parsed_file_path: String, parsed_file_content: String) {
            #[derive(Debug, Serialize, Deserialize)]
            struct PackageJsonFileStruct {
                dependencies: Value
            }
            
            let package_json: serde_json::Value = serde_json::from_str(&parsed_file_content.clone()).expect("package.json file was not formatted correctly!");
            let mut dependencie: HashMap<String, Value> = HashMap::new();
            let mut package_as_hash = HashMap::new();
            let mut package_as_serde = serde_json::Map::new();
            let dependencies = package_json.get("devDependencies");
            package_as_hash.insert(package_name.clone(), Value::String("^".to_string() + package_version.as_str()));
            package_as_serde.insert(package_name.clone(), Value::String("^".to_string() + package_version.as_str()));
            if !dependencies.is_none() {
                let dependencies_json = serde_json::to_string_pretty(&dependencies);
                let mut dependencies_hash_map = HashMap::new();
                #[allow(unused_must_use)] {
                    dependencies_json.map(|op| {
                        let dependencie_at_moment: serde_json::Value = serde_json::from_str(&op).expect("An error occurred while iteratoring of dependencies_json.result");
                        let unwraped_dependencie = dependencie_at_moment.as_object().unwrap();
                        unwraped_dependencie.keys().for_each(|key| {
                            if key.to_string() == package_name.clone() {
                                println!("{}: {} {}", "package.json".bold().bright_cyan(), package_name.clone().to_string().bold().bright_purple(), "is already declared.".bold())
                            }
                            let val = unwraped_dependencie.get(key).unwrap().to_string();
                            dependencies_hash_map.insert(key.to_string(), Value::String(val[1..val.len() - 1].to_string()));
                        });
                    });
                }

                fn hash_merge<K: Hash + Eq + Clone, V: Clone>(first_context: &HashMap<K, V>, second_context: &HashMap<K, V>) -> HashMap<K, V> {
                    let mut new_context = HashMap::new();
                    for (key, value) in first_context.iter() {
                        new_context.insert(key.clone(), value.clone());
                    }
                    for (key, value) in second_context.iter() {
                        new_context.insert(key.clone(), value.clone());
                    }
                    new_context
                }

                let merged_hasmap = hash_merge(&dependencies_hash_map, &package_as_hash);
                let mut serde_hash = serde_json::Map::new();
                merged_hasmap.keys().for_each(|key| {
                    let val = merged_hasmap.get(key).unwrap().to_string();
                    serde_hash.insert(key.to_string(), Value::String(val[1..val.len() - 1].to_string()));
                });
                dependencie.insert("devDependencies".to_string(), Value::Object(serde_hash));
                let merged_data = merge(&package_json, &dependencie);

                fs::write(parsed_file_path, serde_json::to_string_pretty(&merged_data).unwrap().to_string()).unwrap();
            }
            else {
                dependencie.insert("devDependencies".to_string(), Value::Object(package_as_serde));
                let merged_data = merge(&package_json, &dependencie);

                fs::write(parsed_file_path, serde_json::to_string_pretty(&merged_data).unwrap().to_string()).unwrap();
            }

            println!("{}: {} {}{}{} {}", "registry.npmjs.org".bold().bright_cyan(), "Added module".bold().bright_black(), package_name.clone().bold().bright_magenta(), "@".bold(), package_version.clone().to_string().bold().bright_magenta(), "to package.json \"As a dev dependencie\"".bold().bright_black());
        }

        match package_paramater.as_str() {
            "--save"     => module_save    (package_name.clone(), registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap().clone()),
            "--save-dev" => module_save_dev(package_name.clone(), registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap().clone()),
            _            => module_save    (package_name.clone(), registry_data_latest_ver, parsed_file_path, parsed_file_content.unwrap().clone()),
        }

        // Download data from registry_data_latest_tar
        let base_dir_name    = String::from(Path::new(&node_modules_clone.clone()).to_string_lossy().split("\\").next().unwrap().to_string() + "/.dvn-cache/".to_string().as_str()).to_string();
        let base_dir_path = Path::new(&base_dir_name);
        if !Path::exists(base_dir_path) {
            #[allow(unused_must_use)] {
                fs::create_dir_all(base_dir_path);
            }
        }
        let tgz_package_path = String::from(format!("{}.{}.{}", "dpkg", package_name.clone().to_lowercase().to_string().chars()
        .map(|x| match x { 
            '/' => '-',
            _ => x
        }).collect::<String>(), "tgz").as_str());
        
        fn data_from_registry(registry_data_latest_tar: String, tgz_package_name: String, package_name: String, node_modules_clone: String, base_dir_name: String) {
            println!("[{}] | {}: {} {} {}", "/".bold().bright_green(), "registry.npmjs.org".bold().bright_cyan(), "Recieving data of".bold().bright_black(), "#".bold(), registry_data_latest_tar.clone().to_string().bold().bright_purple());

            let binding = String::from(base_dir_name.clone().to_string() + tgz_package_name.clone().as_str());
            let tgz_package_ostream_path = Path::new(&binding);
            
            // If the path of tgz_package doesn't exists, Just download it and do the work :)
            if !Path::exists(tgz_package_ostream_path) {
                #[allow(unused_must_use)] {
                    let tgz_package_in_stream = get(registry_data_latest_tar.clone()).expect("Request failed while contacting with registry.npmjs.org tarball.");
                    let mut tgz_package_out_stream= File::create(tgz_package_ostream_path).expect("Could not create the file");

                    tgz_package_out_stream.write_all(&tgz_package_in_stream.bytes().unwrap()).and_then(|_| Ok({
                        print!("\n");
                        println!("[{}] | {}: {} {} {}", "DOWNLOADED".bold().bright_green(), "registry.npmjs.org".bold().bright_cyan(), "Package".bold().bright_black(), "#".bold(), package_name.clone().to_string().bold().bright_blue());

                        let unpack_to_dist = String::from(node_modules_clone.clone());
                        let unpack_to_pkgd = String::from(node_modules_clone.clone() + package_name.as_str());
                        
                        let tar_gz = File::open(tgz_package_ostream_path)?;
                        let tar = GzDecoder::new(tar_gz);
                        let mut archive = Archive::new(tar);
                        archive.unpack(String::from(unpack_to_dist.clone()));
                        if Path::new(&String::from(unpack_to_pkgd.clone())).exists() {
                            fs::remove_dir_all(unpack_to_pkgd.clone());
                            println!("[{}]    | {} {}{}", "CLEANED".bold().bright_green(), "Old Package Directory".bold().bright_cyan(), "/node_modules/".bold().bright_black(), package_name.clone().to_string().bold().bright_green());
                        }
                        fs::rename(unpack_to_dist + "package", unpack_to_pkgd.clone());
                        let pfc = fs::read_to_string(String::from(node_modules_clone.clone().as_str().to_string() + package_name.clone().as_str() + "/package.json".to_string().as_str()));
                        let pkgjson_val: serde_json::Value = serde_json::from_str(&pfc.unwrap().clone()).expect("package.json file was not formatted correctly!");
                        icd::run(&serde_json::to_string_pretty(&pkgjson_val.clone()).unwrap(), "-od".to_string(), String::from(base_dir_name.clone()));
                    }));
                }
            } else {
                #[allow(unused_must_use)] {
                    let unpack_to_dist = String::from(node_modules_clone.clone());
                    let unpack_to_pkgd = String::from(node_modules_clone.clone() + package_name.as_str());

                    let tar_gz = File::open(tgz_package_ostream_path).unwrap();
                    let tar = GzDecoder::new(tar_gz);
                    let mut archive = Archive::new(tar);
                    archive.unpack(String::from(unpack_to_dist.clone()));
                    if Path::new(&String::from(unpack_to_pkgd.clone())).exists() {
                        fs::remove_dir_all(unpack_to_pkgd.clone());
                        println!("[{}]    | {} {}{}", "CLEANED".bold().bright_green(), "Old Package Directory".bold().bright_cyan(), "/node_modules/".bold().bright_black(), package_name.clone().to_string().bold().bright_green());
                    }
                    fs::rename(unpack_to_dist + "package", unpack_to_pkgd.clone());
                    let pfc = fs::read_to_string(String::from(node_modules_clone.clone().as_str().to_string() + package_name.clone().as_str() + "/package.json".to_string().as_str()));
                    let pkgjson_val: serde_json::Value = serde_json::from_str(&pfc.unwrap().clone()).expect("package.json file was not formatted correctly!");
                    icd::run(&serde_json::to_string_pretty(&pkgjson_val.clone()).unwrap(), "-od".to_string(), base_dir_name.clone());         
                }
            }
        }

        #[allow(unused_must_use)] {
            data_from_registry(registry_data_latest_tar, tgz_package_path, package_name.clone(), node_modules_clone.clone().to_string_lossy().to_string(), String::from(base_dir_name.clone()));
        }

        // Install dependencies of node_modules/folder_of_the_package
        // And cache it in the %temp%/.dvn/cached/folder_of_the_package
        // Compress node_modules/folder_of_the_package

        // When using "dvn run script --unpack-cached" or "dvn script -u-c" it 
        // will unpack the compressed folder then it will run node process in the background

    }
}