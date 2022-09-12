mod secret;
mod error;
mod store;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use clap::{Parser, Subcommand};
use rpassword::read_password;
use crate::secret::Secret;
use crate::store::Store;

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
        /// Watch the value
        #[clap(short, long)]
        watch: bool
    },
    /// Change the pin on the secret store
    Pin {},
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let command: &Commands = &cli.command;

    match command {
        Commands::List {} => list_secrets(),
        Commands::Add { name } => add_secret(name),
        Commands::Remove { name } => remove_secret(name),
        Commands::Show { name, watch } => show_secret(name, watch),
        Commands::Pin {} => change_pin(),
    }
}

fn change_pin() -> Result<(), Box<dyn Error>> {
    let store = open_secret_store()?;
    print!("New ");
    let (store, _) = store.upgrade(get_pw, true);
    save_secrets(store)
}

fn add_secret(name: &String) -> Result<(), Box<dyn Error>> {
    println!("Secret: ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.retain(|c| !c.is_whitespace());

    let mut store = open_secret_store()?;
    let secrets = store.secrets_mut();
    secrets.push(Secret::new(name.clone(), line.trim().to_string()));
    save_secrets(store)
}

fn remove_secret(name: &String) -> Result<(), Box<dyn Error>> {
    let mut store = open_secret_store()?;
    let secrets = store.secrets_mut();
    let index = secrets.iter().position(|s|s.name==*name).unwrap();
    secrets.remove(index);
    save_secrets(store)
}

fn list_secrets() -> Result<(), Box<dyn Error>> {
    for secret in open_secret_store()?.secrets() {
        println!("{}", secret.name)
    }
    Ok(())
}

fn show_secret(name: &str, watch: &bool) -> Result<(), Box<dyn Error>> {
    let store = open_secret_store()?;
    if let Some(secret) = store.secrets().iter().find(|s|s.name==name) {
        let mut value = secret.generate()?;
        println!("{value}");
        if *watch {
            loop {
                sleep(Duration::from_secs(1));
                let new_value = secret.generate()?;
                if value != new_value {
                    value = new_value;
                    println!("{value}");
                }
            }
        }
        return Ok(());
    }
    Err(Box::new(error::Error::from("no secret found")))
}

fn open_secret_store() -> Result<Store, Box<dyn Error>> {
    let path = file_path();
    let file = File::open(path);
    let store = match file {
        Err(std::io::Error{ .. }) => Store::new(),
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Store::from_str(&s, get_pw)?
        }
    };

    let (store, upgraded) = store.upgrade(get_pw, false);
    if upgraded {
        save_secrets(store.clone())?;
    }
    Ok(store)
}

fn save_secrets(store: Store) -> Result<(), Box<dyn Error>> {
    let contents = store.to_string();
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

fn get_pw() -> String {
    print!("PIN: ");
    std::io::stdout().flush().unwrap();
    let pin = read_password().unwrap();
    return pin + "\n";
}

fn file_path() -> PathBuf {
    #![allow(deprecated)]
    match env::home_dir() {
        Some(path) => path.join(Path::new(DEFAULT_FILE)),
        None =>  Path::new(DEFAULT_FILE).to_path_buf(),
    }
}
