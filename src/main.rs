use crate::common::CharacterSlot;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::env;
use std::process;

mod common;
mod save_read;
mod save_write;

fn json_to_save(json_path: &str, base_save_path: &str, output_save_path: &str, nth_slot: usize) -> Result<(), String>
{
    let mut json_file: File;
    match File::open(json_path) {
        Ok(f) => json_file = f,
        Err(e) => return Err(format!("Failed to open {} ({})", json_path, e)),
    };

    let mut json_data = String::new();
    match json_file.read_to_string(&mut json_data) {
        Ok(_) => {},
        Err(e) => return Err(format!("Failed to read {} ({})", json_path, e)),
    };

    let slot: CharacterSlot;
    match serde_json::from_str(&json_data) {
        Ok(s) => {
            slot = s;
            println!("Parsed {} successfully", json_path);
        },
        Err(e) => return Err(format!("Failed to parse {} ({})", json_path, e)),
    };
    let mut buffer: [u8; common::SAVE_SIZE];
    match save_read::file_to_buf(base_save_path) {
        Ok(b) => {
            buffer = b;
        },
        Err(e) => return Err(format!("Failed to read {} ({})", base_save_path, e)),
    }

    match save_write::write_slot(&mut buffer, &slot, nth_slot) {
        Ok(_) => println!("Wrote slot {} successfully", nth_slot),
        Err(e) => return Err(format!("Failed to write slot {} ({})", nth_slot, e)),
    };

    match save_write::buf_to_file(output_save_path, &mut buffer) {
        Ok(_) => println!("Wrote {} successfully", output_save_path),
        Err(e) => return Err(format!("Failed to write {} ({})", output_save_path, e)),
    }
    Ok(())
}

fn save_to_json(save_path: &str, json_path: &str, nth_slot: usize) -> Result<(), String>
{
    let buffer: [u8; common::SAVE_SIZE];
    match save_read::file_to_buf(save_path) {
        Ok(b) => {
            buffer = b;
        },
        Err(e) => return Err(format!("Couldn't read save file {} ({}) !", save_path, e)),
    };

    let slot: CharacterSlot;
    match common::CharacterSlot::from_buf(&buffer, nth_slot) {
        Ok(s) => {
            slot = s;
            println!("Successfully parsed {}", save_path);
        },
        Err(e) => return Err(format!("Couldn't parse {} ({})", save_path, e)),
    };

    let json_string: String;
    match serde_json::to_string_pretty(&slot) {
        Ok(s) => {
            json_string = s;
            println!("Successfully converted as json {}", save_path);
        },
        Err(e) => return Err(format!("Failed to convert as json {} ({})", save_path, e)),
    }

    let mut json_file;
    match File::create(json_path) {
        Ok(f) => json_file = f,
        Err(e) => return Err(format!("Failed to create {} ({})", json_path, e)),
    };

    match json_file.write_all(json_string.as_bytes()) {
        Ok(_) => println!("Successfully wrote json to {}", json_path),
        Err(e) => return Err(format!("Failed to write json to {} ({})", json_path, e)),
    };

    Ok(())
}

fn copy_slot(filepath: &str, origin_slot: usize, dest_slot: usize) -> Result<(), std::io::Error>
{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filepath)?;

    let mut buf = [0; common::SLOT_SIZE];
    file.seek(SeekFrom::Start((common::SLOT_OFFSET + (origin_slot * common::SLOT_SIZE)) as u64)).unwrap();
    file.read_exact(&mut buf)?;
    file.seek(SeekFrom::Start((common::SLOT_OFFSET + (dest_slot * common::SLOT_SIZE)) as u64)).unwrap();
    file.write_all(&buf)?;

    // Copy toggle flags
    let mut toggle_buf = [0; 3];
    file.seek(SeekFrom::Start((common::SLOT_TOGGLE_START + common::SLOT_SIZE) as u64)).unwrap();
    file.read_exact(&mut toggle_buf)?;

    toggle_buf[dest_slot] = toggle_buf[origin_slot];

    file.seek(SeekFrom::Start((common::SLOT_TOGGLE_START + common::SLOT_SIZE) as u64)).unwrap();
    file.write_all(&toggle_buf)?;
    file.flush()?;

    // Recompute checksum
    let mut full_buf = save_read::file_to_buf(filepath)?;
    save_write::buf_to_file(filepath, &mut full_buf)?;
    Ok(())
}

fn wipe_slot(filepath: &str, slot: usize) -> Result<(), std::io::Error>
{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filepath)?;

    let buf = [0; common::SLOT_SIZE];
    file.seek(SeekFrom::Start((common::SLOT_OFFSET + (slot * common::SLOT_SIZE)) as u64)).unwrap();
    file.write_all(&buf)?;

    // Copy toggle flags
    let mut toggle_buf = [0; 3];
    file.seek(SeekFrom::Start((common::SLOT_TOGGLE_START + common::SLOT_SIZE) as u64)).unwrap();
    file.read_exact(&mut toggle_buf)?;

    toggle_buf[slot] = 0;

    file.seek(SeekFrom::Start((common::SLOT_TOGGLE_START + common::SLOT_SIZE) as u64)).unwrap();
    file.write_all(&toggle_buf)?;
    file.flush()?;

    // Recompute checksum
    let mut full_buf = save_read::file_to_buf(filepath)?;
    save_write::buf_to_file(filepath, &mut full_buf)?;
    Ok(())
}

fn print_help()
{
    println!("Usage: ");
    println!("\tmh3se decode [save_file] [json_file] [slot]");
    println!("\tmh3se encode [json_file] [base_save_file] [output_save_file] [slot]");
    println!("\tmh3se copy [save_file] [origin_slot] [destination_slot]");
    println!("\tmh3se wipe [save_file] [slot]");
    println!("This program is licensed under GPLv3 terms.");
}

fn main()
{
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("mh3se v{} | mh3 save-editor", VERSION);
    println!("Project link: https://github.com/JeSuisSurGithub/mh3se");

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        print_help();
        process::exit(0);
    }

    if args[0] == "decode" {
        if args.len() != 4 {
            println!("Not enough arguments!");
            process::exit(1);
        }
        let save_path = args[1].as_str();
        let json_path = args[2].as_str();
        let slot: usize = args[3].parse().unwrap();
        if slot > 2 {
            println!("Slot ranges from 0 to 2!");
            process::exit(1);
        }
        save_to_json(save_path, json_path, slot).unwrap();
    } else if args[0] == "encode" {
        if args.len() != 5 {
            println!("Not enough arguments!");
            process::exit(1);
        }
        let json_path = args[1].as_str();
        let base_save_path = args[2].as_str();
        let output_save_path = args[3].as_str();
        let slot: usize = args[4].parse().unwrap();
        if slot > 2 {
            println!("Slot ranges from 0 to 2!");
            process::exit(1);
        }
        json_to_save(json_path, base_save_path, output_save_path, slot).unwrap();
    } else if args[0] == "copy" {
        if args.len() != 4 {
            println!("Not enough arguments!");
            process::exit(1);
        }
        let save_path = args[1].as_str();
        let src_slot: usize = args[2].parse().unwrap();
        let dst_slot: usize = args[3].parse().unwrap();
        if src_slot > 2 || dst_slot > 2 {
            println!("Slot ranges from 0 to 2!");
            process::exit(1);
        }
        copy_slot(save_path, src_slot, dst_slot).unwrap();
    } else if args[0] == "wipe" {
        if args.len() != 3 {
            println!("Not enough arguments!");
            process::exit(1);
        }
        let save_path = args[1].as_str();
        let slot: usize = args[2].parse().unwrap();
        if slot > 2 {
            println!("Slot ranges from 0 to 2!");
            process::exit(1);
        }
        wipe_slot(save_path, slot).unwrap();
    } else {
        print_help();
    }
    process::exit(0);
}