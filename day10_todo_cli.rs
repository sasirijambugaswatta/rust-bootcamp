use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]//attr macro
struct Task{
    id:usize,
    description: String,
    completed:bool
}

fn main() {
    let mut tasks:Vec<Task> = load_tasks();

    loop{
        println!("ToDo List menu");
        println!("1. Add Tasks");
        println!("2. View Tasks");
        println!("3. Mark Tasks as Complete");
        println!("4. Delete tasks");
        println!("5. Exit");

        let choice = get_input("Enter Your CHoice");

        match choice.trim(){
            "1" => add_tasks(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_complete(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("Tasks saved. Goodbye");
                break;
            }
            _ => println!("Invalid choice")
        }
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to delete: ");
    if let Ok(id) = id.trim().parse::<usize>(){
        if let Some(index) = tasks.iter().position(|t| t.id == id){
            tasks.remove(index);
            println!("Task deleted!")
        }else {
            println!("Task not found!")
        }
    }else {
        println!("Invalid task ID!")
    }

}

fn mark_task_complete(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to mark as completed: ");
    if let Ok(id) = id.trim().parse::<usize>(){
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id){
            task.completed = true;
            println!("task marked as completed ");

        } else {
            println!("Taks not found");
        }
    }else{
        println!("Invalid task id");
    }
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found");
    }else {
        for task in tasks {
            let status = if task.completed {"✅"} else {"❌"};
            println!("{} - {} : {}",task.id, status, task.description);
        }
    }
}

fn add_tasks(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description: ");
    let id = tasks.len() + 1;
    tasks.push(Task { id: id, description: description.trim().to_string(), completed: false });
    println!("Task added!")
}

fn save_tasks(tasks: &Vec<Task>)  {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize task");
    let mut file = File::create("task.json").expect("Failed to save tasks");
    file.write_all(json.as_bytes()).expect("Failed to wire tasks file");
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(content)=> serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_)=> Vec::new()
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the line");

    input
}