import QtQuick 2.3;
import QtQuick.Controls 1.4;
import QtQuick.Layouts 1.2;

ApplicationWindow {
    width: 640
    height: 480
    color: "#f1f1f1"
    visible: true
    title: "RustDown"

    toolBar: ToolBar {
        width: parent.width;

        RowLayout {
            width: parent.widtht;
            height: parent.height;

            Button {
                Layout.alignment: Qt.AlignRight;
                text: "Copy To HTML";

                onClicked: markdown.copy_to_clipboard(mdarea.text);
            }
        }
    }

    RowLayout {
        width: parent.width;
        height: parent.height;

        TextArea {
            id: mdarea;
            Layout.alignment: Qt.AlignCenter;
            Layout.preferredWidth: (parent.width / 2) - 2;
            Layout.preferredHeight: parent.height - 5;
            text: "Markdown";

            Keys.onReleased: rtarea.text = markdown.sync(mdarea.text);

        }

        TextArea {
            id: rtarea;
            Layout.alignment: Qt.AlignCenter;
            Layout.preferredWidth: (parent.width / 2) - 2;
            Layout.preferredHeight: parent.height - 5;
            textFormat: TextEdit.RichText;
            text: "Rich Text";

            onActiveFocusChanged: {
                if(!activeFocus) {
                    rtarea.textFormat = TextEdit.RichText;
                } else {
                    rtarea.textFormat = TextEdit.PlainText;
                    rtarea.text = markdown.sync(mdarea.text);
                }
            }
            Component.onCompleted: rtarea.text = markdown.sync(mdarea.text);
        }
    }
}
