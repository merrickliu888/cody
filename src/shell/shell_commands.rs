#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

pub static BUILT_IN_SHELL_COMMANDS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut set: HashSet<&str> = HashSet::new();
    set.insert("cd");
    set
});

#[derive(Debug)]
pub struct ShellCommand<'a> {
    pub name: &'a str,
    pub arguments: &'a [String],
    // TODO think about how to handle redirection and pipes
    // pub input: Option<String>,
    // pub output: Option<String>,

    // TODO read more on life times / think about it here
    // TODO figure out type for arguments
}

impl<'a> ShellCommand<'a> {
    pub fn new(name: &'a str, arguments: &'a [String]) -> ShellCommand<'a> {
        ShellCommand {
            name: name,
            arguments: arguments,
        }
    }

    pub fn parse_commands(input: &str, variables: &HashMap<String, String>) -> Result<Vec<ShellCommand<'a>>, ()> {
        println!("parsed_commands, input: {}, variables: {:?}", input, variables);
        let fake_commands = vec![Self::new("asdf", &[])];
        Ok(fake_commands)
    }

    pub fn handle_shell_commands(shell_commands: &[ShellCommand]) -> Result<(), ()> {
        println!("handled_shell_command: {:?}", shell_commands);
        Ok(())
    }

    pub fn handle_built_in_shell_commands(shell_commands: &[ShellCommand]) -> Result<(), ()> {
        println!("handled_built_in_shell_command: {:?}", shell_commands);
        Ok(())
    }
}
