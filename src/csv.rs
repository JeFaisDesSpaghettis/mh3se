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
    for k in 0..src.b_pouch.data.len()
    {
        dest.push(format!("{}_{}, {}",
            src.b_pouch.name,
            k,
            id_to_str(&ids.items_list, src.b_pouch.data[k].0 as usize)));
        dest.push(format!("{}_qty_{}, {}",
            src.b_pouch.name,
            k,
            src.b_pouch.data[k].1));
    }
    for k in 0..src.g_pouch.data.len()
    {
        dest.push(format!("{}_{}, {}",
            src.g_pouch.name,
            k,
            id_to_str(&ids.items_list, src.g_pouch.data[k].0 as usize)));
        dest.push(format!("{}_qty_{}, {}",
            src.g_pouch.name,
            k,
            src.g_pouch.data[k].1));
    }
    for k in 0..src.item_box.data.len()
    {
        dest.push(format!("{}_{}, {}",
            src.item_box.name,
            k,
            id_to_str(&ids.items_list, src.item_box.data[k].0 as usize)));
        dest.push(format!("{}_qty_{}, {}",
            src.item_box.name,
            k,
            src.item_box.data[k].1));
    }
}

pub fn csv_to_file(filepath: &String, csv: &Vec<String>) -> io::Result<()>
{
    let mut file = File::create(filepath)?;
    writeln!(file, "NAME, DATA")?;
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

pub fn csv_to_save(csv: &Vec<String>, dest: &mut CharacterSlot, ids: &DataIDs) -> Result<usize, std::num::ParseIntError>
{
    let mut linecount: usize = 0;
    for line in csv
    {
        linecount += 1;
        let parts: Vec<&str> = line.splitn(2, ',').collect();

        if parts.len() != 2
        {
            eprintln!("Missing commas (line {})", linecount);
            return Ok(linecount);
        }

        let name: String = parts[0].trim().to_string();
        let data: String = parts[1].trim().to_string();

        if name == dest.gender.name
        {
            match str_to_id(&data, &ids.gender_list, 0)
            {
                Some(res) => dest.gender.data = res as u8,
                None =>
                {
                    eprintln!("Invalid gender (line {})", linecount);
                    return Ok(linecount);
                }
            }
        }
        else if name == dest.name.name
        {
            let bytes = data.as_bytes();
            let length = std::cmp::min(bytes.len(), 8);
            dest.name.data[..length].copy_from_slice(&bytes[..length]);
        }
        else if name == dest.zenny.name
        {
            dest.zenny.data = data.parse::<u32>()?;
        }
        else if name == dest.playtime.name
        {
            dest.playtime.data = data.parse::<u32>()?;
        }
        else if name.starts_with(dest.b_pouch.name.as_str())
        {
            let mut index_start = dest.b_pouch.name.len() + 1;
            let quantity = name.starts_with((dest.b_pouch.name.clone() + "_qty").as_str());
            if quantity { index_start += 4; }

            let index = name[index_start..].parse::<usize>()?;

            if !quantity
            {
                match str_to_id(&data, &ids.items_list, 0)
                {
                    Some(res) =>
                    {
                        dest.b_pouch.data[index as usize].0 = res as u16;
                    }
                    None =>
                    {
                        eprintln!("Invalid item (line {})", linecount);
                        return Ok(linecount);
                    }
                }
            }
            else
            {
                dest.b_pouch.data[index as usize].1 = data.parse::<i16>()?;
            }
        }
        else if name.starts_with(dest.g_pouch.name.as_str())
        {
            let mut index_start = dest.g_pouch.name.len() + 1;
            let quantity = name.starts_with((dest.g_pouch.name.clone() + "_qty").as_str());
            if quantity { index_start += 4; }

            let index = name[index_start..].parse::<usize>()?;

            if !quantity
            {
                match str_to_id(&data, &ids.items_list, 0)
                {
                    Some(res) =>
                    {
                        dest.g_pouch.data[index as usize].0 = res as u16;
                    }
                    None =>
                    {
                        eprintln!("Invalid item (line {})", linecount);
                        return Ok(linecount);
                    }
                }
            }
            else
            {
                dest.g_pouch.data[index as usize].1 = data.parse::<i16>()?;
            }
        }
        else if name.starts_with(dest.item_box.name.as_str())
        {
            let mut index_start = dest.item_box.name.len() + 1;
            let quantity = name.starts_with((dest.item_box.name.clone() + "_qty").as_str());
            if quantity { index_start += 4; }

            let index = name[index_start..].parse::<usize>()?;

            if !quantity
            {
                match str_to_id(&data, &ids.items_list, 0)
                {
                    Some(res) =>
                    {
                        dest.item_box.data[index as usize].0 = res as u16;
                    }
                    None =>
                    {
                        eprintln!("Invalid item (line {})", linecount);
                        return Ok(linecount);
                    }
                }
            }
            else
            {
                dest.item_box.data[index as usize].1 = data.parse::<i16>()?;
            }
        }
        else if name == "NAME" {}
        else
        {
            eprintln!("Unknown entry in line {}", linecount);
        }
    }
    Ok(linecount)
}
