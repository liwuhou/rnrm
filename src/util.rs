extern crate dirs;

use serde_json::{from_reader, Value};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
const NPMRC: &str = ".npmrc";
const NRMRC: &str = ".nrmrc";
const REGISTRY: &str = "registry";

pub fn get_npmrc_path() -> Option<String> {
    Some(format!("{}/{}", dirs::home_dir()?.display(), NPMRC))
}
pub fn get_nrmrc_path() -> Option<String> {
    Some(format!("{}/{}", dirs::home_dir()?.display(), NRMRC))
}
pub fn getConfigForRcfile(rc_content: &str) -> Option<String> {
    let res: Option<String> = None;
    for line in rc_content.lines() {
        println!("{}", line);
    }
    res
}
pub fn get_current_registry() -> Option<String> {
    let mut result: Option<String> = None;
    let file = File::open(get_npmrc_path()?).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let mut patter = line.split('=');
        if patter.next()? == "registry" {
            result = Some(String::from(patter.next()?));
            break;
        }
    }
    result
}

pub fn get_nrm_registries() -> Option<String> {
    let file = File::open(get_nrmrc_path()?).ok()?;

    Some(String::new())
}

pub fn find_npmrc_config() {
    let argv: Vec<String> = env::args().collect();
    let excuting_dir = argv.get(0).unwrap_or(&String::from("~"));
}

pub fn get_default_registries() -> Result<Value, Box<dyn std::error::Error>> {
    let current_directory = env::current_dir()?;
    let current_exe = env::current_exe()?;
    let file = File::open(current_directory.join("src/registries.json"))?;

    Ok(from_reader(BufReader::new(file))?)
}

pub fn get_registries() -> Option<HashMap<String, String>> {
    let registries = get_default_registries().ok()?;
    let mut registry_map: HashMap<String, String> = HashMap::new();

    if let Value::Object(map) = registries {
        for (registry_name, value) in map {
            if let Some(value) = value.get("registry") {
                if value.is_string() {
                    registry_map.insert(registry_name, value.to_string());
                }
            }
        }
    }
    Some(registry_map)
}
