use crate::cli::Mode;
use atomic_counter::{AtomicCounter, RelaxedCounter};
use regex::Regex;

pub struct Matcher {
    mode: Mode,
    pub score: RelaxedCounter,
}

impl Matcher {
    pub fn new(mode: Mode) -> Self {
        Matcher {
            mode,
            score: RelaxedCounter::new(0),
        }
    }

    pub fn is_match(&self, address: &String) -> bool {
        match &self.mode {
            Mode::StartsWith(input) => &address[..input.len()] == input,
            Mode::Leading(input) => self.incremental_char_match(address, |c| c == *input),
            Mode::NumbersOnly => self.incremental_char_match(address, |c| c.is_numeric()),
            Mode::SpecificChars(input) => {
                self.incremental_char_match(address, |c| input.chars().any(|input_c| input_c == c))
            }
            Mode::Regex(regex) => Regex::new(regex).unwrap().is_match(&address),
        }
    }

    fn incremental_char_match<F>(&self, address: &String, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_matches() {
        let mode = Mode::Leading('f');
        let addr = "fffff1fd4aa1a3dcb830085e99a396d7f3796f62".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(true, matcher.is_match(&addr));
        assert_eq!(5, matcher.score.get());
    }

    #[test]
    fn leading_nomatch() {
        let mode = Mode::Leading('f');
        let addr = "8ffff1fd4aa1a3dcb830085e99a396d7f3796f62".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(false, matcher.is_match(&addr));
        assert_eq!(0, matcher.score.get());
    }

    #[test]
    fn starts_with_matches() {
        let mode = Mode::StartsWith("dead".to_string());
        let addr = "deadaae3aa608bcc91bf997a0ae1e45ac6a23369".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(true, matcher.is_match(&addr));
    }

    #[test]
    fn starts_with_nomatch() {
        let mode = Mode::StartsWith("dead".to_string());
        let addr = "deidaae3aa608bcc91bf997a0ae1e45ac6a23369".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(false, matcher.is_match(&addr));
    }

    #[test]
    fn numbers_only_matches() {
        let mode = Mode::NumbersOnly;
        let addr = "9179761e0283eb801dacce57538d94a98234017d".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(true, matcher.is_match(&addr));
        assert_eq!(7, matcher.score.get());
    }

    #[test]
    fn numbers_only_nomatch() {
        let mode = Mode::NumbersOnly;
        let addr = "a179761e0283eb801dacce57538d94a98234017d".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(false, matcher.is_match(&addr));
        assert_eq!(0, matcher.score.get());
    }

    #[test]
    fn specific_chars_matches() {
        let mode = Mode::SpecificChars("abcde".to_string());
        let addr = "caabedda6a5e70213a62f922958cbd307dd56968".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(true, matcher.is_match(&addr));
        assert_eq!(8, matcher.score.get());
    }

    #[test]
    fn specific_chars_nomatch() {
        let mode = Mode::SpecificChars("1bcde".to_string());
        let addr = "91abedda6a5e70213a62f922958cbd307dd56968".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(false, matcher.is_match(&addr));
        assert_eq!(0, matcher.score.get());
    }

    #[test]
    fn regex_match() {
        let mode = Mode::Regex("^dead.*0dead$".to_string());
        let addr = "deadedda6a5e70213a62f922958cbd307dd0dead".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(true, matcher.is_match(&addr));
    }

    #[test]
    fn regex_nomatch() {
        let mode = Mode::Regex("^dead.*0dead$".to_string());
        let addr = "deadedda6a5e70213a62f922958cbd307dddead0".to_string();
        let matcher = Matcher::new(mode);
        assert_eq!(false, matcher.is_match(&addr));
    }
}
