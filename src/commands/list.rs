use crate::error::Error;
use crate::Task;
use crate::TaskId;
use crate::TodoFile;

use colored::*;
use gregorian::Date;
use iterate::iterate;
use std::cmp::max;
use terminal_size::{terminal_size, Width};

pub fn description() -> &'static str {
    "List all tasks"
}

pub fn usage() -> &'static str {
    "[-l/--long] [<FILTER>]
    -l, --long    detailed output
    FILTER        only show tasks matching words in FILTER"
}

pub fn run(todo: &mut TodoFile, args: &[String]) -> Result<(), Error> {
    let detailed_output = args.iter().any(|s| s == "-l" || s == "--long");
    let filters = args
        .iter()
        .filter(|&s| s != "-l" && s != "--long")
        .cloned()
        .collect::<Vec<String>>();

    // ID column width is 2 for the header "ID" or the length of
    // the longest ID, whichever is largest
    let id_column_width = max(2, todo.tasks().len().to_string().len());

    print_header(detailed_output, id_column_width);
    for (id, task) in todo.iter() {
        if filters.is_empty()
            || filters
                .iter()
                .any(|s| task.description.to_lowercase().contains(&s.to_lowercase()))
        {
            print_task(id, task, detailed_output, id_column_width);
        }
    }

    Ok(())
}

fn print_header(detailed_output: bool, id_column_size: usize) {
    if detailed_output {
        println!(
            "  {:>size$} Pri Completed  Created",
            "ID",
            size = id_column_size
        )
    } else {
        println!("  {:>size$}", "ID", size = id_column_size)
    }
}

fn print_task(id: &TaskId, task: &Task, detailed_output: bool, id_column_width: usize) {
    let mut output = match task.completed {
        true => format!("{:2}", "✔".green()),
        false => "  ".to_string(),
    };

    output.push_str(&format!("{:>width$} ", id, width = id_column_width));

    if detailed_output {
        output.push_str(&format!(
            "{:^3} ",
            task.priority.map(|p| p.to_string()).unwrap_or_default()
        ));
        output.push_str(&format_date(task.completion_date));
        output.push_str(&format_date(task.creation_date));
    }

    let description_width = match detailed_output {
        true => task.description.len() + 1,
        false => {
            let terminal_width = terminal_size().map(|(Width(w), _)| w).unwrap_or(80);
            (terminal_width as usize) - 3 - id_column_width
        }
    };

    output.push_str(&format_description(&task.description, description_width));

    if task.completed {
        println!("{}", output.strikethrough())
    } else {
        println!("{}", output)
    }
}

fn format_date(date: Option<Date>) -> String {
    format!("{:10} ", date.map_or(String::new(), |x| x.to_string()))
}

fn format_description(description: &str, available_width: usize) -> String {
    // If necessary, truncate desciption to fit terminal width
    let description: String = match description.len() > available_width {
        true => iterate![..description.chars().take(available_width - 1), '…'].collect(),
        false => description.to_string(),
    };

    description
        .split_whitespace()
        // choose coloring based on first character
        .map(|word| match word.chars().next() {
            Some('@') => format!("{}", word.yellow()),
            Some('+') => format!("{}", word.magenta()),
            _ => format!("{}", word.cyan()),
        })
        .collect::<Vec<String>>()
        .join(" ")
}
