use super::mappings::Mapping;
use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Result};

#[derive(Serialize, Deserialize, Clone)]
struct SourceMap<'a> {
    mappings: Vec<Mapping>,
    sources: Vec<String>,

    #[serde(borrow)]
    names: Vec<&'a RawValue>,
}

impl<'a> SourceMap<'a> {
    pub fn names(&mut self, index: usize) -> Result<String> {
        if index >= self.names.len() {
            return Ok(String::from(""));
        }

        let raw = self.names[index];

        serde_json::to_string(&raw)
    }
}

#[derive(Serialize, Deserialize)]
struct Offset {
    pub line: i32,
    pub column: i32,
}
struct Section<'a> {
    pub offset: Offset,
    pub map: SourceMap<'a>,
}

pub struct Consumer<'a> {
    pub sourcemap_url: &'a str,
    pub file: &'a str,
    sections: Vec<Section<'a>>,
}

pub struct SourceRet<'a> {
    source: &'a str,
    name: &'a str,
    line: i32,
    column: i32,
    pub ok: bool,
}

impl<'a> Consumer<'a> {
    pub fn source(
        &mut self,
        gen_line: i32,
        gen_col: i32,
    ) -> Option<(String, String, i32, i32, bool)> {
        for s in self.sections.iter_mut() {
            if s.offset.line < gen_line
                || (s.offset.line + 1 == gen_line && s.offset.column <= gen_col)
            {
                let gen_line = gen_line - s.offset.line;
                let gen_col = gen_col - s.offset.column;
                let map = &mut s.map;
                let i = 0;
                let mut matched_mapping: Mapping;
                if i == map.mappings.len() {
                    matched_mapping = map.mappings[i - 1 as usize].clone();
                    if matched_mapping.gen_line != gen_line {
                        return None;
                    }
                } else {
                    matched_mapping = map.mappings[i as usize].clone();

                    if (matched_mapping.gen_line) > gen_line || matched_mapping.gen_col > gen_col {
                        if i == 0 {
                            return None;
                        }

                        matched_mapping = map.mappings[i - 1 as usize].clone();
                    }
                }

                let mut source = String::default();
                if matched_mapping.sources_ind >= 0 {
                    source = map.sources[matched_mapping.sources_ind as usize].clone();
                }

                let mut name = String::default();
                if matched_mapping.names_ind >= 0 {
                    name = map.names(matched_mapping.names_ind as usize).unwrap();
                }
                let line = matched_mapping.source_line;
                let column = matched_mapping.source_column;
                let ok = true;

                return Some((source, name, line, column, ok));
            }
        }

        None
    }
}
