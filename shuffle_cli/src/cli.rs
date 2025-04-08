use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author,
    about,
    version,
    after_help = "If you do not specify any of the \
[--uppercase, --lowercase, --digits, --symbols] flags, \
then uppercase lowercase and digits will be used."
)]
pub struct Cli {
    /// Use UPPERCASE letters [A-Z]
    #[clap(short, long)]
    pub(crate) uppercase: bool,

    /// Use lowercase letters [a-z]
    #[clap(short, long)]
    pub(crate) lowercase: bool,

    /// Use digits [0-9]
    #[clap(short, long)]
    pub(crate) digits: bool,

    /// Use special symbols [*&^%$#@!~]
    #[clap(short, long)]
    pub(crate) braces: bool,

    #[clap(short, long)]
    pub(crate) punctuation: bool,

    #[clap(short, long)]
    pub(crate) quotes: bool,

    #[clap(long)]
    pub(crate) dashes: bool,

    #[clap(short, long)]
    pub(crate) math: bool,

    #[clap(long)]
    pub(crate) logograms: bool,

    /// Sets the required password length
    #[clap(short = 'L', long, value_name = "NUMBER", default_value = "20")]
    length: usize,

    /// Output in a txt file
    #[clap(long)]
    output: Option<String>,

    /// Exclude char
    #[clap(long)]
    exclude: Option<String>,

    /// include char
    #[clap(long)]
    include: Option<String>,

}

impl Cli {

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn output(&self) -> Option<String> {
        self.output.clone()
    }

    pub fn exclude(&self) -> Option<String> {
        self.exclude.clone()
    }

    pub fn include(&self) -> Option<String> {
        self.include.clone()
    }
}

