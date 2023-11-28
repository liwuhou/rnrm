extern crate dirs;

use colored::*;
use regex::Regex;
use serde_json::{from_reader, Value};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub const NPMRC: &str = ".npmrc";
pub const NRMRC: &str = ".nrmrc";
pub const REGISTRY: &str = "registry";

pub fn get_npmrc_path() -> Option<String> {
    Some(format!("{}/{}", dirs::home_dir()?.display(), NPMRC))
}
pub fn get_nrmrc_path() -> Option<String> {
    Some(format!("{}/{}", dirs::home_dir()?.display(), NRMRC))
}
fn extract_registry(line: &str) -> Option<String> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    match line.split_once('=') {
        Some((key, value)) if key.trim().eq(REGISTRY) => Some(String::from(value.trim())),
        _ => None,
    }
}

pub fn get_current_registry() -> Option<String> {
    let file = get_npmrc_path()?;
    let reader = BufReader::new(File::open(file).ok()?);

    // TODO: extract
    reader
        .lines()
        .filter_map(Result::ok)
        .find_map(|line| extract_registry(&line))
}

fn extract_nrmrc_registry(lines: &mut Lines<BufReader<File>>) -> Option<(String, String)> {
    let regex = Regex::new(r"\[(.*?)\]").ok()?;

    let mut registry_name = None;
    let mut registry_addr = None;

    while let Some(line) = lines.next().transpose().ok()? {
        if let Some(captures) = regex.captures(&line) {
            if let Some(name) = captures.get(1) {
                registry_name = Some(name.as_str().to_owned());
            }
        } else if let Some(addr) = extract_registry(&line) {
            registry_addr = Some(addr.to_owned());
            break;
        }
    }

    registry_name.and_then(|name| registry_addr.map(|addr| (name, addr)))
}

pub fn get_nrm_registries() -> Option<HashMap<String, String>> {
    let file = get_nrmrc_path()?;
    let reader = BufReader::new(File::open(file).ok()?);

    let mut result: HashMap<String, String> = HashMap::new();
    let mut lines = reader.lines().into_iter();

    while let Some((registry_name, registry_addr)) = extract_nrmrc_registry(&mut lines) {
        result.insert(registry_name, registry_addr);
    }

    Some(result)
}

pub fn find_npmrc_config() {
    let argv: Vec<String> = env::args().collect();
    let excuting_dir = argv.get(0).unwrap_or(&String::from("~"));
}

pub fn get_default_registries() -> Result<Value, Box<dyn std::error::Error>> {
    let current_directory = env::current_dir()?;
    let file = File::open(current_directory.join("src/registries.json"))?;

    Ok(from_reader(BufReader::new(file))?)
}

pub fn get_registries() -> Option<HashMap<String, String>> {
    let registries = get_default_registries().ok()?;
    let custom_registries = get_nrm_registries();

    let mut registry_map: HashMap<String, String> = HashMap::new();

    if let Value::Object(map) = registries {
        for (registry_name, value) in map {
            if let Some(value) = value.get(REGISTRY) {
                if value.is_string() {
                    registry_map.insert(registry_name, value.to_string());
                }
            }
        }
    }
    if let Some(map) = custom_registries {
        for (registry_name, value) in map {
            registry_map.insert(registry_name, value);
        }
    }
    Some(registry_map)
}

pub fn get_pretty_format(input: &str, is_current_registry: bool) -> String {
    let formatted_str = format!(" {} ", input);
    if is_current_registry {
        print!("{}", "*".green());
    } else {
        print!(" ");
    }

    format!("{:-<width$}", formatted_str, width = 18)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_current_registry() {
        let current_registry_address = get_current_registry().unwrap();
        assert_eq!(current_registry_address, "https://registry.npmjs.org/")
    }
}
