use crate::shell::endpoints::{generate_response_local, generate_response_online};
use crate::shell::variables::{handle_variable_assigment, insert_variables};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::{env, io};

pub struct ShellCommand {
    command: Command,
    input_redirection: Option<String>,
    output_redirection: Option<String>,
    append: bool,
}

impl ShellCommand {
    fn new(
        command: Command,
        input_redirection: Option<String>,
        output_redirection: Option<String>,
        append: bool,
    ) -> ShellCommand {
        ShellCommand {
            command,
            input_redirection,
            output_redirection,
            append,
        }
    }
}

pub enum CommandType {
    Exit,
    Other,
}

pub fn execute_input(
    input: &str,
    online: &bool,
    shell_variables: &mut HashMap<String, String>,
) -> Result<CommandType, String> {
    let first_command_name = input.split_whitespace().next().unwrap();
    if first_command_name == "cody" {
        return execute_ai_command(input, online, shell_variables);
    } else if first_command_name == "cd" {
        return execute_cd(input);
    } else if first_command_name == "exit" {
        return Ok(CommandType::Exit);
    } else {
        let mut parsed_commands = parse_commands(input)?;
        match execute_commands(&mut parsed_commands) {
            Ok(command_type) => return Ok(command_type),
            Err(err) => return Err(format!("{}", err)),
        }
    }
}

pub fn parse_commands(input: &str) -> Result<Vec<ShellCommand>, String> {
    // Split by pipes
    let command_parts: Vec<&str> = input.split('|').collect();
    let mut shell_commands: Vec<ShellCommand> = Vec::new();

    for (command_part_index, command_part) in command_parts.iter().enumerate() {
        let parts: Vec<&str> = command_part.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }
        let mut command = Command::new(parts[0]);
        let mut args: Vec<&str> = Vec::new();
        let mut input_file: Option<String> = None;
        let mut out_file: Option<String> = None;
        let mut append: bool = false;

        let mut i: usize = 1;
        while i < parts.len() {
            let token = parts[i];
            if token == "<" {
                if command_part_index != 0 {
                    return Err("Input redirection only allowed for first command".to_string());
                }

                match parts.get(i + 1) {
                    Some(file) => {
                        input_file = Some(file.to_string());
                        i += 1;
                    }
                    None => return Err("No input file provided".to_string()),
                }
            } else if token == ">" {
                if command_part_index != command_parts.len() - 1 {
                    return Err("Output redirection only allowed for last command".to_string());
                }

                match parts.get(i + 1) {
                    Some(file) => {
                        out_file = Some(file.to_string());
                        i += 1;
                    }
                    None => return Err("No output file provided".to_string()),
                }
            } else if token == ">>" {
                if command_part_index != command_parts.len() - 1 {
                    return Err("Output redirection only allowed for last command".to_string());
                }

                match parts.get(i + 1) {
                    Some(file) => {
                        out_file = Some(file.to_string());
                        append = true;
                        i += 1;
                    }
                    None => return Err("No output file provided".to_string()),
                }
            } else {
                args.push(token)
            }
            i += 1;
        }
        command.args(args);
        shell_commands.push(ShellCommand::new(command, input_file, out_file, append));
    }

    return Ok(shell_commands);
}

pub fn execute_ai_command(
    input: &str,
    online: &bool,
    shell_variables: &mut HashMap<String, String>,
) -> Result<CommandType, String> {
    let user_prompt: &str = input[4..].trim_start();
    let prompt: String = format!(
        "Here are all the user set variables in a Hash Map: {:?}\n\nWrite the shell commands to accomplish this: {}",
        shell_variables, user_prompt
    );

    let llm_response = if *online {
        generate_response_online(prompt)?
    } else {
        generate_response_local(prompt)?
    };
    println!("Generated Shell Commands: {}", llm_response);
    let llm_response = llm_response.trim();

    // Parse variables here.
    let input_with_variables_inserted =
        insert_variables(llm_response, shell_variables).map_err(|err| err.to_string())?;

    // Checking if we are setting a variable else executing commands
    if input_with_variables_inserted.contains("=") {
        match handle_variable_assigment(&input_with_variables_inserted, shell_variables) {
            Ok(_) => return Ok(CommandType::Other),
            Err(err) => return Err(err.to_string()),
        }
    } else {
        let mut parsed_commands = parse_commands(input_with_variables_inserted.as_str())?;
        return execute_commands(&mut parsed_commands).map_err(|err| err.to_string());
    }
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
            let mut file_options = File::options();

            file_options.write(true).create(true);
            let file = if *append {
                file_options.append(true).open(output_file)?
            } else {
                file_options.truncate(true).open(output_file)?
            };

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
