use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

pub fn insert_variables<'a>(input: &'a str, variables: &HashMap<String, String>) -> Result<String, VariableErrors> {
    // For now, variables have to be used in this format: ${...}, and ignore single vs double quote expansion. We will always expand.
    let re = Regex::new(r"\$\{([^}]+)\}").unwrap();
    let replacement = |caps: &Captures| -> Result<String, VariableErrors> {
        let var_name = caps[1].to_string();
        if !variables.contains_key(&var_name) {
            return Err(VariableErrors::InvalidVariableName {
                variable_name: var_name,
            });
        }
        return Ok(variables.get(&var_name).unwrap().clone());
    };
    return replace_all(&re, input, replacement);
}

pub fn handle_variable_assigment<'a>(
    input: &'a str,
    variables: &mut HashMap<String, String>,
) -> Result<(), VariableErrors> {
    let parts: Vec<&str> = input.splitn(2, '=').collect();
    let name: &'a str = parts[0];
    if !check_valid_variable_name(name) {
        return Err(VariableErrors::InvalidVariableName {
            variable_name: name.to_string(),
        });
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

pub enum VariableErrors {
    InvalidVariableName { variable_name: String },
    InvalidVariableValue,
}

impl Display for VariableErrors {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::InvalidVariableName { variable_name } => write!(f, "Invalid variable name: {}", variable_name),
            Self::InvalidVariableValue => write!(f, "Invalid variable value"),
        }
    }
}

fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}
