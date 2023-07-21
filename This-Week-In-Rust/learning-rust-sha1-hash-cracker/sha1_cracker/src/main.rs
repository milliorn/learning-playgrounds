//  SHA-1 cracker
use sha1::Digest;

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    println!("sha1_cracker demo");

    // read the command line
    // Where std::env imports the module env from the standard library and env::args()
    // calls the args function from this module and returns an iterator which can be "collected" into a Vec<String>,
    // a Vector of String objects.
    let args: Vec<String> = env::args().collect();

    // check for the number of arguments and display an error message if it does not match what is expected.
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    // error handling
    let hash_to_crack = args[2].trim();

    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    // reading files
    let wordlist_file = File::open(&args[1])?;
    // we need to reduce the number of SHA-1 hashes generated.
    // using a special kind of dictionary, known as a wordlist,
    // which contains the most common password found in breached websites.
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }
    println!("password not found in wordlist :(");

    Ok(())
}
