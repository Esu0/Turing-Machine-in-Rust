mod tape;
pub use self::tape::{Direction, Tape};
use crate::analyzer::{self, State, TokenGen};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct TuringMachine {
    initial_state: State,
    final_state: HashSet<State>,
    transfunc: HashMap<(State, Option<char>), (State, Option<char>, Direction)>,
}

impl TuringMachine {
    pub fn new(
        initial_state: State,
        final_state: HashSet<State>,
        transfunc: HashMap<(State, Option<char>), (State, Option<char>, Direction)>,
    ) -> Self {
        Self {
            initial_state,
            final_state,
            transfunc,
        }
    }

    pub fn is_accepted(&self, s: String) -> bool {
        let mut tape = Tape::from_string(s);
        let mut state = self.initial_state;
        while let Some((s, c, d)) = self.transfunc.get(&(state, tape.current())) {
            if self.final_state.contains(s) {
                return true;
            } else {
                state = *s;
                tape.next(*c, *d);
            }
        }
        false
    }

    pub fn from_file<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let mut generator = TokenGen::from_file(path);
        let (initial_state, final_state, transfunc) = analyzer::get_map(&mut generator);
        Self {
            initial_state,
            final_state,
            transfunc,
        }
    }
}
