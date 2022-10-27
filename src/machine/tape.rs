use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    Left,
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => "l",
                Direction::Right => "r",
            }
        )
    }
}
pub struct Tape {
    tape: VecDeque<Option<char>>,
    indexer: usize,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            tape: VecDeque::new(),
            indexer: 0,
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            tape: s.chars().map(|c| Some(c)).collect(),
            indexer: 0,
        }
    }

    pub fn from_string(s: String) -> Self {
        Self {
            tape: s.chars().map(|c| Some(c)).collect(),
            indexer: 0,
        }
    }
    pub fn current(&self) -> Option<char> {
        self.tape[self.indexer]
    }

    pub fn next(&mut self, c: Option<char>, d: Direction) {
        self.tape[self.indexer] = c;
        match d {
            Direction::Left => {
                if self.indexer == 0 {
                    self.tape.push_front(None);
                } else {
                    self.indexer -= 1;
                }
            }
            Direction::Right => {
                self.indexer += 1;
                if self.indexer >= self.tape.len() {
                    self.tape.push_back(None);
                }
            }
        }
    }
}
