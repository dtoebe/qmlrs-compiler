#[macro_use]
extern crate qmlrs;
extern crate markdown;

use std::env;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use std::process::Command;

struct Markdown;
impl Markdown {
    fn sync(&self, s: String) -> String {
        markdown::to_html(&*s)
    }
    fn copy_to_clipboard(&self, s: String) {
        let html: String = markdown::to_html(&*s);
        let echo_cmd = Command::new("echo")
            .arg(&*html)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let xclip_cmd = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(mut stdout) = echo_cmd.stdout {
            if let Some(mut stdin) = xclip_cmd.stdin {
                let mut buf: Vec<u8> = Vec::new();
                stdout.read_to_end(&mut buf).unwrap();
                stdin.write_all(&buf).unwrap();
            }
        }

    }
}

Q_OBJECT! {Markdown:
    slot fn sync(String);
    slot fn copy_to_clipboard(String);
}

fn main() {
    let home = env::home_dir();
    let ui_path = Path::join(&home.unwrap(), ".config/rustdown/ui/main.qml");

    let mut engine = qmlrs::Engine::new();

    engine.set_property("markdown", Markdown);

let mut qml_string: String = String::from(" \
import QtQuick 2.3 \
import QtQuick.Controls 1.4 \
import QtQuick.Layouts 1.3 \
 \
ApplicationWindow { \
    width: 640 \
    height: 480 \
    color: \"#f1f1f1\" \
    visible: true \
    title: \"RustDown\" \
 \
    toolBar: ToolBar { \
        width: parent.width \
 \
        RowLayout { \
            width: parent.widtht \
            height: parent.height \
 \
            Button { \
                Layout.alignment: Qt.AlignRight \
                text: \"Copy To HTML\" \
 \
                onClicked: markdown.copy_to_clipboard(mdarea.text); \
            } \
        } \
    } \
 \
    RowLayout { \
        width: parent.width \
        height: parent.height \
 \
        TextArea { \
            id: mdarea \
            Layout.alignment: Qt.AlignCenter \
            Layout.preferredWidth: (parent.width / 2) - 2 \
            Layout.preferredHeight: parent.height - 5 \
            text: \"Markdown\" \
 \
            Keys.onReleased: rtarea.text = markdown.sync(mdarea.text); \
 \
        } \
 \
        TextArea { \
            id: rtarea \
            Layout.alignment: Qt.AlignCenter \
            Layout.preferredWidth: (parent.width / 2) - 2 \
            Layout.preferredHeight: parent.height - 5 \
            textFormat: TextEdit.RichText \
            text: \"Rich Text\" \
 \
            onActiveFocusChanged: { \
                if(!activeFocus) { \
                    rtarea.textFormat = TextEdit.RichText; \
                } else { \
                    rtarea.textFormat = TextEdit.PlainText; \
                    rtarea.text = markdown.sync(mdarea.text); \
                } \
            } \
            Component.onCompleted: rtarea.text = markdown.sync(mdarea.text); \
        } \
    } \
} \
");engine.load_data(&qml_string);
    engine.exec();
}