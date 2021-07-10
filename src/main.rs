use {
    anyhow::Result,
    std::iter::{once, Iterator},
    structopt::StructOpt,
};

mod cli;

fn main() -> Result<()> {
    let cli = &cli::Cli::from_args();
    let content = cli.content()?;
    println!("file content: {}", convert(content));
    Ok(())
}

fn convert(s: String) -> String {
    s.chars().map(|c| convert_char(c)).flatten().collect()
}

fn convert_char(c: char) -> Box<dyn Iterator<Item = char>> {
    if c.is_alphabetic() {
        if c.is_lowercase() {
            Box::new(c.to_uppercase())
        } else {
            Box::new(c.to_lowercase())
        }
    } else {
        Box::new(once(c))
    }
}
