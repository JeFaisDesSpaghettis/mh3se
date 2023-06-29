use std::io::{ BufRead };
use std::fs::File;

pub const CHECKSUM_OFFSET: usize = 0x04;
pub const SLOTS_OFFSET: [usize; 3] = [0x0048, 0x6048, 0xC048];
pub const SLOTS_TOGGLE_OFFSET: [usize; 3] = [0x0, 0x1, 0x2];

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum EquipTypeE {
    None        = 0x0,
    Chest       = 0x1,
    Arms        = 0x2,
    Waist       = 0x3,
    Legs        = 0x4,
    Head        = 0x5,
    Talisman    = 0x6,
    GS          = 0x7,
    SNS         = 0x8,
    HA          = 0x9,
    LA          = 0xA,
    Frame       = 0xB,
    Barrel      = 0xC,
    Stock       = 0xD,
    LS          = 0xE,
    SA          = 0xF,
}

#[derive(PartialEq, PartialOrd)]
pub enum ListGroup{
    NoGroup,
    Armors,
    Weapons,
    Guns
}

pub struct EquipType {
    pub etype: EquipTypeE,
    pub group: ListGroup,
    pub start: usize,
    pub end: usize
}

pub const EQUIP_TYPES: [EquipType ; 16] = [
    EquipType{etype: EquipTypeE::None       , group: ListGroup::NoGroup, start: 0  , end: 0     },
    EquipType{etype: EquipTypeE::Chest      , group: ListGroup::Armors , start: 0  , end: 130   },
    EquipType{etype: EquipTypeE::Arms       , group: ListGroup::Armors , start: 131, end: 255   },
    EquipType{etype: EquipTypeE::Waist      , group: ListGroup::Armors , start: 256, end: 381   },
    EquipType{etype: EquipTypeE::Legs       , group: ListGroup::Armors , start: 382, end: 506   },
    EquipType{etype: EquipTypeE::Head       , group: ListGroup::Armors , start: 507, end: 647   },
    EquipType{etype: EquipTypeE::Talisman   , group: ListGroup::Armors , start: 648, end: 655   },
    EquipType{etype: EquipTypeE::GS         , group: ListGroup::Weapons, start: 0  , end: 93    },
    EquipType{etype: EquipTypeE::SNS        , group: ListGroup::Weapons, start: 94 , end: 185   },
    EquipType{etype: EquipTypeE::HA         , group: ListGroup::Weapons, start: 186, end: 271   },
    EquipType{etype: EquipTypeE::LA         , group: ListGroup::Weapons, start: 378, end: 475   },
    EquipType{etype: EquipTypeE::Frame      , group: ListGroup::Guns   , start: 0  , end: 19    },
    EquipType{etype: EquipTypeE::Barrel     , group: ListGroup::Guns   , start: 20 , end: 39    },
    EquipType{etype: EquipTypeE::Stock      , group: ListGroup::Guns   , start: 40 , end: 60    },
    EquipType{etype: EquipTypeE::LS         , group: ListGroup::Weapons, start: 272, end: 315   },
    EquipType{etype: EquipTypeE::SA         , group: ListGroup::Weapons, start: 316, end: 377   }
];

pub const JEWEL_RANGE: (usize, usize) = (615, 746);

pub struct U8Entry {
    pub name: String,
    pub offset: usize,
    pub data: u8
}

pub struct U16Entry {
    pub name: String,
    pub offset: usize,
    pub data: u16
}

pub struct U32Entry {
    pub name: String,
    pub offset: usize,
    pub data: u32
}

pub struct Name {
    pub name: String,
    pub offset: usize,
    pub data: [u8; 8]
}

#[derive(Default, Clone, Copy)]
pub struct ItemSlot {
    pub id: u16,
    pub qty: i16
}

pub struct ItemSlots {
    pub name: String,
    pub offset: usize,
    pub data: Vec<ItemSlot>
}

/*
    Equipment Slot
    u8: Equipment Type ID
    u8: For Bowgun Frames and Armor: Level, For Talismans: Slots Count
    u16: Equipment ID (or Talisman Grade)
    u8: For Talismans: Skill 2 Points - 10
    u8: For Talismans: Skill 1 Points - 10

    u16: Deco 1 ID / Skill ID
    u16: Deco 2 ID / Skill ID
    u16: Deco 3 ID / Skill ID
 */
pub type EquipSlot = (EquipTypeE, u8, u16, u8, u8, [u16 ; 3]);

pub struct EquipBox {
    pub name: String,
    pub offset: usize,
    pub data: [EquipSlot ; 100 * 8]
}

pub type RGBValue = (u8, u8, u8);

pub struct RGBEntry {
    pub name: String,
    pub offset: usize,
    pub data: RGBValue
}

pub struct CharacterSlot {
    pub slot_enabled:   U8Entry,
    pub gender:         U8Entry,
    pub name:           Name,
    pub zenny:          U32Entry,
    pub playtime:       U32Entry,
    pub b_pouch:        ItemSlots,
    pub g_pouch:        ItemSlots,
    pub item_box:       ItemSlots,
    pub equipment_box:  EquipBox,
    pub hrp:            U32Entry,
    pub hr:             U16Entry,

    /*
        (Offsets for Slot 1)
        Face type       1-9   0x0049 u8    (0x00-0x08)
        Hairstyle       1-11  0x004A u8    (0x00-0x0A)
        Hair color            0x005C u8 x3 (RGB)
        Clothing type   1-4   0x00E6 u8    (0x00-0x03)
        Voice type      1-20  0x00E7 u8    (0x00-0x13)
        Clothing color        0x3A04 u8 x3 (RGB)
        Eye color       1-10  0x3E2E u8    (0x00-0x09)
        Features        0-12  0x3E2F u8    (0x00-0x0C)
        Skin tone       0-612 0x39E8 u16   (0x00-0x264)
     */
    pub face_type:      U8Entry,
    pub hair_type:      U8Entry,
    pub hair_color:     RGBEntry,
    pub cloth_type:     U8Entry,
    pub voice_type:     U8Entry,
    pub cloth_color:    RGBEntry,
    pub eye_color:      U8Entry,
    pub feature_type:   U8Entry,
    pub skin_tone:      U16Entry
}

pub struct DataIDs {
    pub gender_list:    Vec<String>,
    pub items_list:     Vec<String>,
    pub armors_list:    Vec<String>,
    pub weapons_list:   Vec<String>,
    pub gun_parts_list: Vec<String>,
    pub skills_list:    Vec<String>
}

impl CharacterSlot
{
    pub fn default() -> Self
    {
        CharacterSlot {
            slot_enabled: U8Entry{
                name: String::from("Character Toggled"), offset: 0x1A, data: 0
            },
            gender: U8Entry{
                name: String::from("Gender"), offset: 0x00, data: 0
            },
            name: Name{
                name: String::from("Name"), offset: 0x03, data: [0; 8]
            },
            zenny: U32Entry{
                name: String::from("Zenny"), offset: 0x18, data: 0
            },
            playtime: U32Entry{
                name: String::from("Playtime (in seconds)"), offset: 0x1C, data: 0
            },
            b_pouch: ItemSlots{
                name: String::from("Blader Pouch"), offset: 0xA0, data: vec![ItemSlot::default() ; 3 * 8]
            },
            g_pouch: ItemSlots{
                name: String::from("Gunner Pouch"), offset: 0x100, data: vec![ItemSlot::default() ; 4 * 8]
            },
            item_box: ItemSlots {
                name: String::from("Item Box"), offset: 0x180, data: vec![ItemSlot::default() ; 100 * 8]
            },
            equipment_box: EquipBox {
                name: String::from("Equipment Box"), offset: 0x0E00, data: [(EquipTypeE::None, 0, 0, 0, 0, [0, 0, 0]); 100 * 8]
            },
            hrp: U32Entry {
                name: String::from("Hunter Rank Points"), offset: 0x3DE0, data: 0
            },
            hr: U16Entry {
                name: String::from("Hunter Rank"), offset: 0x3DE4, data: 0
            },
            face_type: U8Entry {
                name: String::from("Face Type"), offset: 0x01, data: 0
            },
            hair_type: U8Entry {
                name: String::from("Hair Type"), offset: 0x02, data: 0
            },
            hair_color: RGBEntry {
                name: String::from("Hair RGB Color"), offset: 0x14, data: (255, 255, 255)
            },
            cloth_type: U8Entry {
                name: String::from("Clothing Type"), offset: 0x9E, data: 0
            },
            voice_type: U8Entry {
                name: String::from("Voice Type"), offset: 0x9F, data: 0
            },
            cloth_color: RGBEntry {
                name: String::from("Clothing RGB Color"), offset: 0x39BC, data: (255, 255, 255)
            },
            eye_color: U8Entry {
                name: String::from("Eye Type"), offset: 0x3DE6, data: 0
            },
            feature_type: U8Entry {
                name: String::from("Feature Type"), offset: 0x3DE7, data: 0
            },
            skin_tone: U16Entry {
                name: String::from("Skin Tone"), offset: 0x39A0, data: 0x200
            },
        }
    }
}

impl DataIDs
{
    pub fn new(
        genders_path: &String,
        items_path: &String,
        armors_path: &String,
        weapons_path: &String,
        guns_path: &String,
        skills_path: &String
    ) -> Result<Self, std::io::Error>
    {
        let mut data_id = DataIDs{
            gender_list: Vec::new(),
            items_list: Vec::new(),
            armors_list: Vec::new(),
            weapons_list: Vec::new(),
            gun_parts_list: Vec::new(),
            skills_list: Vec::new()
        };
        data_id.gender_list     = read_lines(genders_path )?;
        data_id.items_list      = read_lines(items_path   )?;
        data_id.armors_list     = read_lines(armors_path  )?;
        data_id.weapons_list    = read_lines(weapons_path )?;
        data_id.gun_parts_list  = read_lines(guns_path    )?;
        data_id.skills_list     = read_lines(skills_path  )?;
        Ok(data_id)
    }

    pub fn get_list(&self, group: &ListGroup) -> Result<&Vec<String>, String>
    {
        match group
        {
            ListGroup::Armors => Ok(&self.armors_list),
            ListGroup::Weapons => Ok(&self.weapons_list),
            ListGroup::Guns => Ok(&self.gun_parts_list),
            _ => Err(format!("No associated group"))
        }
    }
}

pub fn read_lines(filepath: &String) -> Result<Vec<String>, std::io::Error>
{
    let file = File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut csv: Vec<String> = Vec::new();
    for line in reader.lines() {
        csv.push(line?);
    }

    Ok(csv)
}