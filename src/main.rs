use std::{
    fs::OpenOptions,
    io::{Result, Write},
};

use dialoguer::Password;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn get_anagrams(word: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars: Vec<char> = word.chars().collect();
    let length = chars.len();
    info!("generating all possible passwords");
    generate_anagrams(length, &mut chars, &mut result);

    result
}

fn generate_anagrams(length: usize, chars: &mut [char], result: &mut Vec<String>) {
    if length == 1 {
        result.push(chars.iter().collect());
        return;
    }

    for i in 0..length {
        generate_anagrams(length - 1, chars, result);
        if length % 2 == 0 {
            chars.swap(i, length - 1);
        } else {
            chars.swap(0, length - 1);
        }
    }

    result.sort();
    result.dedup()
}

fn main() -> Result<()> {
    const BANNER: &str = r#"
     _                           _                             
    |_)  _. ._   _|  _  ._ _    |_) _.  _  _       _  ._ _|  _ 
    | \ (_| | | (_| (_) | | |   |  (_| _> _> \/\/ (_) | (_| _> 
    "#;

    println!("{BANNER}");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("random password generator started succesfully");

    let input = Password::new()
        .with_prompt("Type a password")
        .interact()?;

    info!("password recived");

    let anagrams = get_anagrams(&input);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./passwords.txt")
        .expect("Error opening file...");

    info!("password file created");

    file.write_all(anagrams.join("\n").as_bytes()).expect("msg");

    info!("passwords saved");

    Ok(())
}