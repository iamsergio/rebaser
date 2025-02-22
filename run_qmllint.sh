cmake --preset=dev &&
	cmake --build build-dev &&
	qmllint -I ./build-dev/qml/ -I ./target/debug/build/rebaser-11cc38be3fbba01f/out/qt-build-utils/qml_modules/ qml/main.qml
