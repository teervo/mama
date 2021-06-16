use crate::commands::Error;
use crate::{TaskId, TodoFile};

pub fn args_to_task_ids(args: &[String]) -> Result<Vec<TaskId>, Error> {
    args.iter()
        .map(|arg| arg.parse::<TaskId>())
        .into_iter()
        .collect()
}

pub fn assert_ids_exist(todo: &TodoFile, ids: &[TaskId]) -> Result<(), Error> {
    if let Some(id) = ids.iter().find(|id| !todo.has_id(id)) {
        Err(Error::IdNotFound(*id))
    } else {
        Ok(())
    }
}
