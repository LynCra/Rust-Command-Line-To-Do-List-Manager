// src/main.rs
mod todo;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use todo::Todo;

#[derive(Parser)]
#[command(name = "todo_cli")]
#[command(about = "A simple to-do list manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    View {},
    Delete { index: usize },
}

fn main() {
    let cli = Cli::parse();
    let file_path = PathBuf::from("todo.json");

    let mut todo = Todo::load(&file_path).expect("Failed to load to-do list");

    match cli.command {
        Commands::Add { task } => {
            todo.add(task);
            todo.save(&file_path).expect("Failed to save to-do list");
            println!("Task added.");
        }
        Commands::View {} => {
            todo.view();
        }
        Commands::Delete { index } => {
            match todo.delete(index) {
                Ok(_) => {
                    todo.save(&file_path).expect("Failed to save to-do list");
                    println!("Task deleted.");
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}
