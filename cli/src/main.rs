use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use uuid::Uuid;

use clap::{Parser, Subcommand};

// Internal workspaces
use usecases;
mod fs_repository;
use crate::fs_repository::FileSystemRepository;
use usecases::TodoSavable;
use usecases::AddTodoUsecase;
use entities::Todo;

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
        todo_id: String,
        #[clap(short, long)]
        content: Option<String>,
    },
}

fn main() -> Result<(), std::io::Error> {

    let repository = FileSystemRepository{};
    let usecase = AddTodoUsecase::new(&repository);
    let usecase = AddTodoUsecase::new(&repository);

    usecase.execute(String::from("Woopdiedoo"));


    let cli = Cli::parse();

    let todo_dir = Path::new("todo/");

    match cli.command {
        Some(Commands::Add { content }) => {
            if !todo_dir.is_dir() {
                fs::create_dir("todo").unwrap();
            }

            let uuid = Uuid::new_v4().to_string();
            let name = todo_dir.to_str().unwrap().to_string() + &uuid;

            let mut file = OpenOptions::new()
                .read(false)
                .write(true)
                .create(true)
                .open(name)?;

            file.write(content.as_bytes())?;

            println!("Command add invoked {}", content);
        }
        Some(Commands::Edit {
            todo_id,
            content: None,
        }) => {
            let editor = env::var("EDITOR").unwrap();
            let path = String::from("todo/") + &todo_id;
            if Path::new(&path).exists() {
                Command::new(editor).arg(path).spawn().unwrap().wait()?;
            } else {
                panic!("File doesn't exist");
            }
        }
        Some(Commands::Edit {
            todo_id,
            content: Some(content),
        }) => {
            let path = String::from("todo/") + &todo_id;
            let path = Path::new(&path);
            let mut file = OpenOptions::new()
                .read(false)
                .write(true)
                .create(false)
                .open(path)?;
            println!("{}", &content);
            file.write(content.as_bytes())?;
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
