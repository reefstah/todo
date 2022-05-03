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
use usecases::ViewTodoUsecase;
use usecases::DeleteTodoUsecase;

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
    Delete {
        todo_id: Uuid,
    }
}

fn main() -> Result<(), std::io::Error> {
    let repository = FileSystemRepository {};
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
        Some(Commands::Delete {
            todo_id
        }) => {
            let usecase = DeleteTodoUsecase::new(&repository);
            usecase.execute(todo_id); 
        }
        None => {
            let usecase = ViewTodoUsecase::new(&repository);
            usecase.execute(); 
        }
    }

    Ok(())
}


