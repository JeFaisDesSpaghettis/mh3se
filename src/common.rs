use std::io::{ self, BufRead };
use std::fs::File;

pub mod locations
{
    pub const CHECKSUM: usize = 0x04;
    pub const SLOTS: [usize; 3] = [0x0048, 0x6048, 0xC048];
}
pub struct U8Entry
{
    pub name: String,
    pub offset: usize,
    pub data: u8
}

pub struct U16Entry
{
    pub name: String,
    pub offset: usize,
    pub data: u16
}

pub struct U32Entry
{
    pub name: String,
    pub offset: usize,
    pub data: u32
}

pub struct NameEntry
{
    pub name: String,
    pub offset: usize,
    pub data: [u8; 8]
}

pub struct BPouch
{
    pub name: String,
    pub offset: usize,
    pub data: [(u16, i16) ; 3 * 8]
}

pub struct GPouch
{
    pub name: String,
    pub offset: usize,
    pub data: [(u16, i16) ; 4 * 8]
}

pub struct ItemBox
{
    pub name: String,
    pub offset: usize,
    pub data: [(u16, i16) ; 100 * 8]
}

pub struct CharacterSlot
{
    pub file_enabled: U32Entry,
    pub slot1_enabled: U8Entry,
    pub slot2_enabled: U8Entry,
    pub slot3_enabled: U8Entry,
    pub gender: U8Entry,
    pub name: NameEntry,
    pub zenny: U32Entry,
    pub playtime: U32Entry,
    pub b_pouch: BPouch,
    pub g_pouch: GPouch,
    pub item_box: ItemBox,
    pub hrp: U32Entry,
    pub hr: U16Entry,
}

pub fn read_lines(filepath: &String) -> io::Result<Vec<String>>
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

pub struct DataIDs
{
    pub gender_list: Vec<String>,
    pub items_list: Vec<String>
}

impl DataIDs
{
    pub fn new(genderpath: &String, itempath: &String) -> io::Result<Self>
    {
        let mut data_id = DataIDs{gender_list: Vec::new(), items_list: Vec::new()};
        data_id.gender_list = read_lines(genderpath)?;
        data_id.items_list = read_lines(itempath)?;
        Ok(data_id)
    }
}

impl CharacterSlot
{
    pub fn default() -> Self
    {
        CharacterSlot
        {
            file_enabled: U32Entry{name: String::from("file_activated"), offset: 0x00, data: 0x00},
            slot1_enabled: U8Entry{name: String::from("char1_activated"), offset: 0x1A, data: 0x00},
            slot2_enabled: U8Entry{name: String::from("char2_activated"), offset: 0x1B, data: 0x00},
            slot3_enabled: U8Entry{name: String::from("char3_activated"), offset: 0x1C, data: 0x00},
            gender: U8Entry{name: String::from("gender"), offset: 0x00, data: 0x00},
            name: NameEntry{name: String::from("name"), offset: 0x03, data: [0; 8]},
            zenny: U32Entry{name: String::from("zenny"), offset: 0x18, data: 0x00},
            playtime: U32Entry{name: String::from("playtime"), offset: 0x1C, data: 0x00},
            b_pouch: BPouch{name: String::from("blade_pouch"), offset: 0xA0, data: [(0, 0) ; 3 * 8]},
            g_pouch: GPouch{name: String::from("gunner_pouch"), offset: 0x100, data: [(0, 0) ; 4 * 8]},
            item_box: ItemBox { name: String::from("item_box"), offset: 0x180, data: [(0, 0) ; 100 * 8]},
            hrp: U32Entry { name: String::from("hrpoints"), offset: 0x3DE0, data: 0x00 },
            hr: U16Entry { name: String::from("hunterrank"), offset: 0x3DE4, data: 0x00 }
        }
    }
}