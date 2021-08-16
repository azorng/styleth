use crate::cli::Mode;
use atomic_counter::{AtomicCounter, RelaxedCounter};

pub struct Matcher {
    mode: Mode
}

impl Matcher {
    pub fn new(mode: Mode) -> Self {
        Matcher { mode }
    }

    pub fn is_match(&self, address: &String) -> bool {
        match &self.mode {
            Mode::StartsWith(input) => &address[..input.len()] == input,
            Mode::Match(pattern) => is_pattern_match(address, &pattern),
            Mode::Leading(input, score) => incremental_char_match(address, &score, |val| val == *input),
            Mode::NumbersOnly(score) => incremental_char_match(address, &score, |val| val.is_numeric()),
            Mode::SpecificChars(input, score) => incremental_char_match(address, &score, |val| input.chars().any(|c| c == val))
        }
    }
}

fn is_pattern_match(address: &String, pattern: &String) -> bool {
    let mut pattern_chars = pattern.chars();
    !address.chars().any(|c| {
        let pattern_char = pattern_chars.next().unwrap();
        pattern_char != c && pattern_char != 'X'
    })
}

fn incremental_char_match<F>(address: &String, score: &RelaxedCounter, f: F) -> bool 
where F: Fn(char) -> bool {
    let mut is_match = false;
    for (i, char_val) in address.chars().enumerate() {
        if f(char_val) {
            if i >= score.get() {
                score.inc();
                is_match = true;
            }
            if i + 1 == address.len() {
                is_match = true;
            }
        } else {
            break;
        }
    }
    if is_match {
        println!("Score: {}", score.get());
    }
    is_match
}
