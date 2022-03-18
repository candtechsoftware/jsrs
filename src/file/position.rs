#[derive(Clone)]
pub struct InFilePosition {
    pub filename: String,
    pub line: usize,
    pub(crate) column: usize,
}

impl InFilePosition {
    pub fn new(filename: String, line: usize, column: usize) -> Self {
        Self {
            filename,
            line,
            column,
        }
    }

    #[inline]
    fn is_valid(&self) -> bool {
        self.line > 0
    }

    pub fn to_string(&self) -> String {
        let mut str = String::from(self.filename.clone());
        let is_valid = self.is_valid();
        if is_valid {
            if str.eq_ignore_ascii_case("") {
                str.push_str(":");
            }
            str.push_str(format!("{}:{}", &self.line, &self.column).as_str());
        }
        str
    }
}
