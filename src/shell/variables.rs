use std::collections::HashMap;

pub fn handle_variable_assigment(input: &str, variables: &HashMap<String, String>) -> Result<(), String> {
    println!(
        "handle_variable_assigment, input: {}, variables: {:?}",
        input, variables
    );
    Ok(())
}
