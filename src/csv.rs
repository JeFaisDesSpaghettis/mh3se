use std::io::{ self, Write, BufRead };
use std::fs::File;

use crate::defs::{ CharacterSlot, GENDERS };

fn name_to_id(name: &String, list: &Vec<String>, mut offset: isize) -> isize
{
    for line in list
    {
        if name == line
        {
            return offset;
        }
        offset += 1;
    }
    return -1
}

fn id_to_name(list: &Vec<String>, offset: usize) -> &String
{
    return &list[offset]
}

pub fn save_to_csv(src: &CharacterSlot, dest: &mut Vec<String>)
{
    dest.push(format!("{}, {}", src.gender.name, if src.gender.data == 0x01 { "female" } else { "male" }));
    let mut name: String = String::from("");
    match String::from_utf8(src.name.data.as_slice().to_vec())
    {
        Ok(res) => { name = res},
        Err(err) => eprintln!("{}", err)
    }
    dest.push(format!("{}, {}", src.name.name, name));
    dest.push(format!("{}, {}", src.zenny.name, src.zenny.data));
    dest.push(format!("{}, {}", src.playtime.name, src.playtime.data));
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
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    let mut csv: Vec<String> = Vec::new();

    for line in reader.lines()
    {
        csv.push(line?);
    }

    Ok(csv)
}

pub fn csv_to_save(csv: &Vec<String>, dest: &mut CharacterSlot)
{
    let mut linecount: usize = 0;
    for line in csv
    {
        linecount += 1;
        let parts: Vec<&str> = line.splitn(2, ',').collect();

        if parts.len() != 2
        {
            println!("Invalid line {}", linecount);
            return
        }
        let name: String = parts[0].trim().to_string();
        let data: String = parts[1].trim().to_string();

        // Gotta find a better solution, probably...
        if name == dest.gender.name
        {
            dest.gender.data = name_to_id(&data, &GENDERS.iter().map(|&s| s.to_string()).collect(), 0) as u8;
        }
        else if name == dest.name.name
        {
            let bytes = data.as_bytes();
            let copy_length = std::cmp::min(bytes.len(), 8);
            dest.name.data[..copy_length].copy_from_slice(&bytes[..copy_length]);
        }
        else if name == dest.zenny.name
        {
            match data.parse::<u32>()
            {
                Ok(number) =>
                {
                    dest.zenny.data = number
                },
                Err(err) => eprintln!("Error {}", err)
            }
        }
        else if name == dest.playtime.name
        {
            match data.parse::<u32>()
            {
                Ok(number) =>
                {
                    dest.playtime.data = number
                },
                Err(err) => eprintln!("Error {}", err)
            }
        }
    }
}
