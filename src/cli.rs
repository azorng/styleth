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
    StartsWith,
    Match
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Cli {
    /// Matches on addresses that starts with given chars. Example: dead69
    #[structopt(long)]
    pub starts_with: Option<String>,

    /// Matches on a given pattern where X equals any char. Example: deadXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX69
    #[structopt(long="match")]
    pub match_value: Option<String>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::from_args()
    }

    pub fn get_mode(&self) -> (Mode, String) {
        if self.starts_with.is_some() {
            let val = self.starts_with.as_ref().unwrap();
            validate_hex(val);
            return (Mode::StartsWith, String::from(val))
        }

        if self.match_value.is_some() {
            let val = self.match_value.as_ref().unwrap();
            let pattern = validate_and_format_pattern(val);

            (Mode::Match, pattern)
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

fn validate_and_format_pattern(pattern: &String) -> String {
    let pattern_length = 40;

    if pattern.len() != pattern_length {
        exit_with_err(format!("Pattern length must be {} chars.", pattern_length));
    }

    let re = Regex::new(r"^[a-fA-F0-9X]*$").unwrap();
    if !re.is_match(pattern) {
        exit_with_err(format!("Invalid pattern syntax: {}. For more information try --help.", pattern));
    }

    pattern.replace("X", ".")
}
