mod cli;
mod deploy;
mod new;
mod utils;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
