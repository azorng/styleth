
pub struct Matcher {
    mode: Mode,
    input: &String
}

impl Matcher {
    fn new(mode: Mode, input: &String) -> Self {
        Matcher {
            mode, input
        }
    }

    fn is_match(&self, address: &String) -> bool {
        match self.mode {
            Mode::StartsWith => address[..self.input.len()] == self.input,
            Mode::Match => is_pattern_match(address, self.input),
            Mode::Leading => is_leading_match(address, self.input, score.get())
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

fn is_leading_match(address: &String, input_value: &String, score: usize) -> bool {
    address[..score] == (0..score).map(|_| input_value.as_str()).collect::<String>()
}

