cargo build &&
	cmake --preset=dev &&
	cmake --build build-dev &&
	QT_BUILD_UTILS=$(find . -path "./target/debug/build/rebaser-*/out/qt-build-utils/qml_modules" | head -n 1) &&
	qmllint -I ./build-dev/qml/ -I ${QT_BUILD_UTILS} qml/main.qml
