use clap::Parser;

/// List Steam games in a directory
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to list if not the current directory
    #[clap(short, long)]
    pub dir: Option<String>,

    /// Forcefully invalidate the appid cache
    #[clap(long)]
    pub invalidate_cache: bool,
}
