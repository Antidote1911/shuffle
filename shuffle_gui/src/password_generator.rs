use shuffle_core::*;


pub struct ComplexPasswordGenerator {
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
}

impl ComplexPasswordGenerator {
    pub fn new(include_uppercase: bool, include_lowercase: bool, include_numbers: bool, include_symbols: bool) -> Self {
        Self {
            include_uppercase,
            include_lowercase,
            include_numbers,
            include_symbols,
        }
    }
    pub fn generate_password(&self, length: usize) -> String {

        let config = PasswordConfig::new(length).unwrap()
            .with_uppercase(self.include_uppercase)
            .with_lowercase(self.include_lowercase)
            .with_digits(self.include_numbers)
            .with_symbols(self.include_symbols)
            .avoid_ambiguous("".to_string());

        generate_password(&config)
    }
}