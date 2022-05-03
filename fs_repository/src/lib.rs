use entities::Todo;
use std::env;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use std::io;
use std::io::prelude::*;

pub fn add(todo: Todo) -> Result<(), std::io::Error> {
    let todo_dir = Path::new("todo/");
    if !todo_dir.is_dir() {
        fs::create_dir("todo").unwrap();
    }

    let name = format!(
        "{path}/{id}",
        path = todo_dir.to_str().unwrap().to_string(),
        id = todo.id().to_string()
    );

    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(name)?;

    file.write(todo.content().as_bytes())?;

    println!("Command add invoked {}", todo.content());
    Ok(())
}

pub fn edit(todo: Todo) -> Result<(), std::io::Error> {
    let todo_dir = Path::new("todo/");
    let todo_path = format!(
        "{path}/{id}",
        path = todo_dir.to_str().unwrap().to_string(),
        id = todo.id().to_string()
    ); 
    let path = Path::new(&todo_path);
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .create(false)
        .open(path)?;
    println!("{}", todo.content());
    file.write(todo.content().as_bytes())?;
    Ok(())
}

pub fn edit_iteractive(todo_id: Uuid) -> Result<(), std::io::Error> {
    let editor = env::var("EDITOR").unwrap();
    let path = String::from("todo/") + &todo_id.to_string();
    if Path::new(&path).exists() {
        Command::new(editor).arg(path).spawn().unwrap().wait()?;
    } else {
        panic!("File doesn't exist");
    }
    Ok(())
}

pub fn view() -> Result<(), std::io::Error> {
    let todo_dir = Path::new("todo/"); 
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
    Ok(())
}

pub fn delete(todo_id: Uuid) -> Result<(), std::io::Error> {
    let path = String::from("todo/") + &todo_id.to_string(); 
    fs::remove_file(path)?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
