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

pub struct StrEntry
{
    pub name: String,
    pub offset: usize,
    pub data: [u8; 8]
}

pub const GENDERS: [&str ; 2] = ["male", "female"];

pub struct CharacterSlot
{
    pub gender: U8Entry,
    pub name: StrEntry,
    pub zenny: U32Entry,
    pub playtime: U32Entry,

    /* [WIP]
    // Worn Equipment...
    worn_weapon_type: u8, // 0x0068
    worn_weapon_id: u16, // 0x006A
    worn_weapon_deco_1: u16,   // 0x006E
    worn_weapon_deco_2: u16,   // 0x0070
    worn_weapon_deco_3: u16,   // 0x0072
    worn_weapon_loc: u16,   // 0x00D4

    worn_head_lvl: u8, // 0x00BD
    worn_head_id: u16, // 0x00BE
    worn_head_deco_1: u16, // 0x00C2, offset to first deco in item list
    worn_head_loc: u16, // 0x00E2

    worn_arms_lvl: u8, // 0x0099
    worn_arms_id: u16, // 0x009A
    worn_arms_deco_1: u16, // 0x009E, offset to first deco in item list
    worn_arms_deco_2: u16, // 0x00A0, offset to first deco in item list
    worn_arms_loc: u16, // 0x00DC
    //...
    b_pouch: [ItemSlot; 8 * 3],
    g_pouch: [ItemSlot; 8 * 4],
    item_box: [ItemSlot; 800],
    // Equipment Box...
    // Hunter rank
    hr: u16*/
}

impl CharacterSlot
{
    pub fn default() -> Self
    {
        CharacterSlot
        {
            gender: U8Entry{name: String::from("gender"), offset: 0x00, data: 0x00},
            name: StrEntry{name: String::from("name"), offset: 0x03, data: [0; 8]},
            zenny: U32Entry{name: String::from("zenny"), offset: 0x18, data: 0x00},
            playtime: U32Entry{name: String::from("playtime"), offset: 0x1C, data: 0x00},
        }
    }
}