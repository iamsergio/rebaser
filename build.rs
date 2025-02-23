use std::env;

use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    let out_dir = format!("{}/qt-build-utils/qml_modules/", out_dir);
    let qmlls_ini_in =
        std::fs::read_to_string("qmlls.ini.in").expect("Failed to read qmlls.ini.in");

    let qmlls_ini = qmlls_ini_in
        .replace("@CXXQT_OUT_DIR@", &out_dir)
        .replace("@CMAKE_BINARY_DIR@", "/data/sources/rebaser/build-dev/");
    std::fs::write(".qmlls.ini", qmlls_ini).expect("Failed to write to .qmlls.ini");

    CxxQtBuilder::new()
        .qt_module("Network")
        .cc_builder(|cc| {
            cc.include("src");
        })
        .qml_module(QmlModule {
            uri: "com.kdab.rebaser",
            rust_files: &["src/cxxqt_object.rs"],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        .build();

    println!("cargo:rustc-link-search=native=build-dev");
    println!("cargo:rustc-link-lib=static=RebaserCpp");
}
