use std::process;
use std::env;

mod save;
mod csv;
mod common;

use crate::common::DataIDs;
use crate::save::{ file_to_buf, buf_to_save, save_to_buf, buf_to_file };
use crate::csv::{ save_to_csv, csv_to_file, file_to_csv, csv_to_save };
use common::CharacterSlot;

fn main()
{
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("mh3se v{} | mh3 save-editor", VERSION);

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4
    {
        eprintln!("Not enough or too much arguments");
        process::exit(1);
    }

    let ids: DataIDs;
    match DataIDs::new(&String::from("genders.txt"), &String::from("items.txt"))
    {
        Ok(res) => ids = res,
        Err(err) =>
        {
            eprintln!("Couldn't access data files {}", err);
            process::exit(1);
        }
    }

    if args[0] == "decode"
    {
        let savepath = &args[1];
        let csvpath = &args[2];
        let character_slot: usize;

        match args[3].parse::<usize>()
        {
            Ok(number) =>
            {
                character_slot = number - 1;
                println!("Slot {} selected", character_slot  + 1);
            },
            Err(err) =>
            {
                eprintln!("Error {}", err);
                process::exit(1);
            }
        }

        let buffer: Vec<u8>;
        match file_to_buf(&String::from(savepath))
        {
            Ok(buf) =>
            {
                buffer = buf;
                println!("{} loaded", savepath);
            },
            Err(err) =>
            {
                eprintln!("Error loading file: {}", err);
                process::exit(1);
            }
        }
        let mut slot: CharacterSlot = CharacterSlot::default();
        let mut csv: Vec<String> = Vec::new();
        buf_to_save(&buffer, &mut slot, character_slot);
        save_to_csv(&slot, &mut csv, &ids);
        match csv_to_file(&String::from(csvpath), &csv)
        {
            Ok(_) =>
            {
                println!("data written to {}", csvpath)
            },
            Err(err) =>
            {
                eprintln!("Error writing file: {}", err);
                process::exit(1);
            }
        }

    }
    else if args[0] == "encode"
    {
        let savepath = &args[1];
        let csvpath = &args[2];
        let character_slot: usize;

        match args[3].parse::<usize>()
        {
            Ok(number) =>
            {
                character_slot = number - 1;
                println!("Slot {} selected", character_slot + 1);
            },
            Err(err) =>
            {
                eprintln!("Error {}", err);
                process::exit(1);
            }
        }

        let csv: Vec<String>;
        match file_to_csv(csvpath)
        {
            Ok(res) =>
            {
                csv = res;
                println!("{} loaded", csvpath);
            }
            Err(err) =>
            {
                eprintln!("Error reading file: {}", err);
                process::exit(1);
            }
        }

        let mut slot: CharacterSlot = CharacterSlot::default();
        let _ = csv_to_save(&csv, &mut slot, &ids);

        let mut buffer: Vec<u8>;
        match file_to_buf(savepath)
        {
            Ok(buf) =>
            {
                buffer = buf;
                println!("{} loaded", savepath);
            },
            Err(err) =>
            {
                eprintln!("Error loading file: {}", err);
                process::exit(1);
            }
        }
        save_to_buf(&slot, &mut buffer, character_slot);
        match buf_to_file(savepath, &mut buffer)
        {
            Ok(_) =>
            {
                println!("data written to {}", savepath);
            }
            Err(err) =>
            {
                eprintln!("Error writing file: {}", err);
                process::exit(1);
            }
        }
    }
    else
    {
        eprintln!("Unknown action");
        process::exit(1);
    }

    process::exit(0);
}