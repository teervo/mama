use strum_macros::{EnumIter, EnumMessage, EnumString};

use crate::error::Error;
use crate::TodoFile;

mod add;
mod complete;
mod help;
mod list;
mod remove;
mod uncomplete;
mod undo;
mod util;

#[derive(Debug, EnumIter, PartialEq, EnumString, EnumMessage)]
pub enum Commands {
    #[strum(serialize = "add")]
    Add,
    #[strum(serialize = "complete")]
    Complete,
    #[strum(serialize = "help")]
    Help,
    #[strum(serialize = "ls")]
    List,
    #[strum(serialize = "rm")]
    Remove,
    #[strum(serialize = "uncomplete")]
    Uncomplete,
    #[strum(serialize = "undo")]
    Undo,
}

impl Commands {
    pub fn run(&self, todo: &mut TodoFile, args: &[String]) {
        let result = match self {
            Commands::Add => add::run(todo, args),
            Commands::Complete => complete::run(todo, args),
            Commands::Help => help::run(todo, args),
            Commands::List => list::run(todo, args),
            Commands::Remove => remove::run(todo, args),
            Commands::Uncomplete => uncomplete::run(todo, args),
            Commands::Undo => undo::run(todo, args),
        };

        if let Some(error) = result.err() {
            eprintln!("⛔ Error: {}\n", error);
            self.usage();
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Commands::Add => add::description(),
            Commands::Complete => complete::description(),
            Commands::Help => help::description(),
            Commands::List => list::description(),
            Commands::Remove => remove::description(),
            Commands::Uncomplete => uncomplete::description(),
            Commands::Undo => undo::description(),
        }
    }

    pub fn usage(&self) -> String {
        match self {
            Commands::Add => add::usage().to_string(),
            Commands::Complete => complete::usage().to_string(),
            Commands::Help => help::usage(),
            Commands::List => list::usage().to_string(),
            Commands::Remove => remove::usage().to_string(),
            Commands::Uncomplete => uncomplete::usage().to_string(),
            Commands::Undo => undo::usage().to_string(),
        }
    }
}
