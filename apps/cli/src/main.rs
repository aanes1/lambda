use std::process::exit;

mod cli;
mod deploy;
mod init;
mod new;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("error: {}", e);
        exit(1);
    }
}
