mod cli;
mod convert;

use {
    crate::convert::{Convertor, SimpleConvertor},
    anyhow::Result,
    structopt::StructOpt,
};

fn main() -> Result<()> {
    let cli = &cli::Cli::from_args();
    let content = cli.content()?;
    let convertor = SimpleConvertor::new();
    println!("file content: {}", convertor.convert(content));
    Ok(())
}
