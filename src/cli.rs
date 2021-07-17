use {
    crate::convert::{random::RandomConvertor, simple::SimpleConvertor, Convertor},
    anyhow::{Context, Error, Result},
    std::{
        fs::File,
        io::{self, BufRead, BufReader, BufWriter, Cursor, Write},
        path::{Path, PathBuf},
    },
    structopt::{clap::ArgGroup, StructOpt},
};

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

    pub fn content(&self) -> Result<String> {
        if let Some(content) = &self.content {
            self.content_from_arg(content)
        } else if let Some(path) = &self.input {
            self.content_from_file(path)
        } else {
            self.content_from_stdin()
        }
    }

    fn content_from_arg(&self, s: &str) -> Result<String> {
        let mut buffer = Cursor::new(s);
        self._content(&mut buffer)
    }

    fn content_from_file(&self, path: &Path) -> Result<String> {
        let file = File::open(path).with_context(|| format!("could not open file `{:?}`", path))?;
        let mut buffer = BufReader::new(file);
        self._content(&mut buffer)
    }

    fn content_from_stdin(&self) -> Result<String> {
        let input = io::stdin();
        let read = input.lock();
        let mut buffer = BufReader::new(read);
        self._content(&mut buffer)
    }

    fn _content(&self, buffer: &mut dyn BufRead) -> Result<String> {
        let mut content = String::new();
        buffer
            .read_to_string(&mut content)
            .with_context(|| "could not read buffer".to_string())?;

        Ok(content)
    }

    pub fn print(&self, s: String) -> Result<()> {
        if let Some(file) = &self.output {
            let handle =
                File::create(file).with_context(|| format!("could not write file `{:?}`", file))?;
            self._print(BufWriter::new(handle), false, s)?;
        } else {
            let stdout = io::stdout();
            let handle = stdout.lock();
            self._print(BufWriter::new(handle), true, s)?;
        };

        Ok(())
    }

    fn _print<T: Write>(&self, mut buffer: BufWriter<T>, console: bool, s: String) -> Result<()> {
        write!(buffer, "{}", s)?;

        if console {
            writeln!(buffer)?;
        }
        Ok(())
    }

    pub fn convert<T: Into<String>>(&self, s: T) -> String {
        let s = s.into();
        match self.conversion() {
            Conversion::Uppercase => SimpleConvertor::uppercase().convert(s),
            Conversion::Lowercase => SimpleConvertor::lowercase().convert(s),
            Conversion::Reverse => SimpleConvertor::reverse().convert(s),
            Conversion::Random(percent, step) => {
                let rng = &mut rand::thread_rng();
                RandomConvertor::new(rng, percent, step).convert(s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(Cli::clap().get_matches_from_safe(&["test"]).is_err(), true);
    }

    #[test]
    fn with_invalid_arg() {
        assert_eq!(
            Cli::clap()
                .get_matches_from_safe(&["test", "-u", "-x"])
                .is_err(),
            true
        );
    }

    #[test]
    fn conversion_with_uppercase() {
        matches!(
            Cli::from_iter(&["test", "--uppercase"]).conversion(),
            Conversion::Uppercase
        );
    }

    #[test]
    fn conversion_with_lowercase() {
        matches!(
            Cli::from_iter(&["test", "--lowercase"]).conversion(),
            Conversion::Lowercase
        );
    }

    #[test]
    fn conversion_with_reverse() {
        matches!(
            Cli::from_iter(&["test", "--reverse"]).conversion(),
            Conversion::Uppercase
        );
    }

    #[test]
    fn conversion_with_random() {
        matches!(
            Cli::from_iter(&["test", "--angry"]).conversion(),
            Conversion::Random(50, None)
        );
    }

    #[test]
    fn conversion_with_random_percent() {
        matches!(
            Cli::from_iter(&["test", "--angry", "-p60"]).conversion(),
            Conversion::Random(60, None)
        );
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
        matches!(
            Cli::from_iter(&["test", "--angry", "-s25"]).conversion(),
            Conversion::Random(50, Some(25))
        );
    }

    #[test]
    fn with_random_and_percent_out_of_bound() {
        assert_eq!(
            Cli::clap()
                .get_matches_from_safe(&["test", "-a", "-p", "101"])
                .is_err(),
            true
        );
    }
}
