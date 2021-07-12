use {
    anyhow::{Context, Error, Result},
    std::path::PathBuf,
    structopt::StructOpt,
};

/// Take the passed text and make it aNgrY by applying random capitalisation.
#[derive(StructOpt)]
pub struct Cli {
    /// The path to the input file to read, optional
    #[structopt(parse(from_os_str), short, long)]
    input: Option<PathBuf>,

    /// The text to convert, optional
    content: Option<String>,
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
}
