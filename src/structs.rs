use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interface {
    Files,
    Editor,
    Clipboard // to do.
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Insert,
    Movement,
    Command
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub is_folder: bool
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub file: usize
}

impl Cursor {
    pub fn default() -> Cursor {
        Cursor { x: 0, y: 0, z: 0, file: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Editor {
    pub interface: Interface,
    pub cursor: Cursor,
    pub files: Vec<File>,
    pub stopped: bool,
    pub before_mode: Mode,
    pub mode: Mode,
    pub file_tmp: String,
    pub command: String,
    pub output: String,
    pub file_lines: usize,
    pub file_lines_vec: Vec<String>
}

impl Editor {
    pub fn default() -> Editor {
        Editor {
            interface: Interface::Files,
            cursor: Cursor::default(),
            files: Vec::new(),
            stopped: false,
            mode: Mode::Movement,
            before_mode: Mode::Movement,
            file_tmp: String::new(),
            command: String::new(),
            output: String::from("Command output"),
            file_lines: 0,
            file_lines_vec: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyEvents {
    pub code: KeyCode,
    pub modifiers: KeyModifiers
}