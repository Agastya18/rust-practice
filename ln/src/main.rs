use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    task: String,
    done: bool,
}

const FILE_NAME: &str = "todos.json";

fn main() {
    println!("Simple Todo CLI");
    loop {
        println!("\n1. Add Task\n2. View Tasks\n3. Mark Task as Done\n4. Exit");
        let choice = get_user_input("Enter your choice:");
        match choice.trim() {
            "1" => add_task(),
            "2" => view_tasks(),
            "3" => mark_done(),
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn add_task() {
    let task = get_user_input("Enter the task:");
    let mut todos = load_tasks();
    todos.push(Todo { task, done: false });
    save_tasks(&todos);
    println!("Task added!");
}

fn view_tasks() {
    let todos = load_tasks();
    if todos.is_empty() {
        println!("No tasks found!");
    } else {
        for (i, todo) in todos.iter().enumerate() {
            let status = if todo.done { "[Done]" } else { "[Pending]" };
            println!("{}: {} {}", i + 1, status, todo.task);
        }
    }
}

fn mark_done() {
    let mut todos = load_tasks();
    if todos.is_empty() {
        println!("No tasks found!");
        return;
    }

    view_tasks();
    let choice = get_user_input("Enter the task number to mark as done:");
    if let Ok(index) = choice.trim().parse::<usize>() {
        if index > 0 && index <= todos.len() {
            todos[index - 1].done = true;
            save_tasks(&todos);
            println!("Task marked as done!");
        } else {
            println!("Invalid task number.");
        }
    } else {
        println!("Please enter a valid number.");
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn load_tasks() -> Vec<Todo> {
    if let Ok(data) = fs::read_to_string(FILE_NAME) {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_tasks(todos: &[Todo]) {
    let data = serde_json::to_string_pretty(todos).unwrap();
    fs::write(FILE_NAME, data).unwrap();
}
