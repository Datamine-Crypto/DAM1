use patterns::{because, source};

pub struct CommandLine;
source!(
    CommandLine,
    "the console's command line: a command word, then options that take a value or stand alone, and plain words that are the texts and questions"
);

pub fn opt<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> Result<T, String> {
    match args.iter().position(|a| a == name) {
        None => Ok(default),
        Some(i) => args.get(i + 1).and_then(|v| v.parse().ok()).ok_or(format!("{name} needs a value")),
    }
}
because!(opt, CommandLine, "the value after an option, or the default when the option is absent, refusing an option with no value after it");

pub fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}
because!(flag, CommandLine, "whether an option that takes no value was given");

pub fn pct(x: f32, percent: f32) -> String {
    format!("{:.1}%", x * percent)
}
because!(pct, "a share as a percent with one decimal, how every score is printed");
