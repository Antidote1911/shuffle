use crate::cli::Cli;

use clap::Parser;
use shuffle_core::*;
use std::{fs::File, io::LineWriter, io::Write};
use color_eyre::eyre::{eyre, Result};
use question::{Answer, Question};

pub mod cli;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error :", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // enable Windows terminal colors
    #[cfg(windows)]
    let _ = enable_ansi_support::enable_ansi_support();

    let opts: Cli = Cli::parse();

    let total_true = [opts.uppercase, opts.lowercase, opts.digits, opts.symbols].into_iter().filter(|b| *b).count();
    if opts.length()<total_true || opts.length()==0{
        return Err(eyre!("Password length must be greater or equal to the number of selected group chars."));
    }

    let mut config = PasswordConfig::new(opts.length())?
        .with_uppercase(opts.uppercase)
        .with_lowercase(opts.lowercase)
        .with_digits(opts.digits)
        .with_symbols(opts.symbols)
        .avoid_ambiguous(opts.exclude().unwrap_or("".to_string()));

    if !config.validate().is_ok(){
        config = Default::default();
        config.length = opts.length();
        config.avoid_ambiguous = opts.exclude().unwrap_or("".to_string());
    }

    let password = generate_password(&config);
    println!("{}", password);

    if opts.output().is_some() {
        let dest = opts.output().unwrap();
        if std::path::Path::new(&dest).is_dir() && &dest != "/dev/null" {
            return Err(eyre!("Can't save file. A folder with this name exist."));
        }
        if std::path::Path::new(&dest).exists() && &dest != "/dev/null" {
            println!("File: '{}' exist.", &dest);
            let answer = Question::new("Try to Overwrite ?")
                .default(Answer::NO)
                .show_defaults()
                .confirm();

            if answer == Answer::YES {
                writetxt(password.clone(), &dest).map_err(|e| eyre!(e))?;
                println!(
                    "{} '{}' {}",
                    "File",
                    opts.output().unwrap(),
                    "was overwritten."
                );
            } else {
                println!("{}", "Writting file canceled.");
            }
        } else {
            writetxt(password.clone(), &dest)?;
        }
    }
    Ok(())
}

fn writetxt(x: String, dest: &String) -> Result<()> {
    let file = File::create(dest)?;
    let mut file = LineWriter::new(file);

    let z=x.to_string();
    file.write_all(z.as_bytes())?;

    println!("{}", "File Saved.");

    Ok(())
}
