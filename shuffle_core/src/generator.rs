use crate::*;
use rand::{rng, seq::SliceRandom, Rng};
use std::collections::HashSet;

/// Generate a password
///
/// # Examples
/// ```
/// # use shuffle_core::{generate_password, PasswordConfig};
///
///  let config = PasswordConfig::new(32).unwrap();
///  config.include_symbols;
///  config.include_lowercase;
///  let password = generate_password(&config);
/// assert_eq!(password.len(), 32);
/// ```
pub fn generate_password(config: &PasswordConfig) -> String {
    let mut rng = rng();
    let mut charset = String::new();
    let mut password = Vec::with_capacity(config.length.into());

    let filtered_lowercase: String = DEFAULT_CHARSETS
        .lowercase
        .chars()
        .filter(|c| !config.avoid_ambiguous.contains(*c))
        .collect();

    let filtered_uppercase: String = DEFAULT_CHARSETS
        .uppercase
        .chars()
        .filter(|c| !config.avoid_ambiguous.contains(*c))
        .collect();

    let filtered_digits: String = DEFAULT_CHARSETS
        .digits
        .chars()
        .filter(|c| !config.avoid_ambiguous.contains(*c))
        .collect();

    let filtered_symbols: String = DEFAULT_CHARSETS
        .symbols
        .chars()
        .filter(|c| !config.avoid_ambiguous.contains(*c))
        .collect();

    if config.include_lowercase && !filtered_lowercase.is_empty() {
        password.push(
            filtered_lowercase
                .chars()
                .nth(rng.random_range(0..filtered_lowercase.len()))
                .unwrap(),
        );
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

    if config.include_symbols && !filtered_symbols.is_empty() {
        password.push(
            filtered_symbols
                .chars()
                .nth(rng.random_range(0..filtered_symbols.len()))
                .unwrap(),
        );
        charset.push_str(&filtered_symbols);
    }

    let charset_chars: Vec<char> = charset.chars().collect();
    let symbols: HashSet<char> = filtered_symbols.chars().collect();

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

    if symbols.contains(&password[0]) {
        if let Some(non_symbol_index) = password.iter().position(|c| !symbols.contains(c)) {
            password.swap(0, non_symbol_index);
        }
    }
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
    fn test_generate_password_with_symbols() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_symbols(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_lowercase() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_symbols(true)
            .with_lowercase(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_uppercase() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_symbols(true)
            .with_uppercase(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_digits() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_symbols(true)
            .with_digits(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_lowercase_and_uppercase() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_symbols(true)
            .with_lowercase(true)
            .with_uppercase(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_with_symbols_and_lowercase_and_digits() {
        let config = PasswordConfig::new(16)
            .unwrap()
            .with_symbols(true)
            .with_lowercase(true)
            .with_digits(true);
        let password = generate_password(&config);

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_password_does_not_start_with_symbol() {
        let config = PasswordConfig {
            length: 64,
            include_lowercase: true,
            include_uppercase: true,
            include_digits: true,
            include_symbols: true,
            avoid_ambiguous: "".to_string(),
        };

        for _ in 0..10000 {
            let password = generate_password(&config);
            let symbols: HashSet<char> = DEFAULT_CHARSETS.symbols.chars().collect();
            assert!(
                !symbols.contains(&password.chars().next().unwrap()),
                "Password started with a symbol: {}",
                password
            );
        }
    }

    #[test]
    fn test_exclude() {
        let config = PasswordConfig {
            length: 64,
            include_lowercase: true,
            include_uppercase: true,
            include_digits: true,
            include_symbols: true,
            avoid_ambiguous: "0s%m8".to_string(),
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
