use crate::commands::Error;
use crate::TodoFile;

pub fn description() -> &'static str {
    "Undo previous command"
}

pub fn usage() -> &'static str {
    "" // no arguments
}

pub fn run(todo: &mut TodoFile, _args: &[String]) -> Result<(), Error> {
    println!("↶ Reverting previous command...\n");
    let backup_file = TodoFile::undo_path();

    if let Ok(previous) = TodoFile::from(&backup_file) {
        *todo = previous;
        todo.save().expect("Error while saving todo.txt");
        crate::commands::list::run(todo, &[])
    } else {
        eprintln!("Error opening {}.", backup_file.to_str().unwrap());
        std::process::exit(1);
    }
}
