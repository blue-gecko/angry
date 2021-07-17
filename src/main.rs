mod cli;
mod convert;

use {crate::cli::Cli, anyhow::Result, structopt::StructOpt};

fn main() -> Result<()> {
    let cli = Cli::from_args();
    cli.print(cli.convert(cli.content()?))?;
    Ok(())
}
