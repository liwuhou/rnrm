use rnrm::Cli;

mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::run();

    // let res = util::get_registries();
    // println!("{:?}", res);
    Ok(())
}
