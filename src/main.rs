#[macro_use]
extern crate qmlrs;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

// Basic io Utils used
struct Utils;
impl Utils {
    // Open Rust files, and needed QML Files
    fn open_files(path: &str) -> File {
        match File::open(path) {
            Ok(file) => file,
            Err(_) => panic!("Unable to open file"),
        }
    }
    // Reads the lines of the file and puts it in a Vec<String>
    fn read_file(file: File) -> Vec<String> {
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect();
        let mut s_lines: Vec<String> = Vec::new();

        // NOTE: Is this best way?
        for line in lines {
            let s: String = line.unwrap().clone();
            s_lines.push(s);
        }

        s_lines
    }
}

// Main struct for all input files (Rust or QML)
struct InputFile {
    path: String,
    contents: Vec<String>,
}
impl InputFile {
    // Initiates new file to be opened and read from
    fn new(p: &str) -> InputFile {
        let file = Utils::open_files(p);
        InputFile {
            path: String::from(p),
            contents: Utils::read_file(file),
        }
    }
}


// Struct for the Rust input files
struct RustInFiles {
    file_data: InputFile,
    change_line: String,
    line_number: i32,
}
impl RustInFiles {
    // Initiates rust input files
    fn new(path: &str) -> RustInFiles {
        let file_data = InputFile::new(path);
        let (index, line) = RustInFiles::search(file_data.contents.clone());
        RustInFiles {
            file_data: file_data,
            change_line: line,
            line_number: index,
        }
    }

    // Searches rust files for `load_local_file` and sends out the index in the vec and the whole
    // line
    fn search(lines: Vec<String>) -> (i32, String) {
        let mut line: String = String::new();
        let mut index: i32 = 0;
        for (i, l) in lines.iter().enumerate() {
            if l.contains("load_local_file") {
                index = i as i32;
                line = l.clone();
            }
        }

        (index, line)
    }

    // Splits the searched dir to get the path to the QML file
    fn split_line(&self) -> String {
        let split = self.change_line.as_str().split("\"");
        let vec = split.collect::<Vec<&str>>();

        String::from(vec[1])
    }
}

// Struct for all QML files
struct QmlInFile {
    file_data: InputFile,
}
impl QmlInFile {
    // Initializes the QML file
    fn new(path: &str) -> QmlInFile {
        QmlInFile { file_data: InputFile::new(path) }
    }
}

// Struct for the output file
// TODO: Not actually output file, but compile from buffer if possible
struct RustOutFile {
    rust_files: RustInFiles,
    qml_files: QmlInFile,
    path: String,
}
impl RustOutFile {
    fn new(rust: RustInFiles, qml: QmlInFile, path: String) -> RustOutFile {
        RustOutFile {
            rust_files: rust,
            qml_files: qml,
            path: path,
        }
    }

    // Inputs QML String just above `load_local_file` then changes that line to load_data
    fn add_qml(&mut self) {
        let mut new_rust: Vec<String> = Vec::new();
        for (i, l) in self.rust_files.file_data.contents.iter().enumerate() {
            if self.rust_files.line_number > i as i32 {
                new_rust.push(l.clone());
                new_rust.push(String::from("\n"));
            } else if i as i32 == self.rust_files.line_number {
                new_rust.push(String::from("let mut qml_string: String = String::from(\" \\\n"));
                for ql in self.qml_files.file_data.contents.iter() {
                    let mut new_qml_string: String = String::new();
                    let qs: String = ql.clone();
                    let s: &str = qs.as_str();
                    if s.contains("\"") {
                        new_qml_string = String::from(s.replace("\"", "\\\""));
                    } else {
                        new_qml_string = ql.clone();
                    }
                    new_rust.push(new_qml_string);
                    new_rust.push(String::from(" \\\n"));

                }
                new_rust.push(String::from("\");"));
                new_rust.push(String::from("engine.load_data(&qml_string);"))
            } else if i as i32 > self.rust_files.line_number {
                new_rust.push(l.clone());
                new_rust.push(String::from("\n"));
            }
        }

        self.rust_files.file_data.contents = new_rust;
    }

    // write output file to disk
    // TODO: Maybe not needed
    fn write_rustfile(&mut self) {
        let rust_file: String = self.rust_files.file_data.contents.iter().cloned().collect();
        let path = Path::new(self.path.as_str());
        let display = path.display();

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't create {}: {}", display, why),
        };

        match file.write_all(rust_file.as_str().as_bytes()) {
            Ok(_) => println!("Succesfully wrote: {}", display),
            Err(why) => panic!("Could not write {}: {}", display, why),
        }
    }
}
fn main() {
    // TODO: get files either automagically or via cli opts.
    let rust_files = RustInFiles::new("markdown.rs");
    let qml_file = QmlInFile::new(rust_files.split_line().as_str());
    let mut out_file = RustOutFile::new(rust_files, qml_file, String::from("out_file.rs"));
    out_file.add_qml();
    out_file.write_rustfile();

    println!("{:?}", out_file.rust_files.file_data.contents);
}
