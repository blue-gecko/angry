use {
    anyhow::{Context, Result},
    structopt::StructOpt,
};

/// Take the passed text and make it aNgrY by applying random capitalisation.
#[derive(StructOpt)]
pub struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
}

impl Cli {
    pub fn content(&self) -> Result<String> {
        let path = &self.path;
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("could not read file `{:?}`", path))?;
        Ok(content)
    }
}
