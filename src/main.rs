mod cli;
mod convert;

use anyhow::Result;

fn main() -> Result<()> {
    let cli = cli::from_args();
    println!("{}", cli.convert(cli.content()?));
    Ok(())
}
