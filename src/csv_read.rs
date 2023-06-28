use crate::common::{
    CharacterSlot,
    DataIDs,
    ItemSlot,
    EquipSlot,
    EquipTypeE,
    EquipType,
    ListGroup,
    read_lines,
    JEWEL_RANGE,
    EQUIP_TYPES
};

fn str_to_int<T>(string: &str) -> Result<T, String>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match string.parse::<T>() {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("Failed to parse value {}", err)),
    }
}

fn str_to_id(string: &String, list: &Vec<String>) -> Result<usize, String>
{
    for (index, item) in list.iter().enumerate() {
        if item == string {
            return Ok(index);
        }
    }
    Err(format!("No matching IDs found for {}", string))
}

fn csv_to_item_slot(
    name: &String,
    item_slot: &mut[ItemSlot],
    row: &Vec<String>,
    ids: &DataIDs) -> Result<(), String>
{
    let index = str_to_int::<usize>(&row[0][(name.len() + 1)..])?;

    match str_to_id(&row[1], &ids.items_list) {
        Ok(res) => {
            item_slot[index as usize].id = res as u16;
            item_slot[index as usize].qty = str_to_int::<i16>(&row[2])?;
            return Ok(());
        }
        Err(_) => {
            return Err(format!("Invalid item {}", &row[1]));
        }
    }
}

fn get_info_from_id(id: usize, group: ListGroup) -> &'static EquipType
{
    for equip_type in EQUIP_TYPES.iter() {
        if equip_type.group != group {
            continue;
        }
        if (equip_type.start <= id) && (id <= equip_type.end) {
            return equip_type;
        }
    }
    &EQUIP_TYPES[0] // Never happens
}

fn set_deco_skill_from_name(deco_skill_slots: &mut [u16 ; 3], talisman_slots: isize, row: &Vec<String>, ids: &DataIDs) -> Result<(), String>
{
    if talisman_slots == -1 {
        for i in 0..3 {
            if row[i + 3] != "NONE" {
                deco_skill_slots[i] =
                    (str_to_id(&row[i + 3], &ids.items_list)? - JEWEL_RANGE.0) as u16;
            }
        }
    }
    else {
        let mut jewels_count = talisman_slots;
        for i in 0..3 {
            if jewels_count > 0 {
                if row[i + 3] != "NONE" {
                    deco_skill_slots[i] =
                        (str_to_id(&row[i + 3], &ids.items_list)? - JEWEL_RANGE.0) as u16;
                }
                jewels_count -= 1;
            }
            else {
                deco_skill_slots[i] = str_to_id(&row[i + 3], &ids.skills_list)? as u16;
            }
        }
    }
    Ok(())
}

fn csv_to_equip_slot(
    name: &String,
    equip_slot: &mut[EquipSlot],
    row: &Vec<String>,
    ids: &DataIDs) -> Result<(), String>
{
    let index = str_to_int::<usize>(&row[0][(name.len() + 1)..])?;
    if row[1] == "NONE" {
        equip_slot[index] = (EquipTypeE::None, 0, 0, 0, 0, [0, 0, 0]);
    }
    else if ids.armors_list.contains(&row[1])
    {
        let armor_id = str_to_id(&row[1], &ids.armors_list)?;
        let equip_info = get_info_from_id(armor_id, ListGroup::Armors);

        if equip_info.etype != EquipTypeE::Talisman {
            equip_slot[index].0 = equip_info.etype;
            equip_slot[index].1 = str_to_int::<u8>(&row[2])? - 1;
            equip_slot[index].2 = (armor_id - equip_info.start) as u16;
            set_deco_skill_from_name(&mut equip_slot[index].5, -1, row, ids)?;
        }
        else {
            equip_slot[index].0 = equip_info.etype;
            equip_slot[index].1 = str_to_int::<u8>(&row[2])?;
            equip_slot[index].2 = (armor_id - equip_info.start) as u16;
            equip_slot[index].3 = (str_to_int::<i8>(&row[7])? + 10) as u8;
            equip_slot[index].4 = (str_to_int::<i8>(&row[6])? + 10) as u8;
            set_deco_skill_from_name(&mut equip_slot[index].5, equip_slot[index].1 as isize, row, ids)?;
        }
    }
    else if ids.weapons_list.contains(&row[1])
    {
        let weapon_id = str_to_id(&row[1], &ids.weapons_list)?;
        let equip_info = get_info_from_id(weapon_id, ListGroup::Weapons);

        equip_slot[index].0 = equip_info.etype;
        equip_slot[index].1 = str_to_int::<u8>(&row[2])? - 1;
        equip_slot[index].2 = (weapon_id - equip_info.start) as u16;
        set_deco_skill_from_name(&mut equip_slot[index].5, -1, row, ids)?;
    }
    else if ids.gun_parts_list.contains(&row[1])
    {
        let weapon_id = str_to_id(&row[1], &ids.gun_parts_list)?;
        let equip_info = get_info_from_id(weapon_id, ListGroup::Guns);

        equip_slot[index].0 = equip_info.etype;
        equip_slot[index].1 = str_to_int::<u8>(&row[2])? - 1;
        equip_slot[index].2 = (weapon_id - equip_info.start) as u16;
        set_deco_skill_from_name(&mut equip_slot[index].5, -1, row, ids)?;
    }
    else {
        return Err(format!("Invalid equipment {}", &row[1]));
    }
    Ok(())
}

pub fn csv_to_save(csv: &Vec<String>, slot: &mut CharacterSlot, ids: &DataIDs) -> Result<usize, String>
{
    let mut linecount: usize = 0;
    for line in csv
    {
        linecount += 1;
        let parts: Vec<String> = line
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if parts[0] == slot.file_enabled.name && parts.len() == 2 {
            match str_to_int::<u32>(&parts[1]) {
                Ok(res) => slot.file_enabled.data = res,
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0] == slot.slot1_enabled.name && parts.len() == 2 {
            match str_to_int::<u8>(&parts[1]) {
                Ok(res) => slot.slot1_enabled.data = res,
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0] == slot.slot2_enabled.name && parts.len() == 2 {
            match str_to_int::<u8>(&parts[1]) {
                Ok(res) => slot.slot2_enabled.data = res,
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0] == slot.slot3_enabled.name && parts.len() == 2 {
            match str_to_int::<u8>(&parts[1]) {
                Ok(res) => slot.slot3_enabled.data = res,
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0] == slot.gender.name && parts.len() == 2 {
            match str_to_id(&parts[1], &ids.gender_list) {
                Ok(res) => slot.gender.data = res as u8,
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err));
                }
            }
        }
        else if parts[0] == slot.name.name && parts.len() == 2 {
            let bytes = parts[1].as_bytes();
            let length = std::cmp::min(bytes.len(), 8);
            slot.name.data[..length].copy_from_slice(&bytes[..length]);
        }
        else if parts[0] == slot.zenny.name && parts.len() == 2 {
            match str_to_int::<u32>(&parts[1]) {
                Ok(res) => slot.zenny.data = res,
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0] == slot.playtime.name && parts.len() == 2 {
            match str_to_int::<u32>(&parts[1]) {
                Ok(res) => slot.playtime.data = res,
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0] == slot.hrp.name && parts.len() == 2 {
            match str_to_int::<u32>(&parts[1]) {
                Ok(res) => slot.hrp.data = res,
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0] == slot.hr.name && parts.len() == 2 {
            match str_to_int::<u16>(&parts[1]) {
                Ok(res) => slot.hr.data = res,
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0].starts_with(slot.b_pouch.name.as_str()) && parts.len() == 3 {
            match csv_to_item_slot(&slot.b_pouch.name, &mut slot.b_pouch.data, &parts, &ids) {
                Ok(_) => {},
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0].starts_with(slot.g_pouch.name.as_str()) && parts.len() == 3 {
            match csv_to_item_slot(&slot.g_pouch.name, &mut slot.g_pouch.data, &parts, &ids) {
                Ok(_) => {},
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0].starts_with(slot.item_box.name.as_str()) && parts.len() == 3 {
            match csv_to_item_slot(&slot.item_box.name, &mut slot.item_box.data, &parts, &ids) {
                Ok(_) => {},
                Err(err) => {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0].starts_with(slot.equipment_box.name.as_str()) && parts.len() >= 2 {
            match csv_to_equip_slot(&slot.equipment_box.name, &mut slot.equipment_box.data, &parts, &ids) {
                Ok(_) => {},
                Err(err) =>
                {
                    return Err(format!("Error on line {} ({})", linecount, err))
                }
            }
        }
        else if parts[0] == "Name" {}
        else {
            return Err(format!("Error on line {}", linecount));
        }
    }
    Ok(linecount)
}


pub fn file_to_csv(filepath: &String) -> Result<Vec<String>, std::io::Error>
{
    read_lines(filepath)
}