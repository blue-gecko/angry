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
    // let mut rng = rand::thread_rng();
    // let mut convertor = RandomConvertor::new(&mut rng);
    let mut convertor = SimpleConvertor::uppercase();
    println!("file content: {}", convertor.convert(content));
    Ok(())
}
