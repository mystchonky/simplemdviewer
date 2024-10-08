use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

mod bridge;

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from(
            "qrc:/qt/qml/org/kde/simplemdviewer/src/qml/main.qml",
        ));
    }

    if let Some(app) = app.as_mut() {
        println!("wwww");
        app.exec();
    }
}
