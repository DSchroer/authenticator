mod secret;
mod error;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use crate::secret::Secret;

const DEFAULT_FILE: &str = ".authenticator";

#[derive(Parser)]
#[clap(author, version)]
#[clap(author = "Dominick Schroer <dominick@schroer.ca>")]
#[clap(about = "TOTP Authenticator CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all secrets
    List {},
    /// Add a new secret
    Add {
        /// Name of the secret
        name: String,
    },
    /// Remove a secret
    Remove {
        /// Name of the secret
        name: String,
    },
    /// Show the value of a secret
    Show {
        /// Name of the secret
        name: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let command: &Commands = &cli.command;

    match command {
        Commands::List {} => list_secrets(),
        Commands::Add { name } => add_secret(name),
        Commands::Remove { name } => remove_secret(name),
        Commands::Show { name } => show_secret(name),
    }
}

fn add_secret(name: &String) -> Result<(), Box<dyn Error>> {
    println!("Secret: ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let mut secrets = open_secrets()?;
    secrets.push(Secret::new(name.clone(), line.trim().to_string()));
    save_secrets(&secrets)
}

fn remove_secret(name: &String) -> Result<(), Box<dyn Error>> {
    let secrets = open_secrets()?;
    let updated = secrets.into_iter().filter(|s|s.name!=*name).collect();
    save_secrets(&updated)
}

fn list_secrets() -> Result<(), Box<dyn Error>> {
    for secret in open_secrets()? {
        println!("{}", secret.name)
    }
    Ok(())
}

fn show_secret(name: &str) -> Result<(), Box<dyn Error>> {
    let secrets = open_secrets()?;
    if let Some(secret) = secrets.iter().find(|s|s.name==name) {
        println!("{}", secret.generate()?);
        return Ok(());
    }
    Err(Box::new(crate::error::Error::from("no secret found")))
}

fn open_secrets() -> Result<Vec<Secret>, Box<dyn Error>> {
    let path = file_path();
    let file = File::open(path);
    match file {
        Err(std::io::Error{ .. }) => Ok(Vec::new()),
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            let mut secrets = Vec::new();
            for line in s.split('\n').filter(|l|!l.is_empty()) {
                secrets.push(Secret::parse(line)?);
            }
            Ok(secrets)
        }
    }
}

fn save_secrets(secrets: &Vec<Secret>) -> Result<(), Box<dyn Error>> {
    let contents = secrets.iter().map(|f|format!("{}\n", f)).collect::<Vec<String>>().join("");
    let path = file_path();

    let mut file;
    if !path.exists() {
        file = File::create(path)?;
    } else {
        file = File::options().write(true).truncate(true).open(path)?;
    }

    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn file_path() -> PathBuf {
    #![allow(deprecated)]
    match env::home_dir() {
        Some(path) => path.join(Path::new(DEFAULT_FILE)),
        None =>  Path::new(DEFAULT_FILE).to_path_buf(),
    }
}
