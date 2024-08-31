use reqwest;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};

pub fn execute_input(input: &str) -> Result<CommandType, String> {
    let first_command_name = input.split_whitespace().next().unwrap();
    if first_command_name == "cody" {
        execute_ai_command(input);
    } else if first_command_name == "cd" {
        return execute_cd(input);
    } else if first_command_name == "exit" {
        return Ok(CommandType::Exit);
    } else {
        execute_commands(&mut parse_commands(input));
    }
    return Ok(CommandType::Other);
}

pub fn parse_commands(input: &str) -> Vec<ShellCommand> {
    // Split by pipes
    let command_parts = input.split('|');
    let mut commands: Vec<ShellCommand> = Vec::new();

    for command_part in command_parts {
        let parts = command_part.trim().split_whitespace();
        let mut args: Vec<&str> = Vec::new();
    }

    // for each split:
    // split by spaces
    // first element is

    return commands;
}

pub fn execute_ai_command(_input: &str) {
    unimplemented!()
    // // get prompt
    // let user_prompt: &str = input[4..].trim_start();

    // // send prompt to llama and wait for response
    // let url = "http://localhost:11434/api/generate";
    // let client = reqwest::blocking::Client::new();
    // let mut req_body = HashMap::new();
    // // req_body.insert("stream", false);
    // // req_body.insert("model", "llama3");

    // let res = client.post(url).json(&req_body).send()?;

    // // parse response

    // // execute commands

    // unimplemented!()
}

pub fn execute_cd(input: &str) -> Result<CommandType, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() > 2 {
        return Err("Too many arguments to cd".to_string());
    }

    let dir = {
        if parts.len() == 1 {
            "/"
        } else {
            parts[1]
        }
    };
    let path = Path::new(dir);
    if let Err(err) = env::set_current_dir(&path) {
        return Err(format!("cd error: {}, {}", dir, err));
    }

    return Ok(CommandType::Other);
}

pub fn execute_commands(shell_commands: &mut Vec<ShellCommand>) {
    let mut child_processes: Vec<Child> = Vec::new();

    let mut prev_process = None;
    for (
        index,
        ShellCommand {
            command,
            input_redirection,
            output_redirection,
            append,
        },
    ) in shell_commands.iter_mut().enumerate()
    {
        // Handling input redirection
        if let Some(input_file) = input_redirection {
            let file_result = File::open(input_file);
            match file_result {
                Ok(file) => {
                    command.stdin(Stdio::from(file));
                }
                Err(err) => eprintln!("{}", err),
            }
        } else if index > 0 {
            let prev_command = &child_processes[index - 1];
            command.stdin(Stdio::from(prev_command.stdout.unwrap()));
        }

        // Handling output redirection
        if let Some(output_file) = output_redirection {
            let file_result = File::options().create(true).append(*append).open(output_file);
            match file_result {
                Ok(file) => {
                    command.stdout(Stdio::from(file));
                }
                Err(err) => eprintln!("{}", err),
            }
        }

        // Executing commands
        let command_spawn = command.spawn();
        match command_spawn {
            Ok(child) => {
                child_processes.push(child);
                prev_process = Some(&child);
            }
            Err(err) => eprintln!("{}", err),
        }
    }

    // Collection exit statuses (possibly change and use wait_with_output, or wait with id)
    // TODO wait with pid, collect vector of pid, and keep track of prev child to get stdin and out
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

pub struct ShellCommand {
    command: Command,
    input_redirection: Option<String>,
    output_redirection: Option<String>,
    append: bool,
}

pub enum CommandType {
    Exit,
    Other,
}
