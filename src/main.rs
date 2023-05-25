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

    match FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .compact()
        .without_time()
        .try_init()
    {
        Ok(()) => info!("random password generator started succesfully"),
        Err(_) => println!("error while creating logger"),
    };

    let input = Password::new().with_prompt(" Type a password").interact()?;

    let anagrams = get_anagrams(&input);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Error opening file...");

    file.write_all(anagrams.join("\n").as_bytes()).expect("msg");

    info!("passwords saved in file {FILE_PATH}");

    Ok(())
}
