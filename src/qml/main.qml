import QtWidgets
import QtQml
import QtQuick as QQ // for QFont

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
            TextEdit {
                id: textEdit
                font.pixelSize: 14
                font.family: "FiraCode Nerd Font Mono"
                plainText: "label onto

# Branch cmake
reset onto
pick 1978840 build: Port to CMake
pick dc6a141 build: Add a CMakePresets.json file
pick 3773644 tests: stabilize
pick 35ad4f5 tests: blacklist tst_formlayout
pick 4f6b627 ci: Add build.yml
update-ref refs/heads/cmake

label cmake

# Branch qt6
reset cmake # ci: Add build.yml
pick e06159d fix(qt6): Added missing QWidget and QTextDocument include
pick 434e91b fix(qt6): Fix build error assigning 0 to Qt::Alignment
pick b057bc9 fix(qt6): Don't use QTextCodec, it's UTF-8 by default
pick ceac1b1 fix(qt6): Use Qt::endl instead of endl
pick 5322e6a fix(qt6): Use correct integer type in Qt6
pick 668bde8 fix(qt6): Add missing includes
pick b8deca2 fix(qt6): Use qmlRegisterAnonymousType
pick a572457 fix(qt6): Don't register QSizePolicy
update-ref refs/heads/qt6

label qt6

# Branch margin
reset qt6 # fix(qt6): Don't register QSizePolicy
pick f23d090 fix(qt6): Add 'margin' property to layouts for compat with Qt 5
update-ref refs/heads/margin

label margin

reset onto
pick 4358f49 personal: Add my vscode workspace
merge -C 83636d6 cmake # Merge branch 'cmake'
merge -C df98d3c qt6 # Merge branch 'qt6'
merge -C 17c09fc margin # Merge branch 'margin'
pick 8d3a2f7 feat: Add support for qmlls language server completiona"

                SyntaxHighlighter {
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
            }
        }
    }
}
