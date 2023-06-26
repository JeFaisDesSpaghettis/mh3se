# mh3se - mh3 save editor

## Roadmap
+ Equipment Worn + Box
* HR and HRP?

## Notes

It is not really a save editor, it just translates save files to a giant spreadsheet for you to modify it and can apply this spreadsheet back to a save file.

Be careful with you saves!!! This is very much experimental please make backups before any modifications!

Also i'm not at all familiar with this programming language (rust) so i can't guarantee good quality and readability

## Usage

### Getting the save files

#### Dolphin
The path to saves for MHTri EU is `dolphin-emu/Wii/title/00010000/524d4850/`

I don't know for US and JP unfortunately try searching in the `title` folder for files named data00, data01 etc...

#### Wii
You'll have to export your saves and unpack them with [FE100](https://www.wiibrew.org/wiki/FE100)

### Read save file to a csv file
```
mh3se decode [save_file] [csv_file] [character_slot]

save_file -> Save file you want to decode like data00, data01 etc...

csv_file -> Output csv file you can modify according to what you want to change name, gender etc...

character_slot -> self-explanatory
```

### Apply csv file to a save file
```
mh3se encode [save_file] [csv_file] [character_slot]

save_file -> Save file on which you want to apply the modifications

csv_file -> Input csv file that describes what to change

character_slot -> self-explanatory
```

### Example of the csv file
```
NAME, DATA
gender, female
name, anyname
zenny, 10000
playtime, 60

note that the playtime is in seconds
```

## Other

Based on [this](https://github.com/sepalani/MHTrIDA/tree/master/save) and (will be based) [this](https://github.com/sepalani/MH3DB)

Some pending findings

```
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
    // Equipment Box...

    // Hunter rank...
    hr: u16
    */
```