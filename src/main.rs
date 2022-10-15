use clap::Parser;
use log::error;
use setenv::cmd::env::apply::{EnvApplyArgs, EnvApplyCommandHandler};
use setenv::cmd::CommandHandler;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Set your environment variables to macOS, Linux, [fly.io](https://fly.io) and more.", long_about = None)]
struct Opts {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Apply(EnvApplyArgs),
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stdout)
        .init();
    let opts = Opts::parse();
    let future = match opts.command {
        SubCommand::Apply(ref args) => EnvApplyCommandHandler {}.handle(args),
    };
    match future.await {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            std::process::exit(1)
        }
    }
}
