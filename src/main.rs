use std::process;

use clap::ArgMatches;
use uuid::Uuid;

use clap::{App, Arg};

mod app;


fn main() {
    let matches = App::new("Do")
        .version("1.0")
        .author("Reef <public@ralph.tech>")
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
        .get_matches();

    let repository = app::file_repository::SingleFileRepository {};

    if matches.is_present("init") {
        if let Err(_) = app::file_repository::init() {
            println!("Failed to initialize");
            process::exit(1);
        }
    }
    match matches.value_of("TASK") {
        Some(_) => {
            let todo = matches.into();
            if let Err(err) = app::usecases::new_todo(repository, todo) {
                println!("error adding todo: {}", err);
                process::exit(1);
            }
        }
        None => {
            let view = app::view::View {};
            app::usecases::show_relevant_usecase(repository, view);
        }
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
