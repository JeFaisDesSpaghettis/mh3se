use std::env;
use std::process;

mod common;
mod save_read;
mod csv_read;
mod save_write;
mod csv_write;

use crate::common::{ DataIDs, CharacterSlot };
use crate::save_read::{ file_to_buf, buf_to_save };
use crate::csv_read::{ file_to_csv, csv_to_save };
use crate::csv_write::{ save_to_csv, csv_to_file, };
use crate::save_write::{ save_to_buf, buf_to_file };

fn main()
{
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("mh3se v{} | mh3 save-editor", VERSION);

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        eprintln!("Not enough or too much arguments !");
        process::exit(1);
    }

    let ids: DataIDs;
    match DataIDs::new(
        &String::from("data/genders.txt"),
        &String::from("data/items.txt"),
        &String::from("data/armors.txt"),
        &String::from("data/weapons.txt"),
        &String::from("data/gun_parts.txt"),
        &String::from("data/skills.txt")
    )
    {
        Ok(res) => {
            ids = res;
            println!("Loaded database files successfully !");
        }
        Err(err) => {
            eprintln!("Couldn't load database files {} !", err);
            process::exit(1);
        }
    }

    let save_path = &args[1];
    let csv_path = &args[2];
    let character_slot: usize;

    match args[3].parse::<usize>() {
        Ok(number) => {
            character_slot = number - 1;
            println!("Slot {} selected !", character_slot + 1);
        },
        Err(err) => {
            eprintln!("Couldn't select requested slot {} !", err);
            process::exit(1);
        }
    }

    if args[0] == "decode"
    {
        let buffer: Vec<u8>;
        match file_to_buf(&String::from(save_path)) {
            Ok(buf) => {
                buffer = buf;
                println!("Loaded save file {} successfully !", save_path);
            },
            Err(err) => {
                eprintln!("Couldn't load save file {} ({}) !", save_path, err);
                process::exit(1);
            }
        }
        let mut slot: CharacterSlot = CharacterSlot::default();
        let mut csv: Vec<String> = Vec::new();
        buf_to_save(&buffer, &mut slot, character_slot);
        save_to_csv(&slot, &mut csv, &ids);
        match csv_to_file(&String::from(csv_path), &csv) {
            Ok(_) => {
                println!("Data written to csv file {} successfully !", csv_path);
            },
            Err(err) => {
                eprintln!("Couldn't write data to csv file {} ({}) !", csv_path, err);
                process::exit(1);
            }
        }

    }
    else if args[0] == "encode"
    {
        let csv: Vec<String>;
        match file_to_csv(csv_path) {
            Ok(res) => {
                csv = res;
                println!("Loaded csv file {} successfully !", csv_path);
            }
            Err(err) => {
                eprintln!("Couldn't load csv file {} ({}) !", csv_path, err);
                process::exit(1);
            }
        }

        let mut slot: CharacterSlot = CharacterSlot::default();
        match csv_to_save(&csv, &mut slot, &ids) {
            Ok(_) => println!("Parsed csv file {} successfully !", csv_path),
            Err(err) => {
                eprintln!("Couldn't parse csv file {} ({})", csv_path, err);
                process::exit(1);
            }
        }

        let mut buffer: Vec<u8>;
        match file_to_buf(save_path) {
            Ok(buf) => {
                buffer = buf;
                println!("Loaded save file {} successfully !", save_path);
            },
            Err(err) => {
                eprintln!("Couldn't load save file {} ({}) !", save_path, err);
                process::exit(1);
            }
        }
        save_to_buf(&slot, &mut buffer, character_slot);
        match buf_to_file(save_path, &mut buffer) {
            Ok(_) => {
                println!("Data written to save file {} successfully !", save_path);
            },
            Err(err) => {
                eprintln!("Couldn't write data to save file {} ({}) !", save_path, err);
                process::exit(1);
            }
        }
    }
    else {
        eprintln!("Invalid arguments !");
        process::exit(1);
    }
    process::exit(0);
}