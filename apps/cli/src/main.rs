mod cli;
mod deploy;
mod new;
mod utils;

fn main() {
    if let Err(e) = cli::run() {
        utils::error(e);
        std::process::exit(1);
    }
}
