mod cli;
mod config;
mod jetbrains;
mod mobaxterm;
mod utils;

use config::Config;

fn exit_with_error(e: impl std::fmt::Display) -> ! {
    eprintln!("{e}");
    std::process::exit(1);
}

#[tokio::main]
async fn main() {
    utils::init().unwrap_or_else(|e| exit_with_error(e));
    let config = Config::new().unwrap_or_else(|e| exit_with_error(e));
    cli::run(&config)
        .await
        .unwrap_or_else(|e| exit_with_error(e));
}
