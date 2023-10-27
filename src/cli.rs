use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    pub stack_size: Option<u64>,
}
