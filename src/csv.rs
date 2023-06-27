use std::io::{ self, Write };
use std::fs::File;

use crate::common::{ CharacterSlot, DataIDs, read_lines };

fn str_to_id(string: &String, list: &Vec<String>, mut offset: usize) -> Option<usize>
{
    for line in list
    {
        if string == line
        {
            return Some(offset);
        }
        offset += 1;
    }
    return None
}

fn id_to_str(list: &Vec<String>, offset: usize) -> &String
{
    return &list[offset]
}

pub fn save_to_csv(src: &CharacterSlot, dest: &mut Vec<String>, ids: &DataIDs)
{
    dest.push(format!("{}, {}", src.gender.name, id_to_str(&ids.gender_list, src.gender.data as usize)));
    let mut name: String = String::from("STUBSTUB");
    match String::from_utf8(src.name.data.as_slice().to_vec())
    {
        Ok(res) => name = res,
        Err(err) =>
        {
            eprintln!("{}", err);
            eprintln!("Replacing name with a placeholder...");
        }
    }
    dest.push(format!("{}, {}", src.name.name, name));
    dest.push(format!("{}, {}", src.zenny.name, src.zenny.data));
    dest.push(format!("{}, {}", src.playtime.name, src.playtime.data));
    dest.push(format!("{}, {}", src.hrp.name, src.hrp.data));
    dest.push(format!("{}, {}", src.hr.name, src.hr.data));
    for k in 0..src.b_pouch.data.len()
    {
        dest.push(format!("{}_{}, {}, {}",
            src.b_pouch.name,
            k,
            id_to_str(&ids.items_list, src.b_pouch.data[k].0 as usize),
            src.b_pouch.data[k].1));
    }
    for k in 0..src.g_pouch.data.len()
    {
        dest.push(format!("{}_{}, {}, {}",
            src.g_pouch.name,
            k,
            id_to_str(&ids.items_list, src.g_pouch.data[k].0 as usize),
            src.g_pouch.data[k].1));
    }
    for k in 0..src.item_box.data.len()
    {
        dest.push(format!("{}_{}, {}, {}",
            src.item_box.name,
            k,
            id_to_str(&ids.items_list, src.item_box.data[k].0 as usize),
            src.item_box.data[k].1));
    }
}

pub fn csv_to_file(filepath: &String, csv: &Vec<String>) -> io::Result<()>
{
    let mut file = File::create(filepath)?;
    writeln!(file, "NAME, DATA0, DATA1")?;
    for line in csv
    {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

pub fn file_to_csv(filepath: &String) -> io::Result<Vec<String>>
{
    read_lines(filepath)
}

pub fn csv_to_items_list(name: &String, dest: &mut[(u16, i16)], parts: &Vec<String>, ids: &DataIDs) -> Result<(), std::num::ParseIntError>
{
    let index = parts[0][(name.len() + 1)..].parse::<usize>()?;

    match str_to_id(&parts[1], &ids.items_list, 0)
    {
        Some(res) =>
        {
            dest[index as usize].0 = res as u16;
            dest[index as usize].1 = parts[2].parse::<i16>()?;
        }
        None =>
        {
            eprintln!("Invalid item");
            return Ok(());
        }
    }
    Ok(())
}

pub fn csv_to_save(csv: &Vec<String>, dest: &mut CharacterSlot, ids: &DataIDs) -> Result<usize, std::num::ParseIntError>
{
    let mut linecount: usize = 0;
    for line in csv
    {
        linecount += 1;
        let parts: Vec<String> = line
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        if parts[0] == dest.gender.name && parts.len() == 2
        {
            match str_to_id(&parts[1], &ids.gender_list, 0)
            {
                Some(res) => dest.gender.data = res as u8,
                None =>
                {
                    eprintln!("Invalid gender (line {})", linecount);
                    return Ok(linecount);
                }
            }
        }
        else if parts[0] == dest.name.name && parts.len() == 2
        {
            let bytes = parts[1].as_bytes();
            let length = std::cmp::min(bytes.len(), 8);
            dest.name.data[..length].copy_from_slice(&bytes[..length]);
        }
        else if parts[0] == dest.zenny.name && parts.len() == 2
        {
            dest.zenny.data = parts[1].parse::<u32>()?;
        }
        else if parts[0] == dest.playtime.name && parts.len() == 2
        {
            dest.playtime.data = parts[1].parse::<u32>()?;
        }
        else if parts[0] == dest.hrp.name && parts.len() == 2
        {
            dest.hrp.data = parts[1].parse::<u32>()?;
        }
        else if parts[0] == dest.hr.name && parts.len() == 2
        {
            dest.hr.data = parts[1].parse::<u16>()?;
        }
        else if parts[0].starts_with(dest.b_pouch.name.as_str()) && parts.len() == 3
        {
            csv_to_items_list(&dest.b_pouch.name, &mut dest.b_pouch.data, &parts, &ids)?;
        }
        else if parts[0].starts_with(dest.g_pouch.name.as_str()) && parts.len() == 3
        {
            csv_to_items_list(&dest.g_pouch.name, &mut dest.g_pouch.data, &parts, &ids)?;
        }
        else if parts[0].starts_with(dest.item_box.name.as_str()) && parts.len() == 3
        {
            csv_to_items_list(&dest.item_box.name, &mut dest.item_box.data, &parts, &ids)?;
        }
        else if parts[0] == "NAME" {}
        else
        {
            eprintln!("Invalid entry in line {}", linecount);
            return Ok(linecount);
        }
    }
    Ok(linecount)
}
