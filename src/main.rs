mod cli;
mod convert;

use {anyhow::Result, structopt::StructOpt};

fn main() -> Result<()> {
    let cli = &cli::Cli::from_args();
    let content = cli.content()?;
    println!("{}", &cli.convert(content));
    Ok(())
}
