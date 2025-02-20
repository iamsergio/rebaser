import QtWidgets

MainWindow {
    id: root

    windowTitle: "Rebaser"
    size: Qt.size(800, 600)

    MenuBar {
        Menu {
            title: "File"
            Action {
                text: "Quit"
                onTriggered: {
                    Qt.quit();
                }
            }
        }
    }

    Widget {
        id: centralWidget
        HBoxLayout {
            TreeView {}
            TextEdit {}
        }
    }
}
