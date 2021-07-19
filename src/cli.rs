use {
    crate::convert::{random::RandomConvertor, simple::SimpleConvertor, Convertor},
    anyhow::{Context, Error, Result},
    std::{
        fs::File,
        io::{self, BufRead, BufReader, Cursor, LineWriter, Write},
        path::PathBuf,
    },
    structopt::{clap::ArgGroup, StructOpt},
};

#[derive(Debug)]
enum Conversion {
    Uppercase,
    Lowercase,
    Reverse,
    Random(u8, Option<u8>),
}

fn parse_percent(s: &str) -> Result<u8> {
    let i: u8 = s.parse()?;

    if i > 100 {
        Err(Error::msg(format!(
            "percentage must be between 0 and 100, was {}",
            i
        )))
    } else {
        Ok(i)
    }
}

/// Take the passed text and make it aNgrY by applying random capitalisation.
#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("action").required(true))]
pub struct Cli {
    /// The path to the input file to read, optional
    #[structopt(parse(from_os_str), short, long)]
    input: Option<PathBuf>,

    /// The path to the output file to write, optional
    #[structopt(parse(from_os_str), short, long)]
    output: Option<PathBuf>,

    /// Percentage chance of random flip
    #[structopt(short, long, default_value = "50", parse(try_from_str = parse_percent))]
    percent: u8,

    /// Step increase of percent if not flipped
    #[structopt(short, long)]
    step: Option<Option<u8>>,

    /// The text to convert, optional
    content: Option<String>,

    /// Convert the text to uppercase
    #[structopt(short, long, group = "action")]
    uppercase: bool,

    /// Convert the text to lowercase
    #[structopt(short, long, group = "action")]
    lowercase: bool,

    /// Reverse the case of the text
    #[structopt(short, long, group = "action")]
    reverse: bool,

    /// Convert the text to random case
    #[structopt(short, long, group = "action", alias = "random")]
    angry: bool,
}

type Convert = Box<dyn Convertor>;
type Input = Box<dyn Iterator<Item = io::Result<String>>>;
type Output = LineWriter<Box<dyn Write>>;

#[allow(dead_code)]
impl Cli {
    const DEFAULT_STEP: u8 = 20;

    fn conversion(&self) -> Conversion {
        if self.uppercase {
            Conversion::Uppercase
        } else if self.lowercase {
            Conversion::Lowercase
        } else if self.reverse {
            Conversion::Reverse
        } else {
            let step = self.step.map(|s| s.unwrap_or(Cli::DEFAULT_STEP));
            Conversion::Random(self.percent, step)
        }
    }

    fn convertor(&self) -> Convert {
        match self.conversion() {
            Conversion::Uppercase => SimpleConvertor::uppercase(),
            Conversion::Lowercase => SimpleConvertor::lowercase(),
            Conversion::Reverse => SimpleConvertor::reverse(),
            Conversion::Random(percent, step) => RandomConvertor::new(percent, step),
        }
    }

    fn input(&self) -> Result<Input> {
        if let Some(input) = &self.content {
            let buffer = Cursor::new(String::from(input));
            Ok(Box::new(buffer.lines()))
        } else if let Some(path) = &self.input {
            let file =
                File::open(path).with_context(|| format!("could not open file `{:?}`", path))?;
            let buffer = BufReader::new(file);
            Ok(Box::new(buffer.lines()))
        } else {
            let read = io::stdin();
            let buffer = BufReader::new(read);
            Ok(Box::new(buffer.lines()))
        }
    }

    fn output(&self) -> Result<Output> {
        if let Some(file) = &self.output {
            let handle =
                File::create(file).with_context(|| format!("could not write file `{:?}`", file))?;
            Ok(LineWriter::new(Box::new(handle)))
        } else {
            let handle = io::stdout();
            Ok(LineWriter::new(Box::new(handle)))
        }
    }

    pub fn convert(&self) -> Result<()> {
        match self.input() {
            Ok(input) => match self.output() {
                Ok(output) => match self._convert(self.convertor(), input, output) {
                    Some(e) => Err(Error::from(e)),
                    None => Ok(()),
                },
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn _convert(
        &self,
        mut convertor: Convert,
        input: Input,
        mut output: Output,
    ) -> Option<io::Error> {
        input
            .filter_map(|s| s.ok())
            .map(|s| writeln!(output, "{}", convertor.convert(s)))
            .filter_map(|r| r.err())
            .nth(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_percent_success() {
        assert!(matches!(parse_percent("100"), Ok(100)));
    }

    #[test]
    fn parse_percent_fail() {
        assert!(matches!(parse_percent("101"), Err(_)));
    }

    #[test]
    fn with_uppercase_arg() {
        assert_eq!(Cli::from_iter(&["test", "-u"]).uppercase, true);
    }

    #[test]
    fn with_uppercase_long_arg() {
        assert_eq!(Cli::from_iter(&["test", "--uppercase"]).uppercase, true);
    }

    #[test]
    fn with_lowercase_arg() {
        assert_eq!(Cli::from_iter(&["test", "-l"]).lowercase, true);
    }

    #[test]
    fn with_lowercase_long_arg() {
        assert_eq!(Cli::from_iter(&["test", "--lowercase"]).lowercase, true);
    }

    #[test]
    fn with_uppercase_and_lowercase_long_arg() {
        assert_eq!(
            Cli::clap()
                .get_matches_from_safe(&["test", "--uppercase", "--lowercase"])
                .is_err(),
            true
        );
    }

    #[test]
    fn with_reverse_arg() {
        assert_eq!(Cli::from_iter(&["test", "-r"]).reverse, true);
    }

    #[test]
    fn with_reverse_long_arg() {
        assert_eq!(Cli::from_iter(&["test", "--reverse"]).reverse, true);
    }

    #[test]
    fn with_random_arg() {
        assert_eq!(Cli::from_iter(&["test", "-a"]).angry, true);
    }

    #[test]
    fn with_random_and_percent_default() {
        assert_eq!(Cli::from_iter(&["test", "--angry"]).percent, 50)
    }

    #[test]
    fn with_random_and_percent_as_1_arg() {
        assert_eq!(Cli::from_iter(&["test", "--random", "-p60"]).percent, 60)
    }

    #[test]
    fn with_random_and_percent_as_2_args() {
        assert_eq!(Cli::from_iter(&["test", "-a", "-p", "70"]).percent, 70)
    }

    #[test]
    fn with_random_and_percent_equals() {
        assert_eq!(Cli::from_iter(&["test", "-a", "-p=80"]).percent, 80)
    }

    #[test]
    fn with_random_and_percent_combined() {
        let cli = Cli::from_iter(&["test", "-ap90"]);
        assert_eq!(cli.angry, true);
        assert_eq!(cli.percent, 90);
    }

    #[test]
    fn without_required_arg() {
        assert!(Cli::clap().get_matches_from_safe(&["test"]).is_err());
    }

    #[test]
    fn with_invalid_arg() {
        assert!(Cli::clap()
            .get_matches_from_safe(&["test", "-u", "-x"])
            .is_err());
    }

    #[test]
    fn conversion_with_uppercase() {
        assert!(matches!(
            Cli::from_iter(&["test", "--uppercase"]).conversion(),
            Conversion::Uppercase
        ));
    }

    #[test]
    fn conversion_with_lowercase() {
        assert!(matches!(
            Cli::from_iter(&["test", "--lowercase"]).conversion(),
            Conversion::Lowercase
        ));
    }

    #[test]
    fn conversion_with_reverse() {
        assert!(matches!(
            Cli::from_iter(&["test", "--reverse"]).conversion(),
            Conversion::Reverse
        ));
    }

    #[test]
    fn conversion_with_random() {
        assert!(matches!(
            Cli::from_iter(&["test", "--angry"]).conversion(),
            Conversion::Random(50, None)
        ));
    }

    #[test]
    fn conversion_with_random_percent() {
        assert!(matches!(
            Cli::from_iter(&["test", "--angry", "-p60"]).conversion(),
            Conversion::Random(60, None)
        ));
    }

    #[test]
    fn conversion_with_random_default_step() {
        matches!(
            Cli::from_iter(&["test", "--angry", "-s"]).conversion(),
            Conversion::Random(50, Some(20))
        );
    }

    #[test]
    fn conversion_with_random_step() {
        assert!(matches!(
            Cli::from_iter(&["test", "--angry", "-s25"]).conversion(),
            Conversion::Random(50, Some(25))
        ));
    }

    #[test]
    fn with_random_and_percent_out_of_bound() {
        assert!(Cli::clap()
            .get_matches_from_safe(&["test", "-a", "-p", "101"])
            .is_err());
    }
}
