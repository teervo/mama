mod commands;
use commands::*;

mod error;
mod task;
mod task_id;
mod task_priority;
mod todo_file;

pub use task::Task;
pub use task_id::TaskId;
pub use task_priority::TaskPriority;
pub use todo_file::TodoFile;

use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    let subcommand = args.nth(1).unwrap_or_else(|| "ls".to_string());
    let sub_args = args.collect::<Vec<String>>();

    let mut todo = match TodoFile::new() {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Unable to open todo.txt for reading.");
            return Err(x);
        }
    };

    match Commands::from_str(&subcommand) {
        Ok(x) => x.run(&mut todo, &sub_args),
        _ => println!("{}", Commands::Help.usage()),
    };

    if todo.unwritten_changes() {
        todo.save().expect("Error while saving todo.txt");
    }

    Ok(())
}
