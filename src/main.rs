extern crate sha1;
extern crate strsim;

use crate::{sha1::Digest, strsim::normalized_levenshtein};

use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

// declare a constant
const SHA1_HASH_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let mut maximum_value: i32 = 0;

    // hashmap to store passwords values
    let mut passwords: HashMap<i32, String> = HashMap::new();

    // collect command-line arguments and store to a vector of strings
    let args: Vec<String> = env::args().collect();

    // check if number of passed elements is not equal to 3..
    if args.len() != 3 {
        println!("Usage: ");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");

        // exit program
        return Ok(());
    }

    // index the hash to crack
    let hash_to_crack = args[2].trim();

    // check if it's a valid hash
    if hash_to_crack.len() != SHA1_HASH_STRING_LENGTH {
        return Err("SHA1 hash is not valid".into());
    }

    // read wordlist file from local storage
    let reader = BufReader::new(File::open(&args[1])?);

    // iterate through all lines
    for line in reader.lines() {
        // extract text from variant
        let line = line?;

        // remove white spaces, compute hash and encode to hex
        let common_password = hex::encode(sha1::Sha1::digest(line.trim().as_bytes()));

        // calculate simiarity between both hashes, round and save as integer
        let similarity: i32 =
            (normalized_levenshtein(hash_to_crack, &common_password) * 100.0).round() as i32;

        // printing is a performance overhead
        // println!("Trying {line} => {common_password}\n\tSimilarity metric: {similarity}%",);

        // store computed similarity and original password
        // string was used due to an error with hashmaps and f64s
        passwords.insert(similarity, line.to_string());

        // collect all `passwords` keys to a vector
        let similarities: Vec<&i32> = passwords.keys().collect::<Vec<&i32>>();

        maximum_value = **(similarities.iter().max().unwrap());

        // do the checking
        match hash_to_crack == common_password {
            true => {
                println!("Password found: {}", line);
                return Ok(());
            }
            false => {}
        }
    }

    println!(
        "Password not found in wordlist :(\n\tMost similar password: {:?}. Similarity: {maximum_value}%",
        passwords.get(&maximum_value).unwrap(),
    );

    Ok(())
}
