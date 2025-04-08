#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    fn cmd() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    #[test]
    fn test_help() {
        let mut cmd = cmd();
        let assert = cmd.arg("--help").assert();

        assert.stdout(predicate::str::contains("password generator"));
    }



    #[test]
    fn test_create_alphanumeric_24_long() {
        let pattern = r"^[a-zA-Z0-9]{24}\n$"; // Ensure exactly 24 chars + newline

        for _ in 0..100 {
            let mut cmd = cmd();
            cmd.args(&["-L", "24","-dlu"])
                .assert()
                .stdout(predicate::str::is_match(pattern).unwrap());
        }
    }

    #[test]
    fn test_create_all_groups_100_long() {
        let pattern = r"^[a-zA-Z0-9]{100}\n$"; // Ensure exactly 100 chars + newline

        for _ in 0..100 {
            let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
            cmd.args(&["-L", "100","-dlu"])
                .assert()
                .stdout(predicate::str::is_match(pattern).unwrap());
        }
    }

    #[test]
    fn test_create_digits() {
        let pattern = r"^[0-9]{20}\n$"; // Ensure exactly 20 numeric chars + newline

        for _ in 0..100 {
            let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
            cmd.arg("-d")
                .assert()
                .stdout(predicate::str::is_match(pattern).unwrap());
        }
    }
    #[test]
    fn test_create_upercase() {
        let pattern = r"^[A-Z]{20}\n$"; // Ensure exactly 10 alphanumeric chars + newline

        for _ in 0..1000 {
            let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
            cmd.arg("-u")
                .assert()
                .stdout(predicate::str::is_match(pattern).unwrap());
        }
    }


    #[test]
    fn integration_error() {
        // Check if an error occurs
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.args(["-L", "0"]).assert().failure();
    }

}
