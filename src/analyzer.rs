pub mod tokenize;
use crate::machine::Direction;
use std::collections::{HashMap, HashSet};
use tokenize::Token;
pub use tokenize::TokenGen;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(u32);

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub fn get_map(
    generator: &mut TokenGen,
) -> (
    State,
    HashSet<State>,
    HashMap<(State, Option<char>), (State, Option<char>, Direction)>,
) {
    let mut statelist = HashMap::new();
    let mut func = HashMap::new();
    let mut snum = 0;
    let initial;
    let mut finalstate = HashSet::new();
    if let Token::Str(s) = generator.current() {
        initial = State(0);
        snum += 1;
        statelist.insert(s.clone(), initial);
    } else {
        panic!("syntax error");
    }
    generator.next();
    generator.next();
    loop {
        match generator.current() {
            Token::Eof => {
                panic!("error");
            }
            Token::Str(s) => {
                if s == &vec!['}'] {
                    break;
                }
                let st;
                if statelist.contains_key(s) {
                    st = statelist[s];
                } else {
                    st = State(snum);
                    statelist.insert(s.clone(), st);
                    snum += 1;
                }
                finalstate.insert(st);
            }
        };
        generator.next();
    }
    generator.next();
    while let Token::Str(s) = generator.current() {
        let state1;
        if !statelist.contains_key(s) {
            state1 = State(snum);
            statelist.insert(s.clone(), state1);
            snum += 1;
        } else {
            state1 = statelist[s];
        }
        generator.next();
        let ci;
        if let Token::Str(s) = generator.current() {
            if s[0] == 'B' {
                ci = None;
            } else {
                ci = Some(s[0]);
            }
        } else {
            panic!("syntax error");
        }
        generator.next();
        generator.next();

        let state2;
        if let Token::Str(s) = generator.current() {
            if !statelist.contains_key(s) {
                state2 = State(snum);
                statelist.insert(s.clone(), state2);
                snum += 1;
            } else {
                state2 = statelist[s];
            }
        } else {
            panic!("syntax error");
        }
        generator.next();
        let co;
        if let Token::Str(s) = generator.current() {
            if s[0] == 'B' {
                co = None;
            } else {
                co = Some(s[0]);
            }
        } else {
            panic!("syntax error");
        }
        generator.next();
        let d;
        if let Token::Str(s) = generator.current() {
            if s[0] == 'r' || s[0] == 'R' {
                d = Direction::Right;
            } else if s[0] == 'l' || s[0] == 'L' {
                d = Direction::Left;
            } else {
                panic!("syntax error");
            }
        } else {
            panic!("syntax error");
        }

        func.insert((state1, ci), (state2, co, d));
        generator.next();
    }
    (initial, finalstate, func)
}
