#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::process::{Child, Command};

pub static BUILT_IN_SHELL_COMMANDS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut set: HashSet<&str> = HashSet::new();
    set.insert("cd");
    set.insert("exit");
    return set;
});

pub fn execute_input(input: &str) -> Result<CommandType, String> {
    let first_command_name = input.split_whitespace().next().unwrap();
    if first_command_name == "cody" {
        execute_ai_command(input);
    } else if first_command_name == "cd" {
        execute_cd(input);
    } else if first_command_name == "exit" {
        return Ok(CommandType::Exit);
    } else {
        execute_commands(&mut parse_commands(input));
    }
    return Ok(CommandType::Other);
}

pub fn parse_commands(input: &str) -> Vec<Command> {
    // Split by pipes

    // for each split:
    // split by spaces
    // first element is

    unimplemented!()
}

pub fn execute_ai_command(input: &str) {
    // Should only be one command here
    unimplemented!()
}

pub fn execute_cd(input: &str) {
    unimplemented!()
}

pub fn execute_commands(commands: &mut Vec<Command>) {
    let mut child_processes: Vec<Child> = Vec::new();

    for command in commands.iter_mut() {
        let command_spawn = command.spawn();
        match command_spawn {
            Ok(child) => child_processes.push(child),
            Err(err) => eprintln!("{}", err),
        }
    }

    for child in child_processes.iter_mut() {
        let child_status = child.wait();
        if let Ok(exit_status) = child_status {
            if !exit_status.success() {
                eprintln!("Command failed")
            }
        } else if let Err(err) = child_status {
            eprintln!("{}", err);
        }
    }
}

pub enum CommandType {
    Exit,
    Other,
}
