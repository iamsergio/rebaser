use serde_json;

/// The bridge definition for our QObject
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, other_branches_json)]
        #[qproperty(QString, work_branches_json)]
        #[qproperty(QString, text)]
        #[namespace = "my_object"]
        type RustController = super::Controller;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "loadData"]
        fn load_data(self: Pin<&mut RustController>);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

/// The Rust struct for the QObject
pub struct Controller {
    other_branches_json: QString,
    work_branches_json: QString,
    text: QString,
}

impl Default for Controller {
    fn default() -> Self {
        println!("Create controller!");
        Self {
            other_branches_json: QString::from(""),
            work_branches_json: QString::from(""),
            text: QString::from(""),
        }
    }
}

#[derive(Debug, Clone)]
enum Command {
    PickCommand { sha1: String, message: String },
    UpdateRefCommand { ref_name: String },
    LabelCommand { name: String },
    ResetCommand { name: String },
    MergeCommand { name: String },
}

fn commands_from_git_text(git_todo_text: &String) -> Vec<Command> {
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
            "merge" => commands.push(Command::MergeCommand {
                name: parts[1..].join(" "),
            }),
            _ => {}
        }
    }

    commands
}

fn work_branch_json(commands: &Vec<Command>) -> String {
    let last_reset_index = commands
        .iter()
        .enumerate()
        .rev()
        .find(|(_, command)| matches!(command, Command::ResetCommand { .. }))
        .map(|(index, _)| index);

    let work_branch_commands = if let Some(index) = last_reset_index {
        commands[index..].to_vec()
    } else {
        commands.to_vec()
    };

    let mut worklist = Vec::new();

    for command in work_branch_commands {
        match command {
            Command::PickCommand { sha1, message } => {
                let mut child = serde_json::Map::new();
                child.insert("name".into(), serde_json::Value::String(message));
                worklist.push(serde_json::Value::Object(child));
            }
            _ => {}
        }
    }

    let mut root = serde_json::Map::new();
    root.insert("children".into(), serde_json::Value::Array(worklist));

    return serde_json::to_string(&root).unwrap();
}

impl qobject::RustController {
    pub fn load_data(self: Pin<&mut Self>) {
        let args: Vec<String> = std::env::args().collect();
        let filename = args
            .get(1)
            .expect("Please provide the git todo list as first argument");
        let file_contents =
            std::fs::read_to_string(filename).expect("Should have been able to read the file11");

        self.set_text(file_contents.clone().into());

        let commands = commands_from_git_text(&file_contents);
        println!("foo31");
        // self.set_work_branches_json(work_branch_json(&commands).into());
    }
}
