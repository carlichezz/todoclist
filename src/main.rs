use clap::{Arg, Command, command};
use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "│ {:<8}│ {:<20}│ {:<20}│", 
            &self.id,
            &self.name[..self.name.len().min(18)],
            &self.description[..self.description.len().min(18)]
        )
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
struct Task{
    id: u32,
    name: String,
    description: String,
}

fn main(){
    let mut task_list: Vec<Task>= get_tasks();
    let banner = include_str!("banner.txt");
    let match_result = command!().
    about(banner)

    .subcommand(
        Command::new("add")
        .about("Add new task")
        .arg(Arg::new("task-name")
            .num_args(1..)
            .short('n')
            .help("Task name")
            .required(true)
        )
        .arg(Arg::new("description")
            .short('d')
            .help("Task description")
            .num_args(1..)

        )
    )
            
    .subcommand(
        Command::new("list")
        .about("List all tasks")
    )
    .get_matches();

    let subcommand_name = match_result.subcommand_name().unwrap();
    if subcommand_name== "add"{
        let task_args = match_result.subcommand_matches("add").unwrap();

        let task_name = task_args
            .get_many::<String>("task-name")
            .unwrap_or_default()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        let task_description = task_args
            .get_many::<String>("description")
            .unwrap_or_default()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        let task_id ;
        if task_list.is_empty(){
            task_id=0;
        } else {
            task_id=task_list.last().unwrap().id + 1;
        }
            let task: Task = Task {
            id: task_id,
            name: task_name.to_string(),
            description: task_description.to_string()
        };
        task_list.push(task);
        println!("{:?}", task_list.last());
        save_tasks(&task_list);
    } else if subcommand_name=="list" {
        print_tasks(&task_list);
    }
}

fn get_tasks() -> Vec<Task> {
    let file = std::fs::read_to_string("tasks.json");
    match file {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        Err(_) => vec![]
    }
}
fn save_tasks(task_list: &Vec<Task>) {
    let content = serde_json::to_string_pretty(task_list).unwrap();
    std::fs::write("tasks.json", content).unwrap();
}
fn print_tasks(task_list: &Vec<Task>) {
    println!("┌─────────┬─────────────────────┬─────────────────────┐");
    println!("│ {:<8}│ {:<20}│ {:<20}│","ID", "Task", "Description");
    for task in task_list {
        println!("├─────────┼─────────────────────┼─────────────────────┤");
        println!("{}", task);
    }
    println!("└─────────┴─────────────────────┴─────────────────────┘");

}