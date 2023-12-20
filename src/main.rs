use rnrm::Cli;

mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::run();
    // util::get_current_registry();
    Ok(())
}
