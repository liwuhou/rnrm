extern crate dirs;

use ini::Ini;
const NPMRC: &str = ".npmrc";
const NRMRC: &str = ".nrmrc";
const REGISTRY: &str = "registry";

pub fn get_npmrc_path() -> Option<String> {
    Some(format!("{}/{}", dirs::home_dir()?.display(), NPMRC))
}
pub fn get_nrmrc_path() -> Option<String> {
    Some(format!("{}/{}", dirs::home_dir()?.display(), NRMRC))
}
pub fn get_current_registry() -> Result<String, Box<dyn std::error::Error>> {
    let ini = Ini::load_from_file(get_npmrc_path().unwrap())?;
    if let Some(global) = ini.section::<String>(None) {
        println!("{:?}", global);
        Ok(global
            .get(REGISTRY)
            .map_or(String::from(""), |v| String::from(v)))
    } else {
        Err("No global section in .npmrc".into())
    }
}
pub fn get_registries() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let custom_registries = Ini::load_from_file(get_nrmrc_path().unwrap());

    println!("{:?}", custom_registries);
    match custom_registries {
        Ok(ini) => {}
        _ => (),
    }

    Ok(Vec::new())
}
