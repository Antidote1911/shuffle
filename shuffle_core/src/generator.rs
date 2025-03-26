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
        let temp =config.included.clone();
        password.push(temp.chars().nth(rng.random_range(0..temp.len())).unwrap());
        charset.push_str(&temp);
    }

    if config.include_lowercase && !filtered_lowercase.is_empty() {
        password.push(filtered_lowercase.chars().nth(rng.random_range(0..filtered_lowercase.len())).unwrap());
        charset.push_str(&filtered_lowercase);
    }

    if config.include_uppercase && !filtered_uppercase.is_empty() {
        password.push(
            filtered_uppercase
                .chars()
                .nth(rng.random_range(0..filtered_uppercase.len()))
                .unwrap(),
        );
        charset.push_str(&filtered_uppercase);
    }

    if config.include_digits && !filtered_digits.is_empty() {
        password.push(
            filtered_digits
                .chars()
                .nth(rng.random_range(0..filtered_digits.len()))
                .unwrap(),
        );
        charset.push_str(&filtered_digits);
    }

    if config.include_braces && !filtered_braces.is_empty() {
        password.push(
            filtered_braces
                .chars()
                .nth(rng.random_range(0..filtered_braces.len()))
                .unwrap(),
        );
        charset.push_str(&filtered_braces);
    }

    if config.include_punctuation && !filtered_punctuation.is_empty() {
        password.push(
            filtered_punctuation
                .chars()
                .nth(rng.random_range(0..filtered_punctuation.len()))
                .unwrap(),
        );
        charset.push_str(&filtered_punctuation);
    }

    if config.include_quotes && !filtered_quotes.is_empty() {
        password.push(
            filtered_quotes
                .chars()
                .nth(rng.random_range(0..filtered_quotes.len()))
                .unwrap(),
        );
        charset.push_str(&filtered_quotes);
    }

    if config.include_dashes && !filtered_dashes.is_empty() {
        password.push(
            filtered_dashes
                .chars()
                .nth(rng.random_range(0..filtered_dashes.len()))
                .unwrap(),
        );
        charset.push_str(&filtered_dashes);
    }

    if config.include_math && !filtered_math.is_empty() {
        password.push(
            filtered_math
                .chars()
                .nth(rng.random_range(0..filtered_math.len()))
                .unwrap(),
        );
        charset.push_str(&filtered_math);
    }

    if config.include_logograms && !filtered_logograms.is_empty() {
        password.push(
            filtered_logograms
                .chars()
                .nth(rng.random_range(0..filtered_logograms.len()))
                .unwrap(),
        );
        charset.push_str(&filtered_logograms);
    }

    let charset_chars: Vec<char> = charset.chars().collect();

    // Fill remaining characters
    while password.len() < config.length.into() {
        //let remaining = config.length.saturating_sub(password.len());

        let c =  {
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

    fn test_generate_password_with_included() {
        let config = PasswordConfig::new(16).unwrap().included("@".parse().unwrap());
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_logograms(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_lowercase() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_logograms(true)
            .with_lowercase(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_uppercase() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_logograms(true)
            .with_uppercase(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_digits() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_logograms(true)
            .with_digits(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_lowercase_and_uppercase() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_logograms(true)
            .with_lowercase(true)
            .with_uppercase(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_lowercase_and_digits() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_logograms(true)
            .with_lowercase(true)
            .with_digits(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);

    }

    #[test]
    fn test_exclude() {
        let config = PasswordConfig {
            length: 64,
            include_lowercase: true,
            include_uppercase: true,
            include_digits: true,
            include_braces: true,
            include_punctuation: true,
            include_quotes: true,
            include_dashes: true,
            include_math: true,
            include_logograms: true,
            excluded: "0s%m8".to_string(),
            included: String::from(""),

        };

        for _ in 0..10000 {
            let password = generate_password(&config);
            assert!(
                !password.contains("0") &&
                !password.contains("%") &&
                !password.contains("m") &&
                !password.contains("8"),
                "Password contain excluded symbol: {}",
                password
            );
        }
    }
}
