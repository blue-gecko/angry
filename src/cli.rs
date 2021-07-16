use {
    crate::convert::{random::RandomConvertor, simple::SimpleConvertor, Convertor},
    anyhow::{Context, Error, Result},
    std::path::PathBuf,
    structopt::{clap::ArgGroup, StructOpt},
};

pub fn from_args() -> Cli {
    Cli::from_args()
}

/// Take the passed text and make it aNgrY by applying random capitalisation.
#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("action").required(true))]
pub struct Cli {
    /// The path to the input file to read, optional
    #[structopt(parse(from_os_str), short, long)]
    input: Option<PathBuf>,

    /// The path to the output file to write, optional
    #[structopt(parse(from_os_str), short, long)]
    output: Option<PathBuf>,

    /// Percentage chance of random flip
    #[structopt(short, long, default_value = "50")]
    percent: u8,

    /// Step increase of percent if not flipped
    #[structopt(short, long)]
    step: Option<Option<u8>>,

    /// The text to convert, optional
    content: Option<String>,

    /// Convert the text to uppercase
    #[structopt(short, long, group = "action")]
    uppercase: bool,

    /// Convert the text to lowercase
    #[structopt(short, long, group = "action")]
    lowercase: bool,

    /// Reverse the case of the text
    #[structopt(short, long, group = "action")]
    reverse: bool,

    /// Convert the text to random case
    #[structopt(short, long, group = "action")]
    angry: bool,
}

impl Cli {
    const DEFAULT_STEP: u8 = 20;

    pub fn content(&self) -> Result<String> {
        if let Some(content) = &self.content {
            Ok(content.to_string())
        } else if let Some(path) = &self.input {
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("could not read file `{:?}`", path))?;

            Ok(content)
        } else {
            Err(Error::msg("Missing content"))
        }
    }

    pub fn convert(&self, s: String) -> String {
        if self.uppercase {
            SimpleConvertor::uppercase().convert(s)
        } else if self.lowercase {
            SimpleConvertor::lowercase().convert(s)
        } else if self.reverse {
            SimpleConvertor::reverse().convert(s)
        } else {
            let rng = &mut rand::thread_rng();
            // if Some(None) then replace with default value
            let step = self.step.map(|s| s.unwrap_or(Cli::DEFAULT_STEP));
            RandomConvertor::new(rng, self.percent, step).convert(s)
        }
    }
}
