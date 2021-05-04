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
            Mode::Leading(input, score) => is_leading_match(address, &input, &score)
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

fn is_leading_match(address: &String, input_value: &String, score: &RelaxedCounter) -> bool {
    let score_val = score.get();
    let incremental_leading = (0..score_val).map(|_| input_value.as_str()).collect::<String>();
    let is_match = address[..score_val] == incremental_leading;
    if is_match {
        println!("Score: {}", score_val);
        score.inc();
    }
    is_match
}

