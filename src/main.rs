use std::{
    fs::OpenOptions,
    io::{Result, Write},
};

use random_passwords::anagram::{get_anagrams, BANNER};

use dialoguer::Password;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

const FILE_PATH: &str = "./passwords.txt";

fn main() -> Result<()> {
    println!("{BANNER}");

    // build and init the logger
    match FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .compact()
        .without_time()
        .try_init()
    {
        Ok(()) => info!("random password generator started succesfully"),
        Err(_) => println!("error while creating logger"),
    };

    // get user input
    let input = Password::new().with_prompt(" Type a password").interact()?;

    // get all anagrams for the given password
    let anagrams = get_anagrams(&input);

    // open the file "passwords.txt" with read and write permissions
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Error opening file...");

    // write and save the generated passwords to the file
    match file.write_all(anagrams.join("\n").as_bytes()) {
        Ok(()) => Ok(info!("passwords saved in file {FILE_PATH}")),
        Err(_) => Ok(error!("error while saving the generated passwords to file")),
    }
}
