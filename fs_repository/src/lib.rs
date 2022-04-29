use entities::Todo;
use std::env;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
