use std::fs;
use std::fs::File;

use std::io;
use std::io::prelude::*;
use std::path::Path;

use uuid::Uuid;

use clap::{Parser, Subcommand};

// Internal workspaces
use usecases;
mod add_todo_cli_presenter;
mod fs_repository;
use crate::add_todo_cli_presenter::AddTodoCliPresenter;
use crate::fs_repository::FileSystemRepository;
use usecases::AddTodoUsecase;
use usecases::EditTodoInteractiveUsecase;
use usecases::EditTodoUsecase;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        content: String,
    },
    Edit {
        todo_id: Uuid,
        #[clap(short, long)]
        content: Option<String>,
    },
}

fn main() -> Result<(), std::io::Error> {
    let repository = FileSystemRepository {};
    let todo_dir = Path::new("todo/");
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { content }) => {
            let todo_id = Uuid::new_v4();
            let usecase = AddTodoUsecase::new(&repository);
            let presenter = AddTodoCliPresenter::new(&todo_id);
            usecase.execute(String::from(content), todo_id, &presenter);
        }
        Some(Commands::Edit {
            todo_id,
            content: None,
        }) => {
            let usecase = EditTodoInteractiveUsecase::new(&repository);
            usecase.execute(todo_id);
        }
        Some(Commands::Edit {
            todo_id,
            content: Some(content),
        }) => {
            let usecase = EditTodoUsecase::new(&repository);
            usecase.execute(content, todo_id);
        }
        None => {
            for entry in fs::read_dir(todo_dir)? {
                let entry = entry?;
                if let Ok(lines) = read_lines(entry.path()) {
                    for line in lines {
                        if let Ok(content) = line {
                            println!("{}, {:?}", content, entry.file_name());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
