use std::io::{ Read };
use std::fs::File;

use crate::common::{CharacterSlot, ItemSlots, EquipTypeE, SLOTS_OFFSET };

fn read_u8(buf: &Vec<u8>, address: usize) -> u8
{
    buf[address]
}

fn read_u16(buf: &Vec<u8>, address: usize) -> u16
{
    ((buf[address + 0] as u16) << 8) |
     (buf[address + 1] as u16)
}

fn read_u32(buf: &Vec<u8>, address: usize) -> u32
{
    ((buf[address + 0] as u32) << 24) |
    ((buf[address + 1] as u32) << 16) |
    ((buf[address + 2] as u32) << 8)  |
     (buf[address + 3] as u32)
}

fn read_name(buf: &Vec<u8>, address: usize) -> [u8; 8]
{
    let segment = &buf[address..address + 8];
    let array: Result<[u8; 8], _> = segment.try_into();

    match array {
        Ok(fixed_array) => return fixed_array,
        Err(_) => {
            eprintln!("Couldn't read name, replacing with a placeholder...");
            return b"STUBSTUB".to_owned()
        }
    }
}

pub fn file_to_buf(filepath: &String) -> Result<Vec<u8>, std::io::Error>
{
    let mut file = File::open(filepath)?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn buf_to_item_slots(buf: &Vec<u8>, container: &mut ItemSlots, slot_n: usize)
{
    for k in 0..container.data.len() {
        container.data[k].id =
            read_u16(buf, SLOTS_OFFSET[slot_n] + container.offset + (k * 4));
        container.data[k].qty =
            read_u16(buf, SLOTS_OFFSET[slot_n] + container.offset + (k * 4) + 2) as i16;
    }
}

pub fn buf_to_save(buf: &Vec<u8>, slot: &mut CharacterSlot, slot_n: usize)
{
    slot.file_enabled.data     = read_u32(buf,  slot.file_enabled.offset    );
    slot.slot1_enabled.data    = read_u8(buf,  slot.slot1_enabled.offset    );
    slot.slot2_enabled.data    = read_u8(buf,  slot.slot2_enabled.offset    );
    slot.slot3_enabled.data    = read_u8(buf,  slot.slot3_enabled.offset    );
    slot.gender.data    = read_u8(buf,  SLOTS_OFFSET[slot_n] + slot.gender.offset  );
    slot.name.data      = read_name(buf, SLOTS_OFFSET[slot_n] + slot.name.offset   );
    slot.zenny.data     = read_u32(buf, SLOTS_OFFSET[slot_n] + slot.zenny.offset   );
    slot.playtime.data  = read_u32(buf, SLOTS_OFFSET[slot_n] + slot.playtime.offset);
    buf_to_item_slots(buf, &mut slot.b_pouch, slot_n);
    buf_to_item_slots(buf, &mut slot.g_pouch, slot_n);
    buf_to_item_slots(buf, &mut slot.item_box, slot_n);
    for k in 0..slot.equipment_box.data.len() {
        match read_u8(buf, SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12))
        {
            1 => slot.equipment_box.data[k].0 = EquipTypeE::Chest,
            2 => slot.equipment_box.data[k].0 = EquipTypeE::Arms,
            3 => slot.equipment_box.data[k].0 = EquipTypeE::Waist,
            4 => slot.equipment_box.data[k].0 = EquipTypeE::Legs,
            5 => slot.equipment_box.data[k].0 = EquipTypeE::Head,
            6 => slot.equipment_box.data[k].0 = EquipTypeE::Talisman,
            7 => slot.equipment_box.data[k].0 = EquipTypeE::GS,
            8 => slot.equipment_box.data[k].0 = EquipTypeE::SNS,
            9 => slot.equipment_box.data[k].0 = EquipTypeE::HA,
            10 => slot.equipment_box.data[k].0 = EquipTypeE::LA,
            11 => slot.equipment_box.data[k].0 = EquipTypeE::Frame,
            12 => slot.equipment_box.data[k].0 = EquipTypeE::Barrel,
            13 => slot.equipment_box.data[k].0 = EquipTypeE::Stock,
            14 => slot.equipment_box.data[k].0 = EquipTypeE::LS,
            15 => slot.equipment_box.data[k].0 = EquipTypeE::SA,
            _ => slot.equipment_box.data[k].0 = EquipTypeE::None
        }
        slot.equipment_box.data[k].1 =
            read_u8(buf, SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 1);
        slot.equipment_box.data[k].2 =
            read_u16(buf, SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 2);
        slot.equipment_box.data[k].3 =
            read_u8(buf, SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 4);
        slot.equipment_box.data[k].4 =
            read_u8(buf, SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 5);
        slot.equipment_box.data[k].5 = [
            read_u16(buf, SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 6),
            read_u16(buf, SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 8),
            read_u16(buf, SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 10)
        ]
    }
    slot.hrp.data    = read_u32(buf,  SLOTS_OFFSET[slot_n] + slot.hrp.offset   );
    slot.hr.data     = read_u16(buf,  SLOTS_OFFSET[slot_n] + slot.hr.offset     );
}
