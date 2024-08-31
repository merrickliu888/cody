use reqwest;
use std::fs::File;
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};
use std::{env, io};

pub fn execute_input(input: &str) -> Result<CommandType, String> {
    let first_command_name = input.split_whitespace().next().unwrap();
    if first_command_name == "cody" {
        return execute_ai_command(input);
    } else if first_command_name == "cd" {
        return execute_cd(input);
    } else if first_command_name == "exit" {
        return Ok(CommandType::Exit);
    } else {
        match execute_commands(&mut parse_commands(input)) {
            Ok(command_type) => return Ok(command_type),
            Err(err) => return Err(format!("{}", err)),
        }
    }
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

pub fn execute_ai_command(_input: &str) -> Result<CommandType, String> {
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

// Assumes everything has been parsed correctly.
pub fn execute_commands(shell_commands: &mut Vec<ShellCommand>) -> io::Result<CommandType> {
    let mut prev_child_process: Option<Child> = None;
    let num_commands = shell_commands.len();
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
            let file = File::open(input_file)?;
            command.stdin(Stdio::from(file));
        } else if let Some(prev) = prev_child_process {
            command.stdin(Stdio::from(prev.stdout.unwrap()));
        }

        // Handling output redirection
        if let Some(output_file) = output_redirection {
            let file = File::options().create(true).append(*append).open(output_file)?;
            command.stdout(Stdio::from(file));
        } else if index < num_commands - 1 {
            command.stdout(Stdio::piped());
        }

        // Executing commands
        prev_child_process = Some(command.spawn()?);
    }

    // Waiting on last child
    let mut last_command = prev_child_process.unwrap();
    last_command.wait()?;
    return Ok(CommandType::Other);
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
