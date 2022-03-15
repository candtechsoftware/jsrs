use super::utils::sourcemap::consumer::Consumer;
use std::sync::Mutex;
use url::{ParseError, Url};

pub mod position;

use position::Postion;

pub struct File<'a> {
    mu: Mutex<()>,

    pub name: &'a str,
    pub src: &'a str,
    pub base: usize,
    pub source_map: Option<Consumer<'a>>,
    pub line_offsets: Vec<i32>,
    pub last_scanned_offset: i32,
}

impl<'a> File<'a> {
    pub fn new(
        name: &'a str,
        src: &'a str,
        base: usize,
        line_offsets: Vec<i32>,
        last_scanned_offset: i32,
    ) -> Self {
        Self {
            mu: Mutex::new(()),
            name,
            src,
            base,
            source_map: None,
            line_offsets,
            last_scanned_offset,
        }
    }

    pub fn set_sourcemap(&mut self, map: Consumer<'a>) {
        self.source_map = Some(map);
    }

    pub fn postion(&mut self, offset: i32) -> Postion {
        let mut line: i32;
        let mut lines_offsets: Vec<i32> = vec![];

        let gaurd = self.mu.lock().unwrap();

        if offset > self.last_scanned_offset {
            line = self.scan_to(offset);
            lines_offsets = self.line_offsets.clone();
            drop(gaurd);
        } else {
            lines_offsets = self.line_offsets.clone();
            drop(gaurd);
            lines_offsets.sort();
            line = lines_offsets.iter().find(|&item| item > &offset).unwrap() - 1 as i32;
        }

        let mut line_start = 0;
        if line >= 0 {
            line_start = lines_offsets[line as usize];
        }

        let row = line + 2;
        let col = offset - line_start + 1;

        if let Some(mp) = &mut self.source_map {
            match mp.source(row, col) {
                Some((source, _, row2, col2, ok)) => {
                    if ok {
                        return Postion {
                            filename: resolve_sourcemap_url(
                                self.name.to_string(),
                                source.to_string(),
                            ),
                            line: row2,
                            column: col2,
                        };
                    }
                }
                None => (),
            }
        }

        Postion::new(self.name.to_string(), row, col)
    }

    fn scan_to(&self, offset: i32) -> i32 {
        0 as i32
    }
}

// TODO: Need to optiomize this or find a library that will do similar work
fn resolve_sourcemap_url<'a>(base_name: String, source: String) -> String {
    let url_res = Url::parse(source.as_str());
    // if url is absolut and valid just return it
    if url_res.is_ok() {
        return url_res.unwrap().to_string();
    }

    if let Err(err) = url_res {
        if err == ParseError::RelativeUrlWithoutBase {
            let base_url = Url::parse(base_name.as_str());
            if let Err(err) = base_url {
                if err == ParseError::RelativeUrlWithoutBase {
                    let mut base_vec: Vec<&str> = base_name.split("/").collect();
                    base_vec.pop();
                    let base = base_vec.join("/");
                    let base_str = base.as_str();
                    let source_str = source.as_str();
                    let base_path = std::path::Path::new(&base_str);
                    let source_path = std::path::Path::new(&source_str);
                    let joined_path = base_path.join(source_path);
                    let path = joined_path.to_str().unwrap();
                    return Url::parse(path).unwrap().to_string();
                }
                let base_path = std::path::Path::new(base_name.as_str());
                let source_path = std::path::Path::new(source.as_str());
                let joined_path = base_path.join(source_path);
                let path = joined_path.to_str().unwrap();
                return Url::parse(path).unwrap().to_string();
            }
        }
    };
    "".to_string()
}
