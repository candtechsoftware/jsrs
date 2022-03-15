use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Mapping {
    pub gen_line: i32,
    pub gen_col: i32,
    pub sources_ind: i32,
    pub names_ind: i32,
    pub source_line: i32,
    pub source_column: i32,
}
