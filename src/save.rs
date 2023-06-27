use std::io::{ self, Read, Write };
use std::fs::File;

use crate::common::locations::*;
use crate::common::{ CharacterSlot };

fn read_u8(filebuffer: &Vec<u8>, address: usize) -> u8
{
    filebuffer[address]
}

fn read_u16(filebuffer: &Vec<u8>, address: usize) -> u16
{
    ((filebuffer[address + 0] as u16) << 8) |
     (filebuffer[address + 1] as u16)
}

fn read_u32(filebuffer: &Vec<u8>, address: usize) -> u32
{
    ((filebuffer[address + 0] as u32) << 24) |
    ((filebuffer[address + 1] as u32) << 16) |
    ((filebuffer[address + 2] as u32) << 8)  |
     (filebuffer[address + 3] as u32)
}

fn read_name(filebuffer: &Vec<u8>, address: usize) -> [u8; 8]
{
    let segment = &filebuffer[address..address+8];
    let array: Result<[u8; 8], _> = segment.try_into();

    match array
    {
        Ok(fixed_array) => return fixed_array,
        Err(_) =>
        {
            eprintln!("Conversion failed");
            return b"ERROR!!!".to_owned()
        }
    }
}

fn write_u8(filebuffer: &mut Vec<u8>, address: usize, data: u8)
{
    filebuffer[address] = data;
}

fn write_u16(filebuffer: &mut Vec<u8>, address: usize, data: u16)
{
    filebuffer[address    ] = ((data >> 8) & 0xFF) as u8;
    filebuffer[address + 1] = ((data)      & 0xFF) as u8;
}

fn write_u32(filebuffer: &mut Vec<u8>, address: usize, data: u32)
{
    filebuffer[address    ] = ((data >> 24) & 0xFF) as u8;
    filebuffer[address + 1] = ((data >> 16) & 0xFF) as u8;
    filebuffer[address + 2] = ((data >> 8)  & 0xFF) as u8;
    filebuffer[address + 3] = ((data)       & 0xFF) as u8;
}

fn write_name(filebuffer: &mut Vec<u8>, address: usize, str: [u8; 8])
{
    for k in 0..8
    {
        filebuffer[address + k] = str[k];
    }
}

pub fn file_to_buf(filepath: &String) -> io::Result<Vec<u8>>
{
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn buf_to_save(src: &Vec<u8>, dest: &mut CharacterSlot, slot_n: usize)
{
    dest.gender.data    = read_u8(src,  SLOTS[slot_n] + dest.gender.offset  );
    dest.name.data      = read_name(src, SLOTS[slot_n] + dest.name.offset    );
    dest.zenny.data     = read_u32(src, SLOTS[slot_n] + dest.zenny.offset   );
    dest.playtime.data  = read_u32(src, SLOTS[slot_n] + dest.playtime.offset);
    for k in 0..dest.b_pouch.data.len()
    {
        dest.b_pouch.data[k].0 = read_u16(src, SLOTS[slot_n] + dest.b_pouch.offset + (k * 4));
        dest.b_pouch.data[k].1 = read_u16(src, SLOTS[slot_n] + dest.b_pouch.offset + (k * 4) + 2) as i16;
    }
    for k in 0..dest.g_pouch.data.len()
    {
        dest.g_pouch.data[k].0 = read_u16(src, SLOTS[slot_n] + dest.g_pouch.offset + (k * 4));
        dest.g_pouch.data[k].1 = read_u16(src, SLOTS[slot_n] + dest.g_pouch.offset + (k * 4) + 2) as i16;
    }
    for k in 0..dest.item_box.data.len()
    {
        dest.item_box.data[k].0 = read_u16(src, SLOTS[slot_n] + dest.item_box.offset + (k * 4));
        dest.item_box.data[k].1 = read_u16(src, SLOTS[slot_n] + dest.item_box.offset + (k * 4) + 2) as i16;
    }
    return
}

pub fn save_to_buf(src: &CharacterSlot, dest: &mut Vec<u8>, slot_n: usize)
{
    write_u8(dest,  SLOTS[slot_n] + src.gender.offset  , src.gender.data     );
    write_name(dest, SLOTS[slot_n] + src.name.offset    , src.name.data   );
    write_u32(dest, SLOTS[slot_n] + src.zenny.offset   , src.zenny.data      );
    write_u32(dest, SLOTS[slot_n] + src.playtime.offset, src.playtime.data   );
    for k in 0..src.b_pouch.data.len()
    {
        write_u16(dest, SLOTS[slot_n] + src.b_pouch.offset + (k * 4), src.b_pouch.data[k].0);
        write_u16(dest, SLOTS[slot_n] + src.b_pouch.offset + (k * 4) + 2, src.b_pouch.data[k].1 as u16);
    }
    for k in 0..src.g_pouch.data.len()
    {
        write_u16(dest, SLOTS[slot_n] + src.g_pouch.offset + (k * 4), src.g_pouch.data[k].0);
        write_u16(dest, SLOTS[slot_n] + src.g_pouch.offset + (k * 4) + 2, src.g_pouch.data[k].1 as u16);
    }
    for k in 0..src.item_box.data.len()
    {
        write_u16(dest, SLOTS[slot_n] + src.item_box.offset + (k * 4), src.item_box.data[k].0);
        write_u16(dest, SLOTS[slot_n] + src.item_box.offset + (k * 4) + 2, src.item_box.data[k].1 as u16);
    }
}

pub fn buf_to_file(filepath: &String, buffer: &mut Vec<u8>) -> io::Result<()>
{
    let checksum: u32 = buffer
        .iter()
        .skip(8)
        .map(|&x| x as u32)
        .sum();
    println!("Checksum: 0x{:08X}", checksum);

    write_u32(buffer, CHECKSUM, checksum);

    let mut file = File::create(filepath)?;
    file.write_all(buffer.as_slice())?;
    Ok(())
}