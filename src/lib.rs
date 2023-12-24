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
    /// Delete a custom registry
    Del(Del),
    /// Change custom registry's name
    Rename(Rename),
}

impl SubCommand {
    pub fn ls() {
        let registries = util::get_registries().unwrap_or_default();
        let current = util::get_current_registry().unwrap_or_default();
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
        let current_registry = util::get_current_registry().unwrap();

        if let Some(registry_name) = util::find_registry_name(&current_registry) {
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
            println!("Found the registry: {}: {}", registry_name, registry);
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
        println!("add {} {}", name, url);
    }
    pub fn del(name: &str) {
        println!("del {}", name);
    }
    pub fn rename(old_name: &str, new_name: &str) {
        println!("rename {} {}", old_name, new_name);
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
