#![allow(unused_imports)]

mod shell {
    pub mod shell_commands;
    pub mod variables;
}
use shell::shell_commands::*;
use shell::variables::*;

use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

fn main() {
    // TODO
    // Natural Language Prompts (AI) = limit use of ai functionality based on shell functionality
    // Build in commands
    // Command Parsing
    // Command Execution
    // I/O Redirection
    // Pipes
    // Environment Variables

    // Initialize the shell
    // start up ollama

    #[allow(unused_mut)]
    let mut shell_variables: HashMap<String, String> = HashMap::new();
    loop {
        // Print prompt
        let path = env::current_dir().expect("Failed to get current path.");
        print!("{}$ ", path.display());
        io::stdout().flush().expect("Failed to flush stdout");

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        let input = input.trim();
        if input == "" {
            continue;
        }

        // Checking if we are setting a variable
        if input.contains("=") {
            handle_variable_assigment(&input, &shell_variables).expect("Failed to assign variable.");
            continue;
        }

        // Parse commands
        let commands: Vec<ShellCommand> =
            ShellCommand::parse_commands(&input, &shell_variables).expect("Failed to parse commands.");
        if BUILT_IN_SHELL_COMMANDS.contains(commands[0].name) {
            ShellCommand::handle_built_in_shell_commands(&commands).expect("Failed to run built in commands.");
        // TODO how to handle exit?
        } else {
            ShellCommand::handle_shell_commands(&commands).expect("Failed to run commands.");
        }

        println!("{}", input);
    }
}
