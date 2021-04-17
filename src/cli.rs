use structopt::StructOpt;

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
