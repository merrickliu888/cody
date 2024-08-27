use std::io::{self, Write};

fn main() {
    // TODO
    // Natural Language Prompts (AI)
    // Build in commands
    // Command Parsing
    // Command Execution
    // I/O Redirection
    // Pipes
    // Environment Variables

    // Initialize the shell
    // start up ollama

    // Start shell loop
    loop {
        // Print prompt
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // Pre process input
        let input = input.trim();

        // Process input and parse command
        if input == "exit" {
            break;
        }
        println!("{}", input);
    }
}
