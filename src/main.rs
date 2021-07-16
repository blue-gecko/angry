mod cli;
mod convert;

use anyhow::Result;

fn main() -> Result<()> {
    let cli = cli::from_args();
    cli.print(cli.convert(cli.content()?))?;
    Ok(())
}
