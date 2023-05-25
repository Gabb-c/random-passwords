use std::{
    fs::OpenOptions,
    io::{Result, Write},
};

use random_passwords::anagram::get_anagrams;

use dialoguer::Password;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

const FILE_PATH: &str = "./passwords.txt";
const BANNER: &str = r#"
     _                           _                             
    |_)  _. ._   _|  _  ._ _    |_) _.  _  _       _  ._ _|  _ 
    | \ (_| | | (_| (_) | | |   |  (_| _> _> \/\/ (_) | (_| _> 
    "#;

fn main() -> Result<()> {
    println!("{BANNER}");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("random password generator started succesfully");

    let input = Password::new().with_prompt("Type a password").interact()?;

    info!("password recived");

    let anagrams = get_anagrams(&input);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Error opening file...");

    info!("password file created");

    file.write_all(anagrams.join("\n").as_bytes()).expect("msg");

    info!("passwords saved");

    Ok(())
}
