pub mod cxxqt_object;
use cxx_qt_lib::{QQmlApplicationEngine, QUrl};
use cxx_qt_lib_extras::QApplication;

fn main() {
    let mut app = QApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    let args: Vec<String> = std::env::args().collect();
    let filename = args
        .get(1)
        .expect("Please provide the git todo list as first argument");

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        // engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/rebaser/qml/main.qml"));
        engine.load(&QUrl::from("qml/main.qml"));
    }

    if let Some(engine) = engine.as_mut() {
        engine
            .as_qqmlengine()
            .on_quit(|_| {
                println!("QML Quit!");
            })
            .release();
    }

    println!("foo4");

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
