use std::{path::{PathBuf}, process::exit};
use clap::{Parser, Subcommand};
use gix;

use crate::is_sudo::check;
mod is_sudo;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli
{

    #[command(subcommand)]
    command: Option<Commands>  
}

#[derive(Subcommand)]
enum Commands
{
    /// Register a repository for automatic commanding
    Register {
        /// The parent directory of your .git
        #[arg(short, long)]
        path: PathBuf,
    },

    /// unregister a repository for automatic commanding
    Unregister {
        /// The parent directory of your .git
        #[arg(short, long)]
        path: PathBuf,
    },

    /// List all registered repositories
    List,

}




fn main()
{

    let args= Cli::parse();


}


fn expand_tilde(path: &str) -> String
{

    if let Some(home) = std::env::home_dir()
    {
        let home_str = match home.to_str()
        {
            Some(str) => str,
            None => exit(2)
        };

        return path.replace("~", home_str);

    }

    
    return path.to_string();
}
