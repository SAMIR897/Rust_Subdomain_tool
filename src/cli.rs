use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "rst",
    about = "Rust Subdomain Tool — fast, async subdomain enumeration",
    version,
    author = "@SAMIR897"
)]
pub struct Cli {
    /// Target domain to enumerate (e.g. example.com)
    #[arg(value_name = "DOMAIN")]
    pub domain: String,

    /// Output results to a file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Resolve subdomains to IP addresses
    #[arg(short, long)]
    pub resolve: bool,

    /// Output results as JSON
    #[arg(long)]
    pub json: bool,

    /// Show verbose/debug output
    #[arg(short, long)]
    pub verbose: bool,
}
