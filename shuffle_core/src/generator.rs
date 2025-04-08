use crate::*;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_hc::Hc128Rng;
use rand_isaac::Isaac64Rng;

/// Generate a password
///
/// # Examples
/// ```
/// # use shuffle_core::{generate_password, PasswordConfig};
///
///  let config = PasswordConfig::new(32).unwrap();
///  config.include_digits;
///  config.include_lowercase;
///  let password = generate_password(&config);
/// assert_eq!(password.len(), 32);
/// ```
pub fn generate_password(config: &PasswordConfig) -> String {
    let mut isaac_seeder = Isaac64Rng::from_os_rng();
    let mut rng = Hc128Rng::from_rng(&mut isaac_seeder);

    let mut charset = String::new();
    let mut password = Vec::with_capacity(config.length);

    let filtered_lowercase: String = DEFAULT_CHARSETS
        .lowercase
        .chars()
        .filter(|c| !config.excluded.contains(*c))
        .collect();

    let filtered_uppercase: String = DEFAULT_CHARSETS
        .uppercase
        .chars()
        .filter(|c| !config.excluded.contains(*c))
        .collect();

    let filtered_digits: String = DEFAULT_CHARSETS
        .digits
        .chars()
        .filter(|c| !config.excluded.contains(*c))
        .collect();

    let filtered_braces: String = DEFAULT_CHARSETS
        .braces
        .chars()
        .filter(|c| !config.excluded.contains(*c))
        .collect();

    let filtered_punctuation: String = DEFAULT_CHARSETS
        .punctuation
        .chars()
        .filter(|c| !config.excluded.contains(*c))
        .collect();

    let filtered_quotes: String = DEFAULT_CHARSETS
        .quotes
        .chars()
        .filter(|c| !config.excluded.contains(*c))
        .collect();

    let filtered_dashes: String = DEFAULT_CHARSETS
        .dashes
        .chars()
        .filter(|c| !config.excluded.contains(*c))
        .collect();

    let filtered_math: String = DEFAULT_CHARSETS
        .math
        .chars()
        .filter(|c| !config.excluded.contains(*c))
        .collect();

    let filtered_logograms: String = DEFAULT_CHARSETS
        .logograms
        .chars()
        .filter(|c| !config.excluded.contains(*c))
        .collect();

    //////////////////
    if !config.included.is_empty() {
        let included_chars: Vec<char> = config.included.chars().collect();
        password.push(included_chars[rng.random_range(0..included_chars.len())]);
        charset.push_str(&config.included);
    }

    if config.include_lowercase && !filtered_lowercase.is_empty() {
        let lowercase_chars: Vec<char> = filtered_lowercase.chars().collect();
        password.push(lowercase_chars[rng.random_range(0..lowercase_chars.len())]);
        charset.push_str(&filtered_lowercase);
    }

    if config.include_uppercase && !filtered_uppercase.is_empty() {
        let uppercase_chars: Vec<char> = filtered_uppercase.chars().collect();
        password.push(uppercase_chars[rng.random_range(0..uppercase_chars.len())]);
        charset.push_str(&filtered_uppercase);
    }

    if config.include_digits && !filtered_digits.is_empty() {
        let digit_chars: Vec<char> = filtered_digits.chars().collect();
        password.push(digit_chars[rng.random_range(0..digit_chars.len())]);
        charset.push_str(&filtered_digits);
    }

    if config.include_braces && !filtered_braces.is_empty() {
        let brace_chars: Vec<char> = filtered_braces.chars().collect();
        password.push(brace_chars[rng.random_range(0..brace_chars.len())]);
        charset.push_str(&filtered_braces);
    }

    if config.include_punctuation && !filtered_punctuation.is_empty() {
        let punctuation_chars: Vec<char> = filtered_punctuation.chars().collect();
        password.push(punctuation_chars[rng.random_range(0..punctuation_chars.len())]);
        charset.push_str(&filtered_punctuation);
    }

    if config.include_quotes && !filtered_quotes.is_empty() {
        let quote_chars: Vec<char> = filtered_quotes.chars().collect();
        password.push(quote_chars[rng.random_range(0..quote_chars.len())]);
        charset.push_str(&filtered_quotes);
    }

    if config.include_dashes && !filtered_dashes.is_empty() {
        let dash_chars: Vec<char> = filtered_dashes.chars().collect();
        password.push(dash_chars[rng.random_range(0..dash_chars.len())]);
        charset.push_str(&filtered_dashes);
    }

    if config.include_math && !filtered_math.is_empty() {
        let math_chars: Vec<char> = filtered_math.chars().collect();
        password.push(math_chars[rng.random_range(0..math_chars.len())]);
        charset.push_str(&filtered_math);
    }

    if config.include_logograms && !filtered_logograms.is_empty() {
        let logogram_chars: Vec<char> = filtered_logograms.chars().collect();
        password.push(logogram_chars[rng.random_range(0..logogram_chars.len())]);
        charset.push_str(&filtered_logograms);
    }

    let charset_chars: Vec<char> = charset.chars().collect();

    // Fill remaining characters
    while password.len() < config.length.into() {
        let c = {
            *charset_chars
                .get(rng.random_range(0..charset_chars.len()))
                .unwrap_or(&'~') // Fallback if empty
        };
        password.push(c);
    }

    password.shuffle(&mut rng);

    password.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        let config = PasswordConfig::new(16).unwrap();
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_multibyte_unicode() {
        let config = PasswordConfig {
            length: 16,
            included: "é".to_string(),
            ..Default::default()
        };
        let password = generate_password(&config);

        assert!(password.contains('é'));
    }
    #[test]
    fn test_generate_password_with_included() {
        let config = PasswordConfig::new(16).unwrap().with_digits(true).included("é".to_string());
        let password = generate_password(&config);

        assert!(password.contains('é'));

    }

    #[test]
    fn test_generate_password_with_symbols() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_logograms(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }
}