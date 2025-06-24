use clap::{Parser, Subcommand,Args};
#[derive(Parser, Debug)]
#[command(name = "todo" , about = "A simple todo list manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand, Debug)]

pub enum Commands {
    Add(AddArgs),
    List,
    Delete(DeleteArgs),
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub desc: String,

    #[arg(long)]
    pub due: String,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub status: bool,
}
#[derive(Args, Debug)]
pub struct DeleteArgs {
    #[arg(long)]
    pub id: usize,
}