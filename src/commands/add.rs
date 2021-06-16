use crate::error::Error;
use crate::Task;
use crate::TaskPriority;
use crate::TodoFile;

use colored::*;
use gregorian::Date;

pub fn description() -> &'static str {
    "Add a new task to the list"
}

pub fn usage() -> &'static str {
    "[-p <PRIORITY>] <description of task to add>
    -p PRIORITY   set the priority level of the added task"
}

pub fn run(todo: &mut TodoFile, args: &[String]) -> Result<(), Error> {
    let mut args = Vec::from(args);

    let priority = parse_priority(&mut args);
    let description = args.join(" ").trim().to_string();

    if description.is_empty() {
        return Err(Error::InsufficientArguments);
    }

    println!("{} Adding '{}' to todo.txt...\n", "+".green(), description);
    todo.add(Task {
        description,
        priority,
        creation_date: Some(Date::today()),
        ..Task::default()
    });

    match priority {
        Some(_) => crate::commands::list::run(todo, &["-l".to_string()]),
        None => crate::commands::list::run(todo, &[]),
    }
}

fn parse_priority(args: &mut Vec<String>) -> Option<TaskPriority> {
    let pri_flag = args.iter().position(|x| x == "-p" || x == "--priority");
    match pri_flag {
        Some(index) => {
            let priority = format!("({})", args[index + 1].to_uppercase()).parse::<TaskPriority>();
            args.remove(index);
            args.remove(index);
            priority.ok()
        }
        None => None,
    }
}
