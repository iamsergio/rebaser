use serde_json;

#[derive(Debug, Clone)]
pub enum Command {
    PickCommand { sha1: String, message: String },
    UpdateRefCommand { ref_name: String },
    LabelCommand { name: String },
    ResetCommand { name: String },
    MergeCommand { name: String },
}

pub fn commands_from_git_text(git_todo_text: &String) -> Result<Vec<Command>, String> {
    let mut commands = vec![];

    for line in git_todo_text.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "pick" => commands.push(Command::PickCommand {
                sha1: parts[1].to_string(),
                message: parts[2..].join(" "),
            }),
            "update-ref" => commands.push(Command::UpdateRefCommand {
                ref_name: parts[1].to_string(),
            }),
            "label" => commands.push(Command::LabelCommand {
                name: parts[1].to_string(),
            }),
            "reset" => commands.push(Command::ResetCommand {
                name: parts[1].to_string(),
            }),
            "merge" => {
                let mut text = parts[1..].join(" ");
                if text.contains("#") {
                    text = text.split("#").next().unwrap().trim().to_string();
                }
                commands.push(Command::MergeCommand { name: text })
            }
            _ => {}
        }
    }

    if commands.is_empty() {
        return Err("No commands found".to_string());
    }

    if let Some(Command::LabelCommand { name }) = commands.first() {
        if name != "onto" {
            return Err("Label command must have value 'onto'".to_string());
        }
    } else {
        return Err("First command must be a label command".to_string());
    }

    Ok(commands)
}

pub fn work_branch_commands(commands: &Vec<Command>) -> Result<Vec<Command>, String> {
    let last_reset_index = commands
        .iter()
        .enumerate()
        .rev()
        .find(|(_, command)| matches!(command, Command::ResetCommand { .. }))
        .map(|(index, _)| index);

    if let Some(index) = last_reset_index {
        Ok(commands[index..].to_vec())
    } else {
        Err("No reset command found".to_string())
    }
}

pub fn work_branch_json(commands: &Vec<Command>) -> Result<String, String> {
    let commands = work_branch_commands(commands)?;

    // Discard the 1st command, which is always "reset onto", we won't display it in the GUI
    let commands = commands[1..].to_vec();

    let mut worklist = Vec::new();

    for command in commands {
        match command {
            Command::PickCommand { sha1, message } => {
                let mut child = serde_json::Map::new();
                child.insert(
                    "name".into(),
                    serde_json::Value::String(format!("pick {}", message)),
                );
                worklist.push(serde_json::Value::Object(child));
            }
            Command::UpdateRefCommand { ref_name } => {
                let mut child = serde_json::Map::new();
                child.insert(
                    "name".into(),
                    serde_json::Value::String(format!("u {}", ref_name)),
                );
                worklist.push(serde_json::Value::Object(child));
            }
            Command::ResetCommand { name } => {
                let mut child = serde_json::Map::new();
                child.insert(
                    "name".into(),
                    serde_json::Value::String(format!("reset {}", name)),
                );
                worklist.push(serde_json::Value::Object(child));
            }
            Command::MergeCommand { name } => {
                let mut child = serde_json::Map::new();
                child.insert(
                    "name".into(),
                    serde_json::Value::String(format!("merge {}", name)),
                );
                worklist.push(serde_json::Value::Object(child));
            }
            _ => continue,
        }
    }

    let mut root = serde_json::Map::new();
    root.insert("children".into(), serde_json::Value::Array(worklist));

    return Ok(serde_json::to_string(&root).unwrap());
}
