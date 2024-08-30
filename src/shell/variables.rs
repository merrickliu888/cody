use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

// pub fn insert_variables(input: &str, variables: &HashMap<String, String>) -> String {
//     // For now, variables have to be used in this format: ${...}, and ignore single vs double quote expansion. We will always expand.
//     // let re = Regex::new(r"\$\{([^}]+\)}").unwrap();
//     // let input_with_variables = re.replace_all(input, |caps| {
//     //     let var_name = caps[0];
//     //     variables.get(var_name).expect("stiff")
//     // })
//     // println!(input_with_variables);
//     // return input_with_variables;
//     let result = String::new();

//     return result;
// }

pub fn handle_variable_assigment<'a>(
    input: &'a str,
    variables: &mut HashMap<String, String>,
) -> Result<(), VariableErrors<'a>> {
    let parts: Vec<&str> = input.splitn(2, '=').collect();
    let name: &'a str = parts[0];
    if !check_valid_variable_name(name) {
        return Err(VariableErrors::InvalidVariableName { variable_name: name });
    }

    let value: &'a str = parts[1];
    if !check_valid_variable_value(value) {
        return Err(VariableErrors::InvalidVariableValue);
    }
    let trimmed_value: &'a str = value.trim_matches('"');

    variables.insert(name.to_string(), trimmed_value.to_string());
    return Ok(());
}

fn check_valid_variable_name(name: &str) -> bool {
    let chars: Vec<char> = name.chars().collect();
    if chars.len() == 0
        || !(chars[0] == '_' || chars[0].is_alphabetic())
        || chars.iter().any(|c| c.is_whitespace())
        || !chars.iter().all(|c| *c == '_' || c.is_alphanumeric())
    {
        return false;
    }
    return true;
}

fn check_valid_variable_value(value: &str) -> bool {
    let chars: Vec<char> = value.chars().collect();

    // For simplicity, we'll say for now variables values must be surrounded by `"`
    if chars.len() < 2 || chars[0] != '"' || chars[chars.len() - 1] != '"' {
        return false;
    }
    return true;
}

pub enum VariableErrors<'a> {
    InvalidVariableName { variable_name: &'a str },
    InvalidVariableValue,
}

impl Display for VariableErrors<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::InvalidVariableName { variable_name } => write!(f, "Invalid variable name: {}", variable_name),
            Self::InvalidVariableValue => write!(f, "Invalid variable value"),
        }
    }
}
