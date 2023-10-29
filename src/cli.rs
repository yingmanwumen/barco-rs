use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub stack_size: Option<u64>,

    #[clap(long)]
    pub version: Option<u64>,

    #[clap(short, long, default_value = "1")]
    pub uid: u64,

    #[clap(short, long, default_value = "/")]
    pub mnt: String,

    #[clap(short, long)]
    pub cmd: Option<String>,

    #[clap(short, long)]
    pub arg: Option<String>,

    #[clap(short, long, default_value = "false")]
    pub verbose: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args() {
        let opts = Cli::parse_from(&["test_program"]);
        assert_eq!(opts.verbose, false);
    }

    #[test]
    fn test_verbose_arg() {
        let opts = Cli::parse_from(&["test_program", "-v"]);
        assert_eq!(opts.verbose, true);
    }

    #[test]
    fn test_uid_arg() {
        let opts = Cli::parse_from(&["test_program", "-u=2"]);
        assert_eq!(opts.uid, 2);
    }
}