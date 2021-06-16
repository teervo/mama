use crate::commands::util::{args_to_task_ids, assert_ids_exist};
use crate::error::Error;
use crate::TodoFile;

pub fn description() -> &'static str {
    "Mark a previously finished task as uncompleted"
}

pub fn usage() -> &'static str {
    "<id of finished task>..."
}

pub fn run(todo: &mut TodoFile, args: &[String]) -> Result<(), Error> {
    if args.is_empty() {
        return Err(Error::InsufficientArguments);
    }

    let ids = args_to_task_ids(args)?;
    assert_ids_exist(todo, &ids)?;

    for id in ids {
        if let Some(task) = todo.uncomplete(id) {
            println!("☐ Marked task {}, '{}' as unfinished", id, task.description);
        }
    }
    println!();

    crate::commands::list::run(todo, &[])
}
