use std::collections::HashMap;
use std::io::Read;

#[derive(PartialEq, Eq)]
pub enum Token {
    Str(Vec<char>),
    Eof,
}

pub struct TokenGen {
    code: Vec<char>,
    current: Token,
    index: usize,
    prev: usize,
}

impl Token {
    fn from_slc(slc: &[char]) -> Self {
        Self::Str(slc.to_vec())
    }

    fn from_char(c: char) -> Self {
        Self::Str(vec![c])
    }
}

impl TokenGen {
    pub fn from_file<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let mut tmp = Self {
            code: std::fs::read_to_string(path).unwrap().chars().collect(),
            current: Token::Eof,
            index: 0,
            prev: 0,
        };
        tmp.next();
        tmp
    }

    pub fn next(&mut self) {
        let l = self.code.len();
        let mut i = self.index;
        loop {
            if i >= l {
                self.current = Token::Eof;
                return;
            }
            if is_space(self.code[i]) {
                i += 1;
            } else {
                break;
            }
        }
        self.prev = i;
        self.index = i;
        if is_mark_1(self.code[i]) {
            self.current = Token::from_char(self.code[i]);
            i += 1;
            self.index = i;
            ()
        } else if is_ident(self.code[i]) {
            i += 1;
            while i < l && (is_ident(self.code[i])) {
                i += 1;
            }
            self.current = Token::from_slc(&self.code[self.index..i]);
            self.index = i;
            ()
        } else if is_mark(self.code[i]) {
            i += 1;
            while i < l && (is_mark(self.code[i])) {
                i += 1;
            }
            self.current = Token::from_slc(&self.code[self.index..i]);
            self.index = i;
            ()
        } else {
            panic!("構文エラー");
        }
    }

    pub fn current(&self) -> &Token {
        &self.current
    }
}

fn is_space(ch: char) -> bool {
    ch.is_whitespace()
}

fn is_ident(ch: char) -> bool {
    const MARK: &[char] = &[
        '~', '|', '-', '^', '\\', '!', '"', '#', '$', '%', '&', '\'', '`', '@', '*', ':', '/', '+',
        '<', '.', '?', ';', '(', ')', '[', ']',
    ];
    ch.is_ascii_alphanumeric() || ch == '_' || MARK.contains(&ch)
}

fn is_mark(ch: char) -> bool {
    const MARK: &[char] = &['=', '>'];
    MARK.contains(&ch)
}

fn is_mark_1(ch: char) -> bool {
    const MARK1: &[char] = &[',', '{', '}'];
    MARK1.contains(&ch)
}
