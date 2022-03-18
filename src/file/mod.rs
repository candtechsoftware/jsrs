pub mod position;

use position::InFilePosition;
use sourcemap::SourceMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct FileSet {
    pub files: Vec<File>,
    pub last: Option<File>,
}

impl FileSet {
    pub fn new() -> Self {
        Self {
            files: Vec::default(),
            last: None,
        }
    }

    pub fn add_file(&mut self, filename: &str, src: String) -> usize {
        let base = self.next_base();
        let file = File {
            name: String::from(filename),
            src,
            base,
            source_map: None,
            line_offsets: Arc::new(Mutex::new(Vec::new())),
            last_scanned_offset: 0,
        };
        self.files.push(file.clone());
        self.last = Some(file);
        base
    }

    fn next_base(&self) -> usize {
        match &self.last {
            None => return 1,
            Some(last) => return last.base + last.src.len() + 1,
        }
    }

    fn file(self, idx: usize) -> Option<File> {
        for file in self.files {
            if idx <= file.base + file.src.len() {
                return Some(file);
            }
        }
        None
    }
    fn postion(&mut self, idx: usize) -> Option<InFilePosition> {
        for file in &mut self.files {
            if idx <= file.base + file.src.len() {
                let pos = file.postion(idx - file.base).clone();
                return Some(pos);
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub src: String,
    pub base: usize,
    pub source_map: Option<SourceMap>,
    pub line_offsets: Arc<Mutex<Vec<usize>>>,
    pub last_scanned_offset: i32,
}

impl<'a> File {
    pub fn new(
        name: &'a str,
        src: &'a str,
        base: usize,
        line_offsets: Vec<usize>,
        last_scanned_offset: i32,
    ) -> Self {
        Self {
            name: String::from(name),
            src: String::from(src),
            base,
            source_map: None,
            line_offsets: Arc::new(Mutex::new(line_offsets)),
            last_scanned_offset,
        }
    }

    pub fn set_sourcemap(&mut self, map: Option<SourceMap>) {
        self.source_map = map;
    }

    pub fn postion(&mut self, offset: usize) -> InFilePosition {
        let mut line: usize;
        if offset > self.last_scanned_offset as usize {
            line = self.scan_to(offset);
        } else {
            line = 10; // to implement;
        }

        let mut line_start = 0;
        if line >= 0 {
            line_start = self.line_offsets.lock().unwrap()[line];
        }

        let row = line + 2;
        let col = offset - line_start + 1;

        match &mut self.source_map {
            Some(map) => {
                if let Ok((source, line, column)) = get_orginal_source(map, row, col) {
                    return InFilePosition {
                        filename: resolve_file_name(self.name.clone(), source),
                        line,
                        column,
                    };
                }
            }
            None => {}
        }

        InFilePosition {
            filename: self.name.clone(),
            line: row,
            column: col,
        }
    }

    fn scan_to(&self, offset: usize) -> usize {
        0 as usize
    }
}

fn get_orginal_source(
    map: &mut SourceMap,
    row: usize,
    col: usize,
) -> Result<(String, usize, usize), String> {
    Err(String::from("Not implemtneted"))
}
fn resolve_file_name(name: String, source: String) -> String {
    String::from("not implemtneted")
}
