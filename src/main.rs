use serde::{Serialize, Deserialize};
use std::{env, process};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    website: String,
    password: String,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct FilePathConf {
    file_path: String,
}

fn add(e: Data, file_path: &str) {
    let mut entry: Vec<Data> = if Path::new(file_path).exists() {
        let data = fs::read_to_string(file_path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    };

    entry.push(e);

    let json = serde_json::to_string_pretty(&entry).expect("Failed to serialize");
    fs::write(file_path, json).unwrap();
}

fn list(file_path: &str) {
    let content = fs::read_to_string(file_path).expect("Error reading.");

    let entries: Vec<Data> = serde_json::from_str(&content).expect("Error deserializing data");

    for entry in entries {
        println!("{} | Username: {}, Password: {}", entry.website, entry.username, entry.password);
    }
}

fn manual() {
    println!("Usage: pwdmgr [OPTIONS] <website> <password> <username>");
    println!("Options:");
    println!("  -a, --add           Adds a entry to the specified json file.");
    println!("  -l, --list          Lists all current entries in the json file.");
    println!("  -f, --file_path     Change file_path (where data is stored).");
    println!("  -h, --help          Displays manual.");
    println!("  -v, --version       Shows the current lc version.");
}

fn get_config_loc() -> PathBuf {
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).expect("Couldnt find home directory");
    let mut path = PathBuf::from(home);
    path.push("file_path.json");
    path
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf_path = get_config_loc();

    if !conf_path.exists() {
        let default = FilePathConf {
            file_path: "data.json".to_string(),
        };
        let json = serde_json::to_string_pretty(&default).unwrap();
        std::fs::write(&conf_path, json).expect("Failed to create config file");
    }

    let mut f_p: String = {
        let f_p_str = std::fs::read_to_string(&conf_path).unwrap_or_else(|_| panic!("Failed to read config file at {:?}", conf_path));
        let config: FilePathConf = serde_json::from_str(&f_p_str).expect("Error parsing file_path.json");
        config.file_path
    };


    let mut data = vec![];

    for arg in &args {
        if arg.starts_with("-") {
            match arg.as_str() {
                "-a" | "--add" => {
                    add(Data { website: args[2].clone(), password: args[3].clone(), username: args[4].clone() }, &f_p);
                }
                "-l" | "--list" => {
                    list(&f_p);
                }
                "-f" | "--file_path" => {
                    f_p = args[2].clone();
                    let json = serde_json::to_string_pretty(&FilePathConf { file_path: f_p.clone() }).expect("Serialization failed");
                    std::fs::write(&conf_path, json).expect("Error writing to file");
                }
                "-h" | "--help" => {
                    manual();
                }
                "-v" | "--version" => {
                    println!("pwdmgr v0.1.0");
                }
                _ => {
                    println!("Invalid flag!");
                    process::exit(1);
                }
            }
        } else {
            data.push(arg);
        }
    }
}
