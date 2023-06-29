# mh3se - mh3 save editor

## Warning
Be careful with you saves!!! Please make backups before any modifications! I didn't make any thorough tests!

Based on [this](https://github.com/sepalani/MHTrIDA/tree/master/save) and [this](https://github.com/sepalani/MH3DB)

## Features
```
• Character Toggle Flags
• Gender
• Name
• Zenny
• Playtime
• HR points
• HR
• All character customization characteristics: voice, skin tone etc...
• Blademaster and Gunner Pouches
• Item Box
• Equipment Box, including custom talismans
```

## Usage

### 1. Getting the save files

#### On dolphin-emu
The path to saves for MHTri EU is `dolphin-emu/Wii/title/00010000/524d4850/`

The path to saves for MHTri US *should be* `dolphin-emu/Wii/title/00010000/524d4845/`

#### On a real console
You'll have to export your saves and unpack them with [FE100](https://www.wiibrew.org/wiki/FE100)

### 2. Read save file to a csv file
```
mh3se decode [save_file] [csv_file] [character_slot]

save_file       -> Save file you want to decode like data00, data01 etc...
csv_file        -> Output csv file you can modify according to what you want to change name, gender etc...
character_slot  -> self-explanatory
```

### 3. Add some cursed things in the csv file... ( • _ • )

| Entry Name        | Equipment Name  |Lvl / Slots|  Deco / Skill   |  Deco / Skill   |  Deco / Skill  | Points 1| Points 2 |
| :-----------------|:----------------|----------:|:----------------|:----------------|:---------------|--------:|---------:|
| Equipment Box 542 | Dragon Talisman |     1     | NONE            | Handicraft      | Fencing        |    15   |    10    |
| Equipment Box 543 | Dragon Talisman |     1     | NONE            | Expert          | Health         |    20   |    -15   |
| Equipment Box 544 | Nega-Babylon    |     1     | Onslaught Jewel | Onslaught Jewel | Onslaught Jewe |         |          |

### 4. Apply csv file to a save file
```
mh3se encode [save_file] [csv_file] [character_slot]

save_file       -> Save file on which you want to apply the modifications
csv_file        -> Input csv file that describes what to change
character_slot  -> self-explanatory
```

[Example CSV File](/example.csv)

## Some random findings
```
/*
    worn_weapon_type: u8, // 0x0068
    worn_weapon_lvl: u8, // 0x0069
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
*/
```