#![allow(unused)]

mod shell {
    pub mod shell_commands;
    pub mod variables;
}
use shell::shell_commands::*;
use shell::variables::*;

use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};

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
    // let (_ollama_server, _ollama_llm) = init_shell();
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
        // Parse variables here.
        let input_with_variables_inserted: String = match insert_variables(input, &shell_variables) {
            Ok(new_input) => new_input,
            Err(err) => {
                eprintln!("Failed to parse variable: {}", err);
                continue;
            }
        };

        // Checking if we are setting a variable
        if input_with_variables_inserted.contains("=") {
            if let Err(err) = handle_variable_assigment(&input, &mut shell_variables) {
                eprintln!("Failed to assign variable: {}", err)
            } else {
                println!("Variable assigned!");
            }
            continue;
        }

        // Parse commands
        // let commands: Vec<ShellCommand> =
        //     ShellCommand::parse_commands(&input, &shell_variables).expect("Failed to parse commands.");
        // if BUILT_IN_SHELL_COMMANDS.contains(commands[0].name) {
        //     ShellCommand::handle_built_in_shell_commands(&commands).expect("Failed to run built in commands.");
        // // TODO how to handle exit?
        // } else {
        //     ShellCommand::handle_shell_commands(&commands).expect("Failed to run commands.");
        // }

        println!("{}", input_with_variables_inserted);
    }
}

struct ChildGuard {
    child: std::process::Child,
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

fn init_shell() -> (ChildGuard, ChildGuard) {
    let ollama_server = Command::new("ollama")
        .arg("serve")
        // .stderr(Stdio::null())
        .spawn()
        .expect("Failed to initialize Cody: ollama serve");
    let ollama_llm = Command::new("ollama")
        .args(["run", "llama3.1"])
        // .stderr(Stdio::null())
        .spawn()
        .expect("Failed to initialize Cody: ollama run llama3.1");
    println!("Cody initialized with LLama3.1");
    return (ChildGuard { child: ollama_server }, ChildGuard { child: ollama_llm });
}
