use clap::Parser;

mod util;

pub struct Cli {}

impl Cli {
    pub fn run() -> () {
        let opts = Args::parse();

        match opts.subcmd {
            SubCommand::Use(args) => {
                SubCommand::r#use(&args.name);
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
        println!("list");
    }
    pub fn current() {
        println!("current");
    }
    pub fn r#use(name: &str) {
        println!("use {}", name);
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
