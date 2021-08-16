use std::process;

use clap::{crate_authors, crate_name, crate_version, ArgMatches};
use uuid::Uuid;

use clap::{App, Arg};

use crate::app::domain::repository::Repository;

mod app;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Writes your tasks to a csv")
        .arg(
            Arg::new("TASK")
                .about("Summary of the task you wish to remember")
                .index(1),
        )
        .arg(
            Arg::new("DEADLINE")
                .short('d')
                .long("deadline")
                .about("Period when you aim to finish or set a calender date for this task"),
        )
        .arg(
            Arg::new("PRIORITY")
                .short('p')
                .long("priority")
                .about("Importance of this task expressed in a positive number till 255")
                .default_value("3"),
        )
        .arg(
            Arg::new("TAG")
                .short('t')
                .long("tags")
                .multiple(true)
                .about("Keywords you can use for search or organizing"),
        )
        .subcommand(App::new("init").about("Creates a new empty storage for you todo's"))
        .subcommand(
            App::new("rm").about("Removes a entry from todo").arg(
                Arg::new("ID")
                    .about("Entry identifier")
                    .index(1)
                    .required(true),
            ),
        )
        .get_matches();

    //let repository = app::file_repository::SingleFileRepository {};
    let repository = app::jsonl_repository::JsonlRepository {};

    match matches.subcommand() {
        Some(("init", _)) => {
            if let Err(_) = app::file_repository::init() {
                println!("Failed to initialize");
            }
        }
        Some(("rm", sub_m)) => {
            let id = sub_m.value_of("ID").unwrap();

            if let Some(repository) = repository.deletable() {
                repository.delete(id);
            } else {
                println!("Deletion not supported for your current TODO persistance");
            }
        }
        _ => match matches.value_of("TASK") {
            Some(_) => {
                let todo = matches.into();
                if let Err(err) = app::usecases::new_todo(repository, todo) {
                    println!("error adding todo: {}", err);
                    process::exit(1);
                }
            }
            None => {
                let view = app::view::View {};
                let usecase = app::usecases::ShowRelevantUseCase::new(&repository);
                usecase.execute(view);
            }
        },
    }
}

impl From<ArgMatches> for app::domain::entities::Todo {
    fn from(matches: ArgMatches) -> Self {
        let task = matches.value_of("TASK").unwrap().into();
        let priority = matches.value_of("PRIORITY").unwrap().parse().unwrap();
        let id = Uuid::new_v4();

        app::domain::entities::Todo {
            id,
            task,
            calender_date: None,
            priority,
        }
    }
}
