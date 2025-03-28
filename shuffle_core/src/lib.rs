pub mod config;
pub mod generator;


pub use config::PasswordConfig;
pub use generator::generate_password;

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const fn getversion() -> &'static str {
    APP_VERSION
}

/// Character sets for password generation
pub struct CharacterSets {
    /// Lowercase letters: a-z
    pub lowercase: &'static str,
    /// Uppercase letters: A-Z
    pub uppercase: &'static str,
    /// Numeric digits: 0-9
    pub digits: &'static str,
    /// Braces: ()[]{}
    pub braces: &'static str,
    /// Punctuation: .,:;
    pub punctuation: &'static str,
    /// Quotes: "'
    pub quotes: &'static str,
    /// Dashes: -/\_|
    pub dashes: &'static str,
    /// Math: !*+<=>?
    pub math: &'static str,
    /// Logograms: #$%&@^`~
    pub logograms: &'static str,

}

/// Default character sets for password generation
pub const DEFAULT_CHARSETS: CharacterSets = CharacterSets {
    lowercase: "abcdefghijklmnopqrstuvwxyz",
    uppercase: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    digits: "0123456789",
    braces: "()[]{}",
    punctuation: ".,:;",
    quotes: "\"'",
    dashes: "-/\\_|",
    math: "!*+<=>?",
    logograms: "#$%&@^`~",
};


