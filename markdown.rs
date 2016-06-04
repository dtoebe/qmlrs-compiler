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

    engine.load_local_file("/home/dtoebe/.config/rustdown/ui/main.qml");

    engine.exec();
}
