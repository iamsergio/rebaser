use serde_json;

#[derive(Debug, Clone)]
pub enum Command {
    PickCommand { sha1: String, message: String },
    UpdateRefCommand { ref_name: String },
    LabelCommand { name: String },
    ResetCommand { name: String },
    MergeCommand { name: String },
}

#[derive(Debug)]
struct Branch {
    commands: Vec<Command>,
}

impl Branch {
    pub fn name(&self) -> String {
        if let Command::LabelCommand { name } = &self.commands[self.commands.len() - 1] {
            name.clone()
        } else {
            String::default()
        }
    }

    pub fn onto_branch_name(&self) -> String {
        if let Command::ResetCommand { name } = &self.commands[0] {
            name.clone()
        } else {
            String::default()
        }
    }
}

impl Command {
    pub fn is_draggable(&self) -> bool {
        match self {
            Command::PickCommand { .. } => true,
            _ => false,
        }
    }

    pub fn description(&self) -> String {
        match self {
            Command::PickCommand { sha1: _, message } => {
                format!("pick {}", message)
            }
            Command::UpdateRefCommand { ref_name } => {
                format!("u {}", ref_name)
            }
            Command::ResetCommand { name } => {
                format!("reset {}", name)
            }
            Command::MergeCommand { name } => {
                format!("merge {}", name)
            }
            Command::LabelCommand { name } => {
                format!("label {}", name)
            }
        }
    }
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

pub fn branches(commands: &[Command]) -> Result<Vec<Branch>, String> {
    if !matches!(commands.first(), Some(Command::LabelCommand { .. })) {
        return Err("First command must be a label".to_string());
    }

    // Discard the label
    let commands = &commands[1..];

    let mut branches = Vec::new();
    let mut current_branch = Vec::new();

    for command in commands {
        if current_branch.is_empty() && !matches!(command, Command::ResetCommand { .. }) {
            return Err("Branch must start with reset command".to_string());
        }

        current_branch.push(command.clone());

        if let Command::LabelCommand { .. } = command {
            branches.push(Branch {
                commands: current_branch.clone(),
            });
            current_branch.clear();
        }
    }

    if branches.is_empty() {
        return Err("No branches found".to_string());
    }

    Ok(branches)
}

pub fn work_branch_json(commands: &Vec<Command>) -> Result<String, String> {
    let commands = work_branch_commands(commands)?;

    // "reset onto" isn't interesting to show
    let commands = commands.iter().filter(|cmd| {
        !matches!(
            cmd,
            Command::ResetCommand { .. } | Command::LabelCommand { .. }
        )
    });

    let mut worklist = Vec::new();

    for command in commands {
        let mut child = serde_json::Map::new();
        child.insert(
            "name".into(),
            serde_json::Value::String(command.description()),
        );

        child.insert(
            "is_draggable".into(),
            serde_json::Value::Bool(command.is_draggable()),
        );

        worklist.push(serde_json::Value::Object(child));
    }

    let mut root = serde_json::Map::new();
    root.insert("children".into(), serde_json::Value::Array(worklist));

    Ok(serde_json::to_string(&root).unwrap())
}

pub fn branches_json(commands: &Vec<Command>) -> Result<String, String> {
    let branches = branches(commands)?;

    let mut root = serde_json::Map::new();
    let mut branch_list_json = Vec::new();

    for branch in branches {
        let mut branch_obj = serde_json::Map::new();
        branch_obj.insert("name".into(), serde_json::Value::String(branch.name()));

        let mut command_list_json = Vec::new();

        // Only include non-reset and non-label commands
        for command in branch
            .commands
            .iter()
            .filter(|cmd| matches!(cmd, Command::PickCommand { .. }))
        {
            let mut child_commands = serde_json::Map::new();
            child_commands.insert(
                "name".into(),
                serde_json::Value::String(command.description()),
            );

            command_list_json.push(serde_json::Value::Object(child_commands));
        }

        branch_obj.insert(
            "children".into(),
            serde_json::Value::Array(command_list_json),
        );

        branch_list_json.push(serde_json::Value::Object(branch_obj));
    }

    root.insert(
        "children".into(),
        serde_json::Value::Array(branch_list_json),
    );

    Ok(serde_json::to_string(&root).unwrap())
}
