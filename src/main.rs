use std::{
    fs::OpenOptions,
    io::{Result, Write},
};

use random_passwords::anagram::{print_banner, FILE_PATH, generate_passwords};

// use dialoguer::Password;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    print_banner();
    let passwords = generate_passwords();
    let passwords_count = passwords.len();

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

    // // get user input
    // let input = Password::new().with_prompt(" Type a password").interact()?;

    // // get all anagrams for the given password
    // let anagrams = get_anagrams(&input);
    // let anagrams_count = anagrams.len();

    // open the file "passwords.txt" with read and write permissions
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Error opening file...");

    // write and save the generated passwords to the file
    match file.write_all(passwords.join("\n").as_bytes()) {
        Ok(()) => Ok(info!("{passwords_count} passwords saved in file {FILE_PATH}")),
        Err(_) => Ok(error!("error while saving the generated passwords to file")),
    }
}
