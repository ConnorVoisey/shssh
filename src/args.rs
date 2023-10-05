use clap::Parser;
#[derive(Parser, Debug)]
pub enum Command {
    Ssh(SshArgs),
    Ls(LsArgs)
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Command,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SshArgs {
    /// servers name
    pub name: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct LsArgs {
    /// Name of the server
    #[arg(short, long)]
    pub name: Option<String>,

    /// Display all server fields
    #[arg(short, long)]
    pub all: bool,
}
