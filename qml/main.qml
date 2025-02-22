import QtWidgets as QW
import QtQml
import QtQuick // for QFont

import com.kdab.rebaser 1.0

QW.MainWindow {
    id: root

    windowTitle: "Rebaser"
    size: Qt.size(800, 700)
    visible: true

    Component.onCompleted: {
        console.log("Loaded main window");
    }

    QW.MenuBar {
        QW.Menu {
            title: "File"
            QW.Action {
                text: "Quit"
                onTriggered: {
                    Qt.quit();
                }
            }
        }
    }

    QW.SyntaxHighlighter {
        id: highlighter
        document: textEdit.document
        onHighlightRequested: text => {
            const keywords = ["pick", "update-ref", "label", "merge", "merge -C", "reset"];

            // Split into lines and check each line
            const lines = text.split('\n');

            let pos = 0;
            for (const line of lines) {
                const trimmed = line.trim();

                if (trimmed.startsWith('#')) {
                    setFormat(pos, line.length, "#998066");
                } else {
                    // Check for git SHA1
                    const match = line.match(/\b[0-9a-f]{7}\b/);
                    if (match) {
                        const lineStartIndex = line.indexOf(match[0]);
                        setFormat(pos + lineStartIndex, 7, "#81a2be");
                    }

                    const secondWordMatches = line.match(/^(reset|label|update-ref)\s+(\S+)/);
                    if (secondWordMatches) {
                        const targetWord = secondWordMatches[2];
                        const targetIndex = line.indexOf(targetWord);
                        setFormat(pos + targetIndex, targetWord.length, "#81a2be");
                    }

                    for (const keyword of keywords) {
                        if (trimmed.startsWith(keyword)) {
                            setFormat(pos + line.indexOf(keyword), keyword.length, "#8959a8");
                        }
                    }
                }

                pos += line.length + 1; // +1 for newline
            }
        }
    }

    QW.Widget {
        id: centralWidget
        QW.HBoxLayout {
            QW.TreeView {}
            QW.TextEdit {
                id: textEdit
                readOnly: true
                font.pixelSize: 14
                font.family: "FiraCode Nerd Font Mono"
                plainText: MyObject.filename
            }
        }
    }
}
