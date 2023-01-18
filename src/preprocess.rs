pub fn split_pipes(input: String) -> Vec<String> {
    // split input into commands by | and collect into a vector
    let commands = input
        .trim()
        .split("|")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    commands
}

pub fn split_command(input: String) -> Vec<String> {
    // split input into commands by " " and collect into a vector
    let commands = input
        .trim()
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // find double quotes and merge them into one string
    let mut merged_commands = Vec::new();
    let mut in_quotes = false;
    let mut current_command = String::new();

    for command in commands {
        if command.starts_with("\"") {
            in_quotes = true;
            current_command.push_str(command.trim_start_matches("\""));
        } else if command.ends_with("\"") {
            in_quotes = false;
            current_command.push_str(" ");
            current_command.push_str(command.trim_end_matches("\""));
            merged_commands.push(current_command);
            current_command = String::new();
        } else if in_quotes {
            current_command.push_str(" ");
            current_command.push_str(command.as_str());
        } else {
            merged_commands.push(command);
        }
    }

    merged_commands
}
