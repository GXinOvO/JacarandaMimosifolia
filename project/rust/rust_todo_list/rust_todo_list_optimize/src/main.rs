#[warn(unused_variables)]

mod cli;
mod database;
mod commands;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use database::Database;

fn main()
{
    let args = Cli::parse();

    let mut db = Database::open();

    // -> 匹配命令调用
    let result = match args.command
    {
        Commands::Info => commands::info(),
        Commands::Add { content } => commands::add(&mut db, content),
        Commands::Remove { id } => commands::remove(&mut db, id),
        Commands::List => commands::list(&mut db),
    };

    // -> 统一处理错误
    if let Err(err) = result 
    {
        eprintln!("\x1b[31merror:\x1b[39m {}", err);
        std::process::exit(1);
    }
}