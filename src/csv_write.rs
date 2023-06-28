use std::io::{ Write };
use std::fs::File;

use crate::common::{
    CharacterSlot,
    DataIDs,
    ItemSlots,
    EquipBox,
    EquipTypeE,
    JEWEL_RANGE,
    EQUIP_TYPES,
};

fn id_to_str(list: &Vec<String>, id: usize) -> &String
{
    return &list[id]
}

fn item_slots_to_lines(item_slots: &ItemSlots, csv: &mut Vec<String>, ids: &DataIDs)
{
    for k in 0..item_slots.data.len() {
        csv.push(format!("{} {}, {}, {}",
        item_slots.name,
            k,
            id_to_str(&ids.items_list, item_slots.data[k].id as usize),
            item_slots.data[k].qty));
    }
}

fn get_deco_skill_name(deco_skill: &[u16 ; 3], talisman_slots: isize, ids: &DataIDs) -> [String ; 3]
{
    if talisman_slots == -1 {
        let mut deco_names: [String; 3] = [String::from("NONE"), String::from("NONE"), String::from("NONE")];
        for i in 0..3 {
            if deco_skill[i] != 0 {
                deco_names[i] = id_to_str(&ids.items_list, JEWEL_RANGE.0 + (deco_skill[i] as usize)).clone()
            }
        }
        return deco_names;
    }
    else {
        let mut deco_skill_names: [String; 3] = [String::from("NONE"), String::from("NONE"), String::from("NONE")];
        let mut jewels_count = talisman_slots;
        for i in 0..3 {
            if jewels_count > 0 {
                if deco_skill[i] != 0 {
                    deco_skill_names[i] = id_to_str(&ids.items_list, JEWEL_RANGE.0 + (deco_skill[i] as usize)).clone()
                }
                jewels_count -= 1;
            }
            else {
                deco_skill_names[i] = id_to_str(&ids.skills_list, deco_skill[i] as usize).clone()
            }
        }
        return deco_skill_names;
    }
}

fn equip_box_to_lines(equip_box: &EquipBox, csv: &mut Vec<String>, ids: &DataIDs) -> Result<(), String>
{
    for k in 0..equip_box.data.len()
    {
        let equip_slot = equip_box.data[k];
        let equip_type = equip_slot.0;
        match equip_type
        {
            // Nothing
            EquipTypeE::None => {
                csv.push(format!("{} {}, {}", equip_box.name, k, "NONE"));
            }
            // Armors and Weapons
            x if (x != EquipTypeE::None) && (x != EquipTypeE::Talisman) =>
            {
                let type_info = &EQUIP_TYPES[equip_type as usize];
                let level = equip_slot.1 + 1;
                let equip_name =
                    id_to_str(ids.get_list(&type_info.group)?, type_info.start + (equip_slot.2 as usize));
                let deco_names = get_deco_skill_name(&equip_slot.5, -1, &ids);
                csv.push(format!("{} {}, {}, {}, {}, {}, {}",
                    equip_box.name,
                    k,
                    equip_name,
                    level,
                    deco_names[0],
                    deco_names[1],
                    deco_names[2]));
            }
            // Talismans
            EquipTypeE::Talisman =>
            {
                let type_info = &EQUIP_TYPES[equip_type as usize];
                let slots_count = equip_slot.1;
                let talisman_grade =
                    id_to_str(ids.get_list(&type_info.group)?, type_info.start + (equip_slot.2 as usize));
                let skill2 = (equip_slot.3 as i8) - 10;
                let skill1 = (equip_slot.4 as i8) - 10;
                let deco_skill_names =
                    get_deco_skill_name(&equip_slot.5, slots_count as isize, &ids);
                csv.push(format!("{} {}, {}, {}, {}, {}, {}, {}, {}",
                    equip_box.name,
                    k,
                    talisman_grade,
                    slots_count,
                    deco_skill_names[0],
                    deco_skill_names[1],
                    deco_skill_names[2],
                    skill1,
                    skill2));
            }
            _ => { return Err(format!("UNKNOWN ERROR")) }
        }
    }
    Ok(())
}

pub fn save_to_csv(slot: &CharacterSlot, csv: &mut Vec<String>, ids: &DataIDs)
{
    csv.push(format!("{}, {}", slot.file_enabled.name, slot.file_enabled.data));
    csv.push(format!("{}, {}", slot.slot1_enabled.name, slot.slot1_enabled.data));
    csv.push(format!("{}, {}", slot.slot2_enabled.name, slot.slot2_enabled.data));
    csv.push(format!("{}, {}", slot.slot3_enabled.name, slot.slot3_enabled.data));
    csv.push(format!("{}, {}", slot.gender.name, id_to_str(&ids.gender_list, slot.gender.data as usize)));
    let mut name: String = String::from("STUBSTUB");
    match String::from_utf8(slot.name.data.as_slice().to_vec()) {
        Ok(res) => name = res,
        Err(err) => {
            eprintln!("Name replaced with a placeholder ({})", err);
        }
    }
    csv.push(format!("{}, {}", slot.name.name, name));
    csv.push(format!("{}, {}", slot.zenny.name, slot.zenny.data));
    csv.push(format!("{}, {}", slot.playtime.name, slot.playtime.data));
    csv.push(format!("{}, {}", slot.hrp.name, slot.hrp.data));
    csv.push(format!("{}, {}", slot.hr.name, slot.hr.data));
    item_slots_to_lines(&slot.b_pouch, csv, ids);
    item_slots_to_lines(&slot.g_pouch, csv, ids);
    item_slots_to_lines(&slot.item_box, csv, ids);
    let _ = equip_box_to_lines(&slot.equipment_box, csv, ids);
}


pub fn csv_to_file(filepath: &String, csv: &Vec<String>) -> Result<(), std::io::Error>
{
    let mut file = File::create(filepath)?;
    writeln!(file,
        "Name, \
        Data / Item Name / Equipment Name, \
        Quantity / Level / Slots Count, \
        Deco 1 / Skill, \
        Deco 2 / Skill, \
        Deco 3 / Skill, \
        Skill 1 Points, \
        Skill 2 Points")?;
    for line in csv {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}