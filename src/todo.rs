// src/todo.rs
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub tasks: Vec<String>,
}

impl Todo {
    pub fn new() -> Self {
        Todo { tasks: Vec::new() }
    }

    pub fn add(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn view(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}: {}", index, task);
        }
    }

    pub fn delete(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err("Invalid index".to_string())
        }
    }

    pub fn load(file_path: &PathBuf) -> io::Result<Todo> {
        if !file_path.exists() {
            return Ok(Todo::new());
        }
        let file = OpenOptions::new().read(true).open(file_path)?;
        let reader = BufReader::new(file);
        let todo = serde_json::from_reader(reader)?;
        Ok(todo)
    }

    pub fn save(&self, file_path: &PathBuf) -> io::Result<()> {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}
