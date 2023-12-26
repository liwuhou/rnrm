#![allow(dead_code)]
extern crate dirs;

use colored::*;
use regex::Regex;
pub use registries::get_internal_registries;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error as IoError, Lines, Write};
use std::sync::OnceLock;

mod registries;

// CONFIG
const NPMRC_NAME: &str = ".npmrc";
const NRMRC_NAME: &str = ".nrmrc";
const REGISTRY: &str = "registry";
static NPMRC_PATH: OnceLock<String> = OnceLock::new();
static NRMRC_PATH: OnceLock<String> = OnceLock::new();

pub enum State {
    Success,
    Error,
    Info,
}

pub fn get_npmrc_path() -> &'static str {
    NPMRC_PATH.get_or_init(|| format!("{}/{}", dirs::home_dir().unwrap().display(), NPMRC_NAME))
}
pub fn get_nrmrc_path() -> &'static str {
    NRMRC_PATH.get_or_init(|| format!("{}/{}", dirs::home_dir().unwrap().display(), NRMRC_NAME))
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

pub fn get_current_registry() -> Option<(String, String)> {
    let file = get_npmrc_path();
    let reader = BufReader::new(File::open(file).ok()?);

    reader
        .lines()
        .filter_map(Result::ok)
        .find_map(|line| extract_registry(&line))
        .map(|registry_url| {
            if let Some(registry_name) = find_registry_name(&registry_url) {
                (registry_name, registry_url)
            } else {
                ("UNKNOWN".to_owned(), registry_url)
            }
        })
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
    let file = get_nrmrc_path();
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
    let _excuting_dir = argv.get(0).unwrap_or(&String::from("~"));
    todo!()
}

pub fn get_registries() -> Option<BTreeMap<String, String>> {
    let default_registries = get_internal_registries().clone();
    let custom_registries = get_nrm_registries().unwrap_or_default();

    let mut registry_map: BTreeMap<String, String> = BTreeMap::new();
    registry_map.extend(default_registries.into_iter());
    registry_map.extend(custom_registries.into_iter());

    Some(registry_map)
}

pub fn add_registry_config(name: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let mut registries = get_registries().unwrap_or_default();

    let is_invalid = registries
        .iter()
        .any(|(_name, _url)| name == _name || _url == url);

    if is_invalid {
        return Err("Duplicated registry or url.".into());
    }

    registries.insert(name.to_string(), url.to_string());

    // write
    let file = get_nrmrc_path();
    let output = OpenOptions::new().append(true).open(file);
    if let Ok(mut output) = output {
        let insert_text = format!("\n[{}]\nregistry={}", name, url);
        output.write_all(insert_text.as_bytes())?;
    }

    Ok(())
}

pub fn delete_registry(name: &str) -> Result<(), Box<dyn Error>> {
    let mut registries = get_nrm_registries().unwrap_or_default();

    if let None = registries.remove(name) {
        return Err("Not found the registry.".into());
    } else {
        // rewrite
        let file = get_nrmrc_path();
        let mut output = File::create(file)?;
        for (registry_name, registry_url) in registries.iter() {
            writeln!(output, "[{}]", registry_name)?;
            writeln!(output, "registry={}\n", registry_url)?;
        }
    }

    Ok(())
}

pub fn check_registry_valid(name: &str, url: &str) -> bool {
    let registries = get_registries().unwrap_or_default();

    !registries
        .iter()
        .any(|(_name, _url)| name == _name || _url == url)
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

fn unpack_quote(input: &str) -> String {
    let input = input.trim();

    if input.starts_with('"') && input.ends_with('"') {
        input[1..input.len() - 1].to_owned()
    } else {
        input.to_owned()
    }
}

pub fn find_registry_name(registry: &str) -> Option<String> {
    let registries = get_registries().unwrap_or_default();

    if let Some((key, _)) = registries
        .iter()
        .find(|(_, value)| unpack_quote(*value) == unpack_quote(registry))
    {
        Some(key.clone())
    } else {
        None
    }
}

pub fn use_registry(registry: &str) -> Result<(), IoError> {
    let file = get_npmrc_path();
    let reader = BufReader::new(File::open(file)?);
    let mut file_vec = vec![];

    // read
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("registry=") {
                file_vec.push(format!("registry={}", registry));
            } else {
                file_vec.push(line);
            }
        }
    }

    // write
    let mut output = File::create(file)?;
    for line in file_vec.iter() {
        writeln!(output, "{}", line)?;
    }

    Ok(())
}

pub fn print_heading(state: State) {
    match state {
        State::Success => {
            print!("{} ", String::from(" SUCCESS ").black().on_green());
        }
        State::Error => {
            print!("{} ", String::from(" ERROR ").white().on_red());
        }
        State::Info => {
            print!("{} ", String::from(" INFO ").white().on_blue());
        }
    }
}
