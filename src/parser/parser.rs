use super::scope::Scope;
use super::token_types::Token;

struct Parser {
    str: String,
    len: usize,
    base: usize,

    chr: char,
    char_offset: usize,
    offset: usize,

    idx: usize,

    token: Token,
    literal: String,

    scope: Box<Scope>,
    insert_semicolon: bool,
    implicit_semicolon: bool,

    errors: Vec<String>,

    recover: Recover,

    mode: Mode,

    opts: Options,

    file: Box<std::fs::File>,
}

struct Options {}
struct Mode {}

struct Recover {
    idx: usize,
    count: usize,
}
