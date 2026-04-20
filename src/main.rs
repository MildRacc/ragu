use std::{path::{PathBuf}, process::exit};
use clap::{Parser, Subcommand};
use gix;

use crate::is_sudo::check;
mod is_sudo;
mod daemon;

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

    /// Start daemon
    Up,
    
    /// Stop daemon
    Down,

    /// List all registered repositories
    List,

}



enum Features 
{
    AutoCommit = 0x01,
    AutoPush = 0x02,
    AutoPull = 0x04
}

struct Registry
{

        name: String,
        path: PathBuf,
        excluded_features: u8,

}



fn main()
{

    let args = Cli::parse();

    let ragud = daemon::Ragud::new(); 

    if let Some(cmd) = args.command
    {
        

        match cmd {
            Commands::List => {},
            Commands::Register{path} => {register(&path);},
            _ => {exit(1)}
        }
    

    }
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


fn register(dir: &PathBuf)
{

    let repo = match gix::open(dir)
    {
        Ok(r) => r,
        Err(_) => exit(1),
    };

    
}



fn unregister()
{


}
