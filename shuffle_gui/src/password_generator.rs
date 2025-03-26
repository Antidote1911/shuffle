use shuffle_core::*;


pub struct ComplexPasswordGenerator {
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_logograms: bool,
}

impl ComplexPasswordGenerator {
    pub fn new(include_uppercase: bool, include_lowercase: bool, include_numbers: bool, include_logograms: bool) -> Self {
        Self {
            include_uppercase,
            include_lowercase,
            include_numbers,
            include_logograms,
        }
    }
    pub fn generate_password(&self, length: usize) -> String {

        let config = PasswordConfig::new(length).unwrap()
            .with_uppercase(self.include_uppercase)
            .with_lowercase(self.include_lowercase)
            .with_digits(self.include_numbers)
            .with_logograms(self.include_logograms);

        generate_password(&config)
    }
}