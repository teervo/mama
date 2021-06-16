use crate::Task;
use crate::TaskId;
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::iter::Iterator;

#[derive(Default)]
pub struct TodoFile {
    tasks: BTreeMap<TaskId, Task>,
    changed: bool,
}

impl TodoFile {
    pub fn new() -> std::io::Result<Self> {
        match Self::from(&Self::path()) {
            Err(x) if x.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            x => x,
        }
    }

    pub fn from(path: &std::path::Path) -> std::io::Result<Self> {
        let masks = std::fs::read_to_string(path)?
            .lines()
            .filter_map(|s| s.parse::<Task>().ok())
            .enumerate()
            .map(|(a, t)| (TaskId(a), t))
            .collect();

        Ok(Self {
            tasks: masks,
            changed: false,
        })
    }

    pub fn unwritten_changes(&self) -> bool {
        self.changed
    }

    /// Checks whether `id` is a valid identifier for the file
    pub fn has_id(&self, id: &TaskId) -> bool {
        self.tasks.contains_key(id)
    }

    /// After deleting a task, the keys in `tasks` are not necessarily
    /// just a simple enumeration of the values anymore. This fixes that.
    pub fn refresh_ids(&mut self) {
        self.tasks = self
            .tasks
            .values()
            .enumerate()
            .map(|(id, task)| (TaskId(id), task.clone()))
            .collect();
    }

    /// Writes the tasks to the disk, backing up any pre-existing file
    pub fn save(&mut self) -> std::io::Result<()> {
        if Self::path().exists() {
            // Backup existing todo.txt
            std::fs::copy(Self::path(), Self::undo_path())?;
        }

        let mut file = std::fs::File::create(Self::path())?;
        self.tasks().try_for_each(|s| writeln!(file, "{}", s))?;
        self.changed = false;

        Ok(())
    }

    /// Returns an iterator of the (TaskId, Task) pairs
    pub fn iter(&self) -> impl Iterator<Item = (&TaskId, &Task)> {
        self.tasks.iter()
    }

    /// Returns an iterator over the tasks.
    pub fn tasks(&self) -> std::collections::btree_map::Values<TaskId, Task> {
        self.tasks.values()
    }

    pub fn add(&mut self, task: Task) {
        let id = TaskId(self.tasks.len());
        self.tasks.insert(id, task);
        self.changed = true;
    }

    /// Returns true if `index` is a valid, existing task ID
    pub fn has_task(&self, index: TaskId) -> bool {
        index < self.tasks.len()
    }

    /// Sets the state of task at `index` to completed.
    /// On successs, returns the finished task. If index is out of bounds,
    /// returns None.
    pub fn complete(&mut self, index: TaskId) -> Option<&Task> {
        match self.tasks.get_mut(&index) {
            Some(task) => {
                task.complete();
                self.changed = true;
                Some(&*task)
            }
            None => None,
        }
    }

    /// Sets the state of task at `index` to uncompleted.
    /// On successs, returns the modified task. If index is out of bounds,
    /// returns an error.
    pub fn uncomplete(&mut self, index: TaskId) -> Option<&Task> {
        match self.tasks.get_mut(&index) {
            Some(task) => {
                task.uncomplete();
                self.changed = true;
                Some(&*task)
            }
            None => None,
        }
    }

    /// Deletes the task at `index`.
    /// On successs, returns the deleted task. If index is out of bounds,
    /// returns None.
    pub fn delete(&mut self, index: TaskId) -> Option<Task> {
        self.changed = true;
        self.tasks.remove(&index)
    }

    pub fn path() -> std::path::PathBuf {
        dirs::home_dir()
            .expect("Unable to determine home directory.")
            .join("todo.txt")
    }

    pub fn undo_path() -> std::path::PathBuf {
        dirs::cache_dir()
            .expect("Unable to determine cache directory.")
            .join("todo.txt.backup")
    }
}
