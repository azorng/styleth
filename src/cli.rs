use structopt::StructOpt;
use indicatif::{ProgressBar as IndicatifBar, ProgressStyle};
use std::process;
use regex::Regex;

pub struct ProgressBar {
    bar: IndicatifBar
}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        let progress_bar = IndicatifBar::new_spinner();
        progress_bar.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}] {pos} attempts"));
        progress_bar.set_draw_delta(100);

        ProgressBar { bar: progress_bar }
    }

    pub fn tick(&self) {
        self.bar.inc(1);
    }
}

pub enum Mode {
    StartsWith(String),
    Match(String),
    Leading(char),
    NumbersOnly,
    SpecificChars(String)
}

#[derive(StructOpt)]
#[structopt(verbatim_doc_comment, rename_all = "kebab-case")]
pub struct Cli {
    /// Matches on addresses that starts with given chars. 
    /// Example: ./styleth --starts-with dead69
    #[structopt(verbatim_doc_comment, name = "hex text", short="s", long="starts-with")]
    pub starts_with: Option<String>,

    /// Matches on a given pattern where X equals any char.
    /// Example: ./styleth --match deadXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX69
    #[structopt(verbatim_doc_comment, name="pattern", short="m", long="match")]
    pub match_value: Option<String>,

    /// Takes a single char as input and performs an incremental matching. 
    /// Example: ./styleth --leading 0
    #[structopt(verbatim_doc_comment, name = "hex char", short="l", long="leading")]
    pub leading: Option<char>,

    /// Matches on random numbers.
    /// Example: ./styleth --random-numbers
    #[structopt(verbatim_doc_comment, short="n", long="numbers-only")]
    pub numbers_only: bool,

    /// Matches on specific hex chars without any particular order.
    /// Example: ./styleth --specific-chars abc123
    #[structopt(verbatim_doc_comment, short="c", long="specific-chars")]
    pub specific_chars: Option<String>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::from_args()
    }

    pub fn get_mode(&self) -> Mode {
        if self.starts_with.is_some() {
            let val = self.starts_with.as_ref().unwrap();
            validate_hex(val);
            return Mode::StartsWith(String::from(val));
        }

        if self.match_value.is_some() {
            let pattern = self.match_value.as_ref().unwrap();
            validate_and_format_pattern(pattern);
            return Mode::Match(String::from(pattern));
        }

        if self.leading.is_some() {
            let val = self.leading.as_ref().unwrap().to_string();
            validate_hex(&val);
            return Mode::Leading(self.leading.unwrap());
        }

        if self.numbers_only {
            return Mode::NumbersOnly;
        }

        if self.specific_chars.is_some() {
            let val = self.specific_chars.as_ref().unwrap().to_string();
            validate_hex(&val);
            return Mode::SpecificChars(String::from(val));
        }

        else { 
            exit_with_err("Select a valid option. For more information try --help.".to_string());
            panic!();
        }
    }

}

fn exit_with_err(msg: String) {
    println!("{}", msg);
    process::exit(1)
}

fn validate_hex(s: &String) {
    let re = Regex::new(r"^[a-fA-F0-9]*$").unwrap();
    if !re.is_match(s) {
        exit_with_err(format!("The value {} is not a valid hex.", s));
    }
}

fn validate_and_format_pattern(pattern: &String) {
    let pattern_length = 40;

    if pattern.len() != pattern_length {
        exit_with_err(format!("Pattern length must be {} chars.", pattern_length));
    }

    let re = Regex::new(r"^[a-fA-F0-9X]*$").unwrap();
    if !re.is_match(pattern) {
        exit_with_err(format!("Invalid pattern syntax: {}. For more information try --help.", pattern));
    }
}
