/// Error type for password configuration validation
#[derive(Debug)]
pub enum PasswordConfigError {
    ZeroLength,
    NoCharacterSetsEnabled,
    LengthTooShortForSets { length: usize, sets_count: usize },
    NotEnoughAvailableCharacters { length: usize, available: usize },
    PinLengthTooShort,
}

impl std::fmt::Display for PasswordConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroLength => write!(f, "Password length must be greater than 0."),
            Self::NoCharacterSetsEnabled => {
                write!(f, "At least one character set must be included.")
            }
            Self::LengthTooShortForSets { length, sets_count } => {
                write!(
                    f,
                    "Password length ({}) must be at least equal to the number of required character sets ({}).",
                    length, sets_count
                )
            }
            Self::NotEnoughAvailableCharacters { length, available } => {
                write!(
                    f,
                    "Password length ({}) is too long given the restricted character set ({} available characters after avoiding ambiguous ones).",
                    length, available
                )
            }
            Self::PinLengthTooShort => write!(f, "PIN length must be at least 4 characters."),
        }
    }
}

impl std::error::Error for PasswordConfigError {}

/// Configuration for password generation
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// Length of the password to generate
    pub length: usize,

    /// Include lowercase letters (a-z)
    pub include_lowercase: bool,

    /// Include uppercase letters (A-Z)
    pub include_uppercase: bool,

    /// Include numeric digits (0-9)
    pub include_digits: bool,

    /// Include braces: ()[]{}
    pub include_braces: bool,

    /// Include punctuation: .,:;
    pub include_punctuation: bool,

    /// Include quotes: "'
    pub include_quotes: bool,

    /// Include dashes: -/\_|
    pub include_dashes: bool,

    /// Include math: !*+<=>?
    pub include_math: bool,

    /// Include logograms: #$%&@^`~
    pub include_logograms: bool,

    /// Avoid ambiguous characters (0O1Il5S)
    pub excluded: String,

    /// Also include characters
    pub included: String,
}

impl Default for PasswordConfig {
    /// Creates a default password configuration:
    /// - 18 characters long
    /// - Includes lowercase, uppercase, digits, and symbols
    /// - Avoids ambiguous characters
    /// - Requires at least one character from each included set
    fn default() -> Self {
        Self {
            length: 10,
            include_lowercase: true,
            include_uppercase: true,
            include_digits: true,
            include_braces: false,
            include_punctuation: false,
            include_quotes: false,
            include_dashes: false,
            include_math: false,
            include_logograms: false,
            excluded: String::from(""),
            included: String::from(""),
        }
    }
}

impl PasswordConfig {
    /// Creates a new password configuration with the specified length
    /// and default settings for other options
    pub fn new(length: usize) -> Result<Self, PasswordConfigError> {
        if length == 0 {
            return Err(PasswordConfigError::ZeroLength);
        }

        Ok(Self {
            length,
            ..Self::default()
        })
    }

    /// Builder method to set whether to include lowercase letters
    pub const fn with_lowercase(mut self, include: bool) -> Self {
        self.include_lowercase = include;
        self
    }

    /// Builder method to set whether to include uppercase letters
    pub const fn with_uppercase(mut self, include: bool) -> Self {
        self.include_uppercase = include;
        self
    }

    /// Builder method to set whether to include digits
    pub const fn with_digits(mut self, include: bool) -> Self {
        self.include_digits = include;
        self
    }

    /// Builder method to set whether to include braces
    pub const fn with_braces(mut self, include: bool) -> Self {
        self.include_braces = include;
        self
    }

    /// Builder method to set whether to include punctuation
    pub const fn with_punctuation(mut self, include: bool) -> Self {
        self.include_punctuation = include;
        self
    }

    /// Builder method to set whether to include quotes
    pub const fn with_quotes(mut self, include: bool) -> Self {
        self.include_quotes = include;
        self
    }

    /// Builder method to set whether to include dashes
    pub const fn with_dashes(mut self, include: bool) -> Self {
        self.include_dashes = include;
        self
    }

    /// Builder method to set whether to include math
    pub const fn with_math(mut self, include: bool) -> Self {
        self.include_math = include;
        self
    }

    /// Builder method to set whether to include logograms
    pub const fn with_logograms(mut self, include: bool) -> Self {
        self.include_logograms = include;
        self
    }

    /// Builder method to set excluded chars
    pub fn excluded(mut self, include: String) -> Self {
        self.excluded=include;
        self

    }

    /// Builder method to set included chars
    pub fn included(mut self, include: String) -> Self {
        self.included=include;
        self

    }

    /// Validates the configuration
    pub const fn validate(&self) -> Result<(), PasswordConfigError> {
        if self.length == 0 {
            return Err(PasswordConfigError::ZeroLength);
        }

        let sets_count = self.include_lowercase as usize
            + self.include_uppercase as usize
            + self.include_digits as usize
            + self.include_braces as usize
            + self.include_punctuation as usize
            + self.include_quotes as usize
            + self.include_dashes as usize
            + self.include_math as usize
            + self.include_logograms as usize;

        if sets_count == 0 {
            return Err(PasswordConfigError::NoCharacterSetsEnabled);
        }

        if self.length < sets_count {
            return Err(PasswordConfigError::LengthTooShortForSets {
                length: self.length,
                sets_count,
            });
        }

        Ok(())
    }
}
