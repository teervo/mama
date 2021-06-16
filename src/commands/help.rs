use crate::commands::Commands;
use crate::error::Error;
use crate::TodoFile;
use strum::{EnumMessage, IntoEnumIterator};

use std::fmt::Write;
use std::str::FromStr;

pub fn description() -> &'static str {
    "Show help for a command"
}

pub fn usage() -> String {
    let mut s = String::new();
    writeln!(s, "A command line application for managing todo.txt\n").ok();
    writeln!(s, "Usage: mama <command> [arguments]\n").ok();

    writeln!(s, "Available commands:").ok();
    for cmd in Commands::iter() {
        let name = cmd.get_serializations().get(0).unwrap_or(&"");
        writeln!(s, "{:<14}{}", name, cmd.description()).ok();
    }
    s
}

pub fn run(_f: &mut TodoFile, args: &[String]) -> Result<(), Error> {
    let arg = args.get(0).map(|s| Commands::from_str(s).ok()).flatten();

    match arg {
        Some(x) => {
            let cmd = x.get_serializations().get(0).unwrap();
            println!("mama {} - {}\n", cmd, x.description());
            println!("Usage: mama {} {}", cmd, x.usage())
        }
        None => println!("{}", usage()),
    };

    Ok(())
}
