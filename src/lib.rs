use clap::Parser;
use colored::Colorize;

mod util;

pub struct Cli {}

impl Cli {
    pub fn run() -> () {
        let opts = Args::parse();

        match opts.subcmd {
            SubCommand::Use(args) => {
                SubCommand::r#use(&args.name).unwrap();
            }
            SubCommand::Ls => {
                SubCommand::ls();
            }
            SubCommand::Current => {
                SubCommand::current();
            }
            SubCommand::Add(args) => {
                SubCommand::add(&args.name, &args.url);
            }
            SubCommand::Set(args) => {
                SubCommand::set(&args.name, &args.url);
            }
            SubCommand::Del(args) => {
                SubCommand::del(&args.name);
            }
            SubCommand::Rename(args) => {
                SubCommand::rename(&args.old_name, &args.new_name);
            }
        }
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    /// List all the registries
    Ls,
    /// Show current registry
    Current,
    /// Change current registry
    Use(Use),
    /// Add a custom registry
    Add(Add),
    /// Add or modify a custom registry
    Set(Add),
    /// Delete a custom registry
    Del(Del),
    /// Change custom registry's name
    Rename(Rename),
}

impl SubCommand {
    pub fn ls() {
        let registries = util::get_registries().unwrap_or_default();
        let (_, current) = util::get_current_registry().unwrap_or_default();
        for (registry_name, registry_addr) in registries.iter() {
            let addr = registry_addr.replace('"', "");
            println!(
                "{} {}",
                &util::get_pretty_format(registry_name, current == addr),
                registry_addr.replace('"', "")
            )
        }
    }
    pub fn current() {
        if let Some((registry_name, current_registry)) = util::get_current_registry() {
            println!(
                "You are using {} registry: {}",
                registry_name.green(),
                current_registry.yellow()
            );
        } else {
            println!("Nothing");
        }
    }
    pub fn r#use(registry_name: &str) -> Result<(), std::io::Error> {
        let registries = util::get_registries().unwrap_or_default();
        if let Some(registry) = registries.get(registry_name) {
            util::use_registry(registry)?;
            util::print_heading(util::State::Success);
            println!("The registry has been changed to '{}'", registry_name);
        } else {
            util::print_heading(util::State::Error);
            println!(
                "{}",
                format!("The registry '{}' is not found.", registry_name).red()
            );
        }
        Ok(())
    }
    pub fn add(name: &str, url: &str) {
        match util::add_registry_config(name, url) {
            Err(_) => {
                util::print_heading(util::State::Error);
                println!("The registry name or url is already in the rnrm registries!");
            }
            Ok(_) => {
                util::print_heading(util::State::Success);
                println!(
                    // TODO: prompt
                    "Add registry {name} success, run {} command to use {name} registry.",
                    format!("rnrm use {name}").green()
                )
            }
        }
    }
    pub fn set(_name: &str, _url: &str) {
        util::print_heading(util::State::Info);
        println!("Coming soon...");
    }
    pub fn del(name: &str) {
        let is_del_current_registry =
            util::get_current_registry().map(|(current_name, _)| current_name == name);

        match util::delete_registry(name) {
            Ok(_) => {
                util::print_heading(util::State::Success);
                println!("Delete registry {} success.", name);
                if let Some(true) = is_del_current_registry {
                    // TODO: prompt
                    SubCommand::r#use("npm").unwrap();
                }
            }
            Err(err) => {
                if let Some(err) = err.downcast_ref::<util::CustomError>() {
                    util::handle_custom_error(err);
                } else {
                    // FIXME: Handler errors
                }
            }
        }
    }
    pub fn rename(old_name: &str, new_name: &str) {
        match util::rename_registry(old_name, new_name) {
            Ok(_) => {
                util::print_heading(util::State::Success);
                println!(
                    "The registry '{}' has been renamed to '{}'",
                    old_name, new_name
                );
            }
            Err(err) => {
                if let Some(err) = err.downcast_ref::<util::CustomError>() {
                    util::handle_custom_error(err);
                } else {
                    // FIXME: Handler errors
                }
            }
        }
    }
}

#[derive(Debug, Parser)]
pub struct Use {
    pub name: String,
}

#[derive(Debug, Parser)]
pub struct Add {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Parser)]
pub struct Del {
    pub name: String,
}

#[derive(Debug, Parser)]
pub struct Rename {
    pub old_name: String,
    pub new_name: String,
}
