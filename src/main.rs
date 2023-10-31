mod cli;

use anyhow::Result;
use barco_rs::Container;
use clap::Parser;
use cli::Cli;
use nix::sys::resource::{getrlimit, rlim_t, Resource};

fn get_stack_rlimit() -> Result<rlim_t> {
    match getrlimit(Resource::RLIMIT_STACK) {
        Ok((soft, _hard)) => Ok(soft),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    let stack_size = match cli.stack_size {
        Some(s) => s,
        None => get_stack_rlimit()?,
    };
    let mut container = Container::new(stack_size, cli.mnt);
    container.start()?;
    match container.wait()? {
        0 => Ok(()),
        status => std::process::exit(status as i32),
    }
}
