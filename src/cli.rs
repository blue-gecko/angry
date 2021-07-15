use {
    crate::convert::{random::RandomConvertor, simple::SimpleConvertor, Convertor},
    anyhow::{Context, Error, Result},
    std::path::PathBuf,
    structopt::{clap::ArgGroup, StructOpt},
};

/// Take the passed text and make it aNgrY by applying random capitalisation.
#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("action").required(true))]
pub struct Cli {
    /// The path to the input file to read, optional
    #[structopt(parse(from_os_str), short, long)]
    input: Option<PathBuf>,

    /// The text to convert, optional
    content: Option<String>,

    /// Convert to uppercase
    #[structopt(short, long, group = "action")]
    uppercase: bool,

    /// Convert to lowercase
    #[structopt(short, long, group = "action")]
    lowercase: bool,

    /// Convert to reverse case
    #[structopt(short, long, group = "action")]
    reverse: bool,

    /// Convert to angry random case
    #[structopt(short, long, group = "action")]
    angry: bool,
}

impl Cli {
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
            RandomConvertor::new(rng).convert(s)
        }
    }
}
