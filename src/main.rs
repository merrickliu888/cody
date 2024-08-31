mod shell {
    pub mod ai;
    pub mod endpoints;
    pub mod shell_commands;
    pub mod variables;
}
use shell::ai;
use shell::shell_commands;
use shell::shell_commands::*;
use shell::variables::*;

use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

fn main() {
    // Initialize the shell
    dotenv().ok();
    let mut online = true;
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Use: `cody <online or offline>` or `cody`")
    } else if args.len() == 2 {
        match args[1].as_str() {
            "online" => (),
            "offline" => {
                let (_llm_server, _llm) = ai::init_local_llm();
                online = false;
            }
            _ => panic!("Use: `cody <online or offline>` or `cody`"),
        }
    }
    if online {
        println!("Cody initialized with online LLM")
    } else {
        println!("Cody initialized with local LLM")
    }

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

        // Checking if we are setting a variable else executing commands
        if input_with_variables_inserted.contains("=") {
            if let Err(err) = handle_variable_assigment(&input_with_variables_inserted, &mut shell_variables) {
                eprintln!("Failed to assign variable: {}", err);
            }
        } else {
            let execution_result = execute_input(&input_with_variables_inserted, &online, &mut shell_variables);
            match execution_result {
                Ok(command_type) => {
                    if let shell_commands::CommandType::Exit = command_type {
                        break;
                    }
                }
                Err(err) => eprintln!("Failed to execute commands: {}", err),
            }
        }
    }
}
