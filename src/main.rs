use std::io;
use std::env;
use std::fs;
use std::path;
use std::io::prelude::*;

struct Config {
    action: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let action = args[1].clone();
    let filename = args[2].clone();

    Config { action, filename  }
}

fn get_shifts() -> u8 {
    let mut shifts = String::new();
    println!("Enter number of shifts: ");
    io::stdin().read_line(&mut shifts).expect("Unable to read user input.");

    shifts.trim().parse::<u8>().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(&config.filename).expect("Something went wrong reading the file");
    println!("Action to be performed: {}", &config.action);

    let shifts = get_shifts();

    // For encryption
    if &config.action == "encrypt" {
        println!("File contents are being encrypted...");
        let enc_file: Vec<u8> = contents.bytes().map(|byte| byte + &shifts).collect();
        
        // Writing to file the contents of encrypted file
        let mut new_filename: String = "enc_".to_owned();
        new_filename.push_str(&config.filename);
        let path = path::Path::new(&new_filename);
        let display = path.display();
        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        match file.write_all(&enc_file) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => println!("Encryption successfully done! Please look for a file named {}", display),
        }
        

    } 
    // For decryption
    else if &config.action == "decrypt" {
        println!("File contents are being depcrypted...");
        let dec_file: Vec<u8> = contents.bytes().map(|byte| byte - &shifts).collect();
        let mut new_filename: String = "dec_".to_owned();
        new_filename.push_str(&config.filename);
        let path = path::Path::new(&new_filename);
        let display = path.display();
        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        match file.write_all(&dec_file) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => println!("Decryption succesfully done! Please look for a file named {}", display),
        }
    }
}
