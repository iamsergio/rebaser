use crate::rebaser;

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

impl qobject::RustController {
    pub fn load_data(mut self: Pin<&mut Self>) {
        let args: Vec<String> = std::env::args().collect();
        let filename = args
            .get(1)
            .expect("Please provide the git todo list as first argument");
        let file_contents =
            std::fs::read_to_string(filename).expect("Should have been able to read the file11");

        match rebaser::commands_from_git_text(&file_contents) {
            Ok(commands) => {
                match rebaser::work_branch_json(&commands) {
                    Ok(json) => {
                        self.as_mut().set_work_branches_json(json.into());
                    }
                    Err(e) => {
                        println!("Error getting work branch json: {}", e);
                    }
                }

                match rebaser::branches_json(&commands) {
                    Ok(json) => {
                        self.as_mut().set_other_branches_json(json.into());
                    }
                    Err(e) => {
                        println!("Error getting branches json: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error parsing git commands: {}", e);
                return;
            }
        }

        self.as_mut().set_text(file_contents.clone().into());
    }
}
