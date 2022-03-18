use crate::file::position::InFilePosition;
use crate::file::File;

use super::lexer::*;
use super::scope::Scope;
use super::token_types::Token;

#[derive(Default)]
pub struct Parser {
    pub str: String,
    pub len: usize,
    pub base: usize,

    pub chr: char,
    pub char_offset: usize,
    pub offset: usize,

    pub idx: usize,

    pub token: Token,
    pub literal: String,

    pub scope: Box<Scope>,
    pub insert_semicolon: bool,
    pub implicit_semicolon: bool,

    pub errors: Vec<String>,

    pub recover: Recover,

    pub mode: Mode,

    pub opts: Option<Options>,

    pub file: Option<File>,
}

impl Parser {
    pub fn new(filename: &str, src: &str, base: usize, opts: Option<Options>) -> Self {
        Self {
            chr: ' ',
            str: String::from(src),
            len: src.len(),
            base,
            file: Some(File::new(filename, src, base, Vec::new(), 0)),
            opts,
            ..Default::default()
        }
    }

    pub fn error_unexpected_token(&self, value: &Token) {}

    pub fn skip_white_space(&self) {}
    pub fn scan_identifier(&mut self) -> Result<(String, String, bool), String> {
        unimplemented!();
    }
    pub fn scan(&mut self) -> (Token, String, String, usize) {
        self.implicit_semicolon = false;
        let mut tkn = Token::Illegal;
        let mut literal: String;
        let mut parsed_literal: String;
        let mut idx = 0;

        loop {
            self.skip_white_space();

            let idx = self.idx_of(self.char_offset);
            let insert_semicolon = false;

            match self.chr {
                chr if is_identifier(chr) => {
                    let mut has_escaped: bool;
                    let result = self.scan_identifier();
                    if !result.is_err() {
                        (literal, parsed_literal, has_escaped) = result.unwrap();
                        if parsed_literal.len() > 1 {
                            let mut strict: bool;
                            (tkn, strict) = is_keyword(parsed_literal);
                            if has_escaped {
                                self.insert_semicolon = true;
                                if tkn != 0 && tkn != Token::Let as usize
                                    || parsed_literal == "true"
                                    || parsed_literal == "false"
                                    || parsed_literal == "null"
                                {
                                    tkn = Token::Keyword;
                                } else {
                                    tkn = Token::Identifier;
                                }
                                return (tkn, literal, parsed_literal, idx);
                            }
                            match tkn {
                                Token::Illegal => {
                                    if parsed_literal == "true" || parsed_literal == "false" {
                                        self.insert_semicolon = true;
                                        tkn = Token::Boolean;
                                        return (tkn, literal, parsed_literal, idx);
                                    }
                                }
                                Token::Keyword => {
                                    if !strict {
                                        return (tkn, literal, parsed_literal, idx);
                                    }
                                }
                                Token::This
                                | Token::Break
                                | Token::Throw
                                | Token::Return
                                | Token::Continue
                                | Token::Debugger => {
                                    self.insert_semicolon = true;
                                }
                                _ => {}
                            }
                        }

                        self.insert_semicolon = true;
                        tkn = Token::Identifier;
                        return (tkn, literal, parsed_literal, idx);
                    }
                    tkn = Token::Illegal;
                }
                _ => {}
            }
        }
    }

    pub fn next(self) {
        todo!()
    }

    pub fn optional_semicolon(mut self) {
        if self.token == Token::Semicolon as usize {
            self.next();
            return;
        }
        if self.insert_semicolon {
            self.implicit_semicolon = false;
            return;
        }

        if self.token != Token::EOF as usize && self.token != Token::RightBrace as usize {
            self.expect(Token::Semicolon);
        }
    }

    pub fn semicolon(mut self) {
        match self.token {
            Token::RightParenthesis | Token::RightBracket => {}
            _ => {
                if self.implicit_semicolon {
                    self.insert_semicolon = false;
                    return;
                }
                self.expect(Token::Semicolon);
            }
        }
    }

    pub fn idx_of(&mut self, offset: usize) -> usize {
        self.base + offset
    }
    pub fn expect(self, value: Token) -> usize {
        let idx = self.idx;
        if self.token != value as usize {
            self.error_unexpected_token(&self.token)
        }
        self.next();
        idx
    }

    pub fn position(self, idx: usize) -> InFilePosition {
        let position = self.file.unwrap().postion(idx - self.base);
        position
    }
}

#[derive(Default)]
pub struct Options {}
#[derive(Default)]
pub struct Mode {}

#[derive(Default)]
pub struct Recover {
    pub idx: usize,

    pub count: usize,
}

pub fn is_keyword(token: String) -> (Token, bool) {
    unimplemented!();
}
