use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub name: String,
    pub description: String,
    pub due_date: String,
    pub status: bool,
}

const FILE: &str = "todos.json";

pub fn load_todos() -> Vec<Todo> {
    let file = OpenOptions::new()
        .read(true)
        .open(FILE);

    let mut data = String::new();

    match file {
        Ok(mut f) => {
            f.read_to_string(&mut data).unwrap();
            if data.is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
            }
        }
        Err(_) => Vec::new(),
    }
}

pub fn save_todos(todos: &[Todo]) {
    let json = serde_json::to_string_pretty(todos).expect("Serialization failed");

    let mut file = File::create(FILE).expect("Could not create file");
    file.write_all(json.as_bytes()).expect("Write failed");
}
