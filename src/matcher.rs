use crate::cli::Mode;
use atomic_counter::{AtomicCounter, RelaxedCounter};

pub struct Matcher {
    mode: Mode,
    pub score: RelaxedCounter
}

impl Matcher {
    pub fn new(mode: Mode) -> Self {
        Matcher { mode, score: RelaxedCounter::new(0) }
    }

    pub fn is_match(&self, address: &String) -> bool {
        match &self.mode {
            Mode::StartsWith(input) => &address[..input.len()] == input,
            Mode::Match(pattern) => self.is_pattern_match(address, &pattern),
            Mode::Leading(input) => self.incremental_char_match(address, |c| c == *input),
            Mode::NumbersOnly => self.incremental_char_match(address, |c| c.is_numeric()),
            Mode::SpecificChars(input) => self.incremental_char_match(address, |c| input.chars().any(|input_c| input_c == c))
        }
    }

    fn is_pattern_match(&self, address: &String, pattern: &String) -> bool {
        let mut pattern_chars = pattern.chars();
        !address.chars().any(|c| {
            let pattern_char = pattern_chars.next().unwrap();
            pattern_char != c && pattern_char != 'X'
        })
    }

    fn incremental_char_match<F>(&self, address: &String, f: F) -> bool 
        where F: Fn(char) -> bool {
            let mut is_match = false;
            for (i, char_val) in address.chars().enumerate() {
                if f(char_val) {
                    if i >= self.score.get() {
                        self.score.inc();
                        is_match = true;
                    }
                    if i + 1 == address.len() {
                        is_match = true;
                    }
                } else {
                    break;
                }
            }
            is_match
        }
}
