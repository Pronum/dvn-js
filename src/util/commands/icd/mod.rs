use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use colored::Colorize;
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tar::Archive;

pub fn run(file_content: &String, installation_type: String, tgz_base_name: String) {
    let env_path = env::current_dir().unwrap().to_owned();
    
    let dmut_dir_path = env_path.display().to_string();
    let _nmut_pkg_main: &str  = "/node_modules/";

    if !Path::new(&format!("{}{}", dmut_dir_path, "/package.json")).exists() {
        return println!("{} {}", "Project is not initialized!".bright_black().bold(), "Please use \"init\" command before installing a package.".bright_cyan().bold());
    }

    let node_modules_strp = format!("{}{}", dmut_dir_path.clone(), _nmut_pkg_main.clone()).to_string();
    let node_modules_path  = Path::new(node_modules_strp.as_str());
    let node_modules_clone = node_modules_path.clone();

    if !node_modules_clone.exists() {
        std::fs::create_dir_all(node_modules_clone).unwrap_or_default();
        println!("{}: {} {} {}", "dvn.base.dir".bold().bright_cyan(), "Created".bold().bright_black(), "#".bold(), "/node_modules/".bold().bright_yellow());
    }

    // Download data from registry_data_latest_tar
    
    fn data_from_registry(registry_data_latest_tar: String, tgz_package: String, package_name: String, package_version: String, node_modules_clone: String, tgz_main: String) {
        std::thread::spawn(move || {
            #[allow(unused_must_use)] {
                let tgz_package_in_stream = get(registry_data_latest_tar.clone()).expect("Request failed while contacting with registry.npmjs.org tarball.");
                let tgz_path = format!("{}{}", tgz_main, tgz_package);
                let tgz_package_ostream_path = Path::new(&tgz_path);

                let unpack_to_dist = node_modules_clone.clone().to_string();
                let unpack_to_pkgd = format!("{}{}", node_modules_clone.clone(), package_name.as_str()).to_string();
                
                if !Path::exists(tgz_package_ostream_path) {
                    let mut tgz_package_out_stream= File::create(Path::new(&tgz_package_ostream_path.to_string_lossy().to_string())).expect("Could not create the file");
                    tgz_package_out_stream.write_all(&tgz_package_in_stream.bytes().unwrap()).and(Ok(|| {
                    })).and_then(|_| Ok({

                        if Path::new(&String::from(unpack_to_pkgd.clone())).exists() {
                            fs::remove_dir_all(unpack_to_pkgd.clone());
                        }

                        let path = tgz_package_ostream_path;
        
                        let tar_gz = File::open(path)?;
                        let tar = GzDecoder::new(tar_gz);
                        let mut archive = Archive::new(tar);
                        archive.unpack(String::from(unpack_to_dist.clone())).and_then(|_| {
                            fs::rename(unpack_to_dist.clone() + "package", unpack_to_pkgd.clone());
                            println!("[{}] | {}: {} {} {}{}{}", "#".bold().bright_green(), "registry.npmjs.org".bold().bright_cyan(), "Received data of".bold().bright_black(), "#".bold(), package_name.clone().to_string().bold().bright_purple(), "@".bold(), package_version.clone().to_string().bold().bright_purple());
                            Ok({})
                        });
                    }));
                }
                else {
                    let path = tgz_package_ostream_path;
        
                    let tar_gz = File::open(path).unwrap();
                    let tar = GzDecoder::new(tar_gz);
                    let mut archive = Archive::new(tar);
                    archive.unpack(String::from(unpack_to_dist.clone())).and_then(|_| {
                        fs::rename(unpack_to_dist.clone() + "package", unpack_to_pkgd.clone());
                        println!("[{}] | {}: {} {} {}{}{}", "#".bold().bright_green(), "registry.npmjs.org".bold().bright_cyan(), "Received data of".bold().bright_black(), "#".bold(), package_name.clone().to_string().bold().bright_purple(), "@".bold(), package_version.clone().to_string().bold().bright_purple());
                        Ok({})
                    });
                }
            };
        });
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    struct NPMRegistryStruct {
        _id: String,
        _rev: String,
        versions: Value,
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    struct NPMRegistryVersions {
        version: String,
    }    

    // Only parse Dependencies modules
    pub fn module_parse_odep(parsed_file_content: String, node_modules_clone: String, tgz_pkg: String) {
        let package_json: serde_json::Value = serde_json::from_str(&parsed_file_content).expect("package.json file was not formatted correctly!");
        if package_json.get("dependencies").is_none() { return };
        let dependencies: serde_json::Value = package_json.get("dependencies").unwrap().clone();

        if !dependencies.as_object().is_none() {
            #[allow(unused_must_use)] {
                dependencies.as_object().into_iter().for_each(|iterator| {
                    // REWORK NEEDED TOMMOROW => I HATE CHAT GPT IT'S BROKEN AS HELL
                    // REWORK NEEDED TOMMOROW => I HATE CHAT GPT IT'S BROKEN AS HELL
                    // REWORK NEEDED TOMMOROW => I HATE CHAT GPT IT'S BROKEN AS HELL
                    // REWORK NEEDED TOMMOROW => I HATE CHAT GPT IT'S BROKEN AS HELL
                    // REWORK NEEDED TOMMOROW => I HATE CHAT GPT IT'S BROKEN AS HELL
                    // REWORK NEEDED TOMMOROW => I HATE CHAT GPT IT'S BROKEN AS HELL
                    // REWORK NEEDED TOMMOROW => I HATE CHAT GPT IT'S BROKEN AS HELL
                    iterator.keys().for_each(|ikey| {
                        let package_name = ikey.to_string();
                        let package_version_unr = iterator.get(ikey).unwrap().to_string().replace("^", "").replace("~", "");
                        let package_version     = package_version_unr[1..package_version_unr.len() - 1].to_string();
                        
                        if Path::exists(&Path::new(&String::from(node_modules_clone.clone() + package_name.as_str()))) {
                            return;
                        }

                        let uri = format!("{}/{}", "https://registry.npmjs.org", package_name).to_string();
                        let res = get(String::from(uri)).unwrap();

                        // Prints registry.npmjs.org status, Not needed => Cuz it's ugly in the console for multiple dependencies
                        // println!("{}: {}", "registry.npmjs.org".bold().bright_cyan(), res.status())
                        
                        if res.status().to_string() == "200 OK" {
                            let registry_data_f = format!("{}", res.text().unwrap().to_string());
                            let registry_data_s: NPMRegistryStruct          = serde_json::from_str(registry_data_f.as_str()).unwrap();
                            let registry_data_latest_dist   = registry_data_s.versions.get(package_version.clone()).unwrap().get("dist");

                            if !registry_data_latest_dist.is_none() {
                                let registry_data_latest_tarq: String       = registry_data_latest_dist.unwrap().get("tarball").unwrap().to_string();
                                let registry_data_latest_tar: String        = registry_data_latest_tarq[1..registry_data_latest_tarq.len() - 1].to_string();
                                let tgz_package_path = String::from(format!("{}.{}.{}", "dpkg", package_name.clone().to_lowercase().to_string().chars()
                                .map(|x| match x { 
                                    '/' => '-',
                                    _ => x
                                }).collect::<String>(), "tgz").as_str());
                                data_from_registry(registry_data_latest_tar, tgz_package_path, package_name, package_version, node_modules_clone.clone(), tgz_pkg.clone());
                            }
                        }
                    });
                });
            }
        }
    }

    // Only parse devDependencies modules
    pub fn module_parse_ovdp(parsed_file_content: String, node_modules_clone: String, tgz_pkg: String) {
        let package_json: serde_json::Value = serde_json::from_str(&parsed_file_content).expect("package.json file was not formatted correctly!");
        if package_json.get("devDependencies").is_none() { return };
        let dependencies: serde_json::Value = package_json.get("devDependencies").unwrap().clone();

        if !dependencies.as_object().is_none() {
            #[allow(unused_must_use)] {
                dependencies.as_object().into_iter().for_each(|iterator| {
                    iterator.keys().for_each(|ikey| {
                        let package_name = ikey.to_string();
                        let package_version_unr = iterator.get(ikey).unwrap().to_string().replace("^", "").replace("~", "");
                        let package_version     = package_version_unr[1..package_version_unr.len() - 1].to_string();
                        
                        if Path::exists(&Path::new(&String::from(node_modules_clone.clone() + package_name.as_str()))) {
                            return;
                        }

                        let uri = format!("{}/{}", "https://registry.npmjs.org", package_name).to_string();
                        let res = get(String::from(uri)).unwrap();

                        // Prints registry.npmjs.org status, Not needed => Cuz it's ugly in the console for multiple dependencies
                        // println!("{}: {}", "registry.npmjs.org".bold().bright_cyan(), res.status())
                        
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

                        if res.status().to_string() == "200 OK" {
                            let registry_data_f = format!("{}", res.text().unwrap().to_string());
                            let registry_data_s: NPMRegistryStruct          = serde_json::from_str(registry_data_f.as_str()).unwrap();
                            let registry_data_latest_dist   = registry_data_s.versions.get(package_version.clone()).unwrap().get("dist");

                            if !registry_data_latest_dist.is_none() {
                                let registry_data_latest_tarq: String       = registry_data_latest_dist.unwrap().get("tarball").unwrap().to_string();
                                let registry_data_latest_tar: String        = registry_data_latest_tarq[1..registry_data_latest_tarq.len() - 1].to_string();
                                let tgz_package_path = String::from(format!("{}.{}.{}", "dpkg", package_name.clone().to_lowercase().to_string().chars()
                                .map(|x| match x { 
                                    '/' => '-',
                                    _ => x
                                }).collect::<String>(), "tgz").as_str());
                                data_from_registry(registry_data_latest_tar, tgz_package_path, package_name, package_version, node_modules_clone.clone(), tgz_pkg.clone());
                            }
                        }
                    });
                });
            }
        }
    }

    // Parse both Dependencies and devDependencies modules
    pub fn module_parse_both(parsed_file_content: String, node_modules_clone: String, pkg_main: String) {
        #[allow(unused_must_use)] {
            module_parse_odep(parsed_file_content.clone(), node_modules_clone.clone(), pkg_main.clone());
            module_parse_ovdp(parsed_file_content.clone(), node_modules_clone.clone(), pkg_main.clone());
        }
    }   

    match installation_type {
        _ => module_parse_both(file_content.to_string(), node_modules_clone.to_string_lossy().to_string(), tgz_base_name.clone())
    } 
}