mod cli;
mod todo;
mod add;
mod list;
mod delete; 

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => add::handle(args),
        Commands::List => list::handle(),
        Commands::Delete(args) => delete::handle(args), // ✅
    }
}
