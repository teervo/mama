use crate::commands::util::{args_to_task_ids, assert_ids_exist};
use crate::error::Error;
use crate::TodoFile;

pub fn description() -> &'static str {
    "Remove a task from the list"
}

pub fn usage() -> &'static str {
    "<id of task to delete>..."
}

pub fn run(todo: &mut TodoFile, args: &[String]) -> Result<(), Error> {
    if args.is_empty() {
        return Err(Error::InsufficientArguments);
    }

    let ids = args_to_task_ids(args)?;
    assert_ids_exist(todo, &ids)?;

    for id in ids {
        if let Some(task) = todo.delete(id) {
            println!("❌ Deleted task {}, '{}'.", id, task.description);
        }
    }
    println!();
    todo.refresh_ids();

    crate::commands::list::run(todo, &[])
}
