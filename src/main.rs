/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt

To decrypt:

cargo run --  --message "{base64 input}" --decrypt

*/
#![allow(deprecated)]

use aes_cbc::{decrypt, encrypt};
use clap::Parser;
use rustc_serialize::base64::{FromBase64, ToBase64, STANDARD};
use std::fs::OpenOptions;
use std::io::{Result, Write};
use std::time::Instant;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// The message to encrypt or decrypt
    #[arg(short, long)]
    message: String,

    /// The key to use for cipher
    #[arg(
        short,
        long,
        default_value = "11, 240, 187, 101, 86, 31, 106, 213, 158, 232, 52, 243, 245, 149, 250, 129, 249, 152, 173, 1, 82, 169, 242, 238, 28, 88, 67, 186, 127, 88, 129, 245"
    )]
    key: String,

    /// The iv to use for cipher
    #[arg(
        short,
        long,
        default_value = "171, 250, 33, 152, 5, 58, 106, 206, 82, 13, 91, 144, 70, 67, 203, 91"
    )]
    iv: String,
}

fn append_to_md_file(
    file_name: &str,
    message: &str,
    encrypt: bool,
    content: &str,
    time: &u128,
    mem: &u64,
) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;

    let mut file = std::io::BufWriter::new(file);
    let e_d = if encrypt { "encryption" } else { "decryption" };
    writeln!(
        file,
        "\nThe orginal message to the cipher is {}\n\n",
        message
    )?;
    writeln!(file, "The result of the {} is {}\n\n", e_d, content)?;
    writeln!(file, "Elapsed time: {} microseconds\n\n", time)?;
    writeln!(file, "Memory used: {} kb\n\n", mem)?;

    println!("Content appended to {} successfully!", file_name);

    Ok(())
}

// run it
fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let mem_info_before = sys_info::mem_info().unwrap();

    let key: [u8; 32] = (&args.key)
        .split(',')
        .map(|s| s.trim().parse().expect(s))
        .collect::<Vec<u8>>()
        .try_into()
        .expect("Expected a string with exactly 32 u8 values separated by commas");
    let iv: [u8; 16] = (&args.iv)
        .split(',')
        .map(|s| s.trim().parse().expect("Failed to parse u8"))
        .collect::<Vec<u8>>()
        .try_into()
        .expect("Expected a string with exactly 16 u8 values separated by commas");
    println!("key {}", key.to_base64(STANDARD));
    println!("iv {}", iv.to_base64(STANDARD));

    if args.encrypt {
        let encoded = encrypt((&args.message).as_bytes(), &key, &iv).ok().unwrap();
        println!("{}", encoded.to_base64(STANDARD));
        let encoded_string: String = encoded.to_base64(STANDARD);
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time);
        let mem_info_after = sys_info::mem_info().unwrap();
        let mem_used = mem_info_after.total - mem_info_before.total;
        match append_to_md_file(
            "rust_times.md",
            &args.message,
            args.encrypt,
            &encoded_string,
            &elapsed_time.as_micros(),
            &mem_used,
        ) {
            Ok(_) => {}
            Err(e) => println!("Error: {:?}", e),
        }
    } else if args.decrypt {
        let decoded_bytes = match (&args.message).from_base64() {
            Ok(decoded_bytes) => decoded_bytes,
            Err(err) => {
                println!("Error decoding Base64: {:?}", err);
                return; // Exit or handle the error as needed.
            }
        };
        let decoded = decrypt(&decoded_bytes, &key, &iv).ok().unwrap();
        let decoded_string: String = decoded.iter().map(|&d| d as u8 as char).collect();
        println!("{}", decoded_string);
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time);
        let mem_info_after = sys_info::mem_info().unwrap();
        let mem_used = mem_info_after.total - mem_info_before.total;

        match append_to_md_file(
            "rust_times.md",
            &args.message,
            args.encrypt,
            &decoded_string,
            &elapsed_time.as_micros(),
            &mem_used,
        ) {
            Ok(_) => {}
            Err(e) => println!("Error: {:?}", e),
        }
    } else {
        println!("Please specify either --encrypt or --decrypt");
    }
}
