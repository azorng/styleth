use structopt::StructOpt;
use indicatif::{ProgressBar as IndicatifBar, ProgressStyle};

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

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Cli {
    #[structopt(long)]
    pub starts_with: String,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::from_args()
    }
}

