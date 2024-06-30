"use strict";

let lastChoice = "";

let Database = {};
let SaveSlot = {};

const IdToType = [
    "NONE",
    "chest",
    "arms",
    "waist",
    "legs",
    "head",
    "talisman",
    "gs",
    "sns",
    "ha",
    "la",
    "frame",
    "barrel",
    "stock",
    "ls",
    "sa",
];

function rgbToHex(r, g, b) {
    return "#" + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1).toUpperCase();
}

function hexToRgb(hex) {
    let bigint = parseInt(hex.slice(1), 16);
    let r = (bigint >> 16) & 255;
    let g = (bigint >> 8) & 255;
    let b = bigint & 255;
    return [r, g, b];
}

function onTypeChange(target) {
    let typeId = Database.type.indexOf(target.value);

    const row = target.parentElement.parentElement;
    if ((typeId >= 7 && typeId <= 10) || (typeId >= 14 && typeId <= 15)) {
        row.replaceWith(generateMeleeWeapon({
            MeleeWeapon: {
                type_id: typeId,
                unused_lvl: 0,
                id: 0,
                unused_skill2_pt: 0,
                unused_skill1_pt: 0,
                deco1: 0,
                deco2: 0,
                deco3: 0
            }
        }, row.id));
    }
    else if (typeId >= 11 && typeId <= 13) {
        row.replaceWith(generateRangedWeapon({
            RangedWeapon: {
                type_id: typeId,
                lvl: 0,
                id: 0,
                unused_skill2_pt: 0,
                unused_skill1_pt: 0,
                deco1: 0,
                deco2: 0,
                deco3: 0
            }
        }, row.id));
    }
    else if (typeId >= 1 && typeId <= 5) {
        row.replaceWith(generateArmor({
            Armor: {
                type_id: typeId,
                lvl: 0,
                id: 0,
                unused_skill2_pt: 0,
                unused_skill1_pt: 0,
                deco1: 0,
                deco2: 0,
                deco3: 0
            }
        }, row.id));
    }
    else if (typeId == 6) {
        row.replaceWith(generateOneSlotTalisman({
            OneSlotTalisman: {
                type_id: 6,
                slot_count: 1,
                id: 0,
                skill2_pt: 0,
                skill1_pt: 0,
                deco1: 0,
                skill1_id: 0,
                skill2_id: 0
            }
        }, row.id));
    }
    else {
        row.replaceWith(generateBlankEquipSlot({
            BlankEquipSlot: {
                buf: [0,0,0,0,0,0,0,0,0,0,0,0]
            }
        }, row.id));
    }
}

function onSlotCountChange(target) {
    const currentRow = target.parentElement.parentElement;

    switch (parseInt(target.value)) {
        case 0: {
            currentRow.replaceWith(generateZeroSlotTalisman({
                ZeroSlotTalisman: {
                    type_id: 6,
                    slot_count: 0,
                    id: 0,
                    skill2_pt: 0,
                    skill1_pt: 0,
                    skill1_id: 0,
                    skill2_id: 0,
                    unused_deco3: 0
                }
            }, currentRow.id));
            break;
        }
        case 1: {
            currentRow.replaceWith(generateOneSlotTalisman({
                OneSlotTalisman: {
                    type_id: 6,
                    slot_count: 1,
                    id: 0,
                    skill2_pt: 0,
                    skill1_pt: 0,
                    deco1: 0,
                    skill1_id: 0,
                    skill2_id: 0
                }
            }, currentRow.id));
            break;
        }
        case 2: {
            currentRow.replaceWith(generateTwoSlotTalisman({
                TwoSlotTalisman: {
                    type_id: 6,
                    slot_count: 2,
                    id: 0,
                    unused_skill2_pt: 0,
                    skill1_pt: 0,
                    deco1: 0,
                    deco2: 0,
                    skill1_id: 0
                }
            }, currentRow.id));
            break;
        }
        case 3: {
            currentRow.replaceWith(generateThreeSlotTalisman({
                ThreeSlotTalisman: {
                    type_id: 6,
                    slot_count: 3,
                    id: 0,
                    unused_skill2_pt: 0,
                    unused_skill1_pt: 0,
                    deco1: 0,
                    deco2: 0,
                    deco3: 0
                }
            }, currentRow.id));
            break;
        }
        default: {
            console.log("Something's wrong!");
            break;
        }
    }
}

function generateBlankCell()
{
    const blankCell = document.createElement("td");
    const blankText = document.createElement("p");
    blankText.style.textAlign = "center";
    blankText.textContent = 'X';
    blankCell.appendChild(blankText);
    return blankCell;
}

function generateIntSelectCell(value, min, max, talismanSlotCount = false)
{
    const intSelectCell = document.createElement("td");
    const intSelectInput = document.createElement("input");
    if (talismanSlotCount) {
        intSelectInput.onchange = function () {
            onSlotCountChange(intSelectInput);
        };
    }
    intSelectInput.type = "number";
    intSelectInput.min = min;
    intSelectInput.max = max;
    intSelectInput.value = value;
    intSelectCell.appendChild(intSelectInput);
    return intSelectCell;
}

function generateSelectCell(optList, optSelect, typeChange = false)
{
    const suggestCell = document.createElement("td");
    const suggestInput = document.createElement("textarea");

    suggestInput.value = optList[optSelect];
    suggestInput.placeholder = "Search...";

    const suggestBox = document.createElement("div");
    suggestBox.className = "suggestions";

    suggestInput.oninput = function () {
        const query = this.value.toLowerCase();
        suggestBox.innerHTML = '';

        if (query.length > 0) {
            const inferedSuggest = optList.filter(item => item.toLowerCase().includes(query));
            inferedSuggest.forEach(suggestion => {
                const suggestionItem = document.createElement('div');
                suggestionItem.className = 'suggestion-item';
                suggestionItem.textContent = suggestion;
                suggestionItem.onclick = function () {
                    suggestInput.value = suggestion;
                    const event = new Event('input', {
                        bubbles: true,
                        cancelable: true
                    });
                    suggestInput.dispatchEvent(event);
                    suggestBox.innerHTML = '';
                    if (typeChange) {
                        onTypeChange(suggestInput);
                    }
                };
                suggestBox.appendChild(suggestionItem);
            });
        }
    };

    suggestCell.appendChild(suggestInput);
    suggestCell.appendChild(suggestBox);

    return suggestCell;
}

function generateBlankEquipSlot(item, id) {
    const row = document.createElement("tr");
    row.id = id;

    row.appendChild(generateSelectCell(Database.type, 0, true));

    for (let i = 0; i < 7; i++) {
        row.appendChild(generateBlankCell());
    }

    return row;
}

function generateOneSlotTalisman(item, id) {
    let slotsCount = item.OneSlotTalisman.slot_count;
    let skill1Pt = item.OneSlotTalisman.skill1_pt;
    let skill2Pt = item.OneSlotTalisman.skill2_pt;

    const row = document.createElement("tr");
    row.id = id;

    row.appendChild(generateSelectCell(Database.type, item.OneSlotTalisman.type_id, true));
    row.appendChild(generateIntSelectCell(slotsCount, 0, 3, true));
    row.appendChild(generateSelectCell(Database[IdToType[[item.OneSlotTalisman.type_id]]], item.OneSlotTalisman.id));
    row.appendChild(generateIntSelectCell(skill1Pt - 10, -10, 245));
    row.appendChild(generateIntSelectCell(skill2Pt - 10, -10, 245));
    row.appendChild(generateSelectCell(Database.jewel, item.OneSlotTalisman.deco1));
    row.appendChild(generateSelectCell(Database.skill, item.OneSlotTalisman.skill1_id));
    row.appendChild(generateSelectCell(Database.skill, item.OneSlotTalisman.skill2_id));

    return row;
}

function generateZeroSlotTalisman(item, id) {
    let slotsCount = item.ZeroSlotTalisman.slot_count;
    let skill1Pt = item.ZeroSlotTalisman.skill1_pt;
    let skill2Pt = item.ZeroSlotTalisman.skill2_pt;

    const row = document.createElement("tr");
    row.id = id;

    row.appendChild(generateSelectCell(Database.type, item.ZeroSlotTalisman.type_id, true));
    row.appendChild(generateIntSelectCell(slotsCount, 0, 3, true));
    row.appendChild(generateSelectCell(Database[IdToType[[item.ZeroSlotTalisman.type_id]]], item.ZeroSlotTalisman.id));
    row.appendChild(generateIntSelectCell(skill1Pt - 10, -10, 245));
    row.appendChild(generateIntSelectCell(skill2Pt - 10, -10, 245));
    row.appendChild(generateSelectCell(Database.skill, item.ZeroSlotTalisman.skill1_id));
    row.appendChild(generateSelectCell(Database.skill, item.ZeroSlotTalisman.skill2_id));
    row.appendChild(generateBlankCell());

    return row;
}

function generateTwoSlotTalisman(item, id) {
    let slotsCount = item.TwoSlotTalisman.slot_count;
    let skill1Pt = item.TwoSlotTalisman.skill1_pt;

    const row = document.createElement("tr");
    row.id = id;

    row.appendChild(generateSelectCell(Database.type, item.TwoSlotTalisman.type_id, true));
    row.appendChild(generateIntSelectCell(slotsCount, 0, 3, true));
    row.appendChild(generateSelectCell(Database[IdToType[[item.TwoSlotTalisman.type_id]]], item.TwoSlotTalisman.id));
    row.appendChild(generateIntSelectCell(skill1Pt - 10, -10, 245));
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.TwoSlotTalisman.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.TwoSlotTalisman.deco2));
    row.appendChild(generateSelectCell(Database.skill, item.TwoSlotTalisman.skill1_id));

    return row;
}

function generateThreeSlotTalisman(item, id) {
    let slotsCount = item.ThreeSlotTalisman.slot_count;

    const row = document.createElement("tr");
    row.id = id;

    row.appendChild(generateSelectCell(Database.type, item.ThreeSlotTalisman.type_id, true));
    row.appendChild(generateIntSelectCell(slotsCount, 0, 3, true));
    row.appendChild(generateSelectCell(Database[IdToType[[item.ThreeSlotTalisman.type_id]]], item.ThreeSlotTalisman.id));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.ThreeSlotTalisman.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.ThreeSlotTalisman.deco2));
    row.appendChild(generateSelectCell(Database.jewel, item.ThreeSlotTalisman.deco3));

    return row;
}

function generateArmor(item, id) {
    let equipLevel = item.Armor.lvl;

    const row = document.createElement("tr");
    row.id = id;

    row.appendChild(generateSelectCell(Database.type, item.Armor.type_id, true));
    row.appendChild(generateIntSelectCell(equipLevel + 1, 0, 32));
    row.appendChild(generateSelectCell(Database[IdToType[[item.Armor.type_id]]], item.Armor.id));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.Armor.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.Armor.deco2));
    row.appendChild(generateSelectCell(Database.jewel, item.Armor.deco3));

    return row;
}

function generateMeleeWeapon(item, id) {
    const row = document.createElement("tr");
    row.id = id;

    row.appendChild(generateSelectCell(Database.type, item.MeleeWeapon.type_id, true));
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database[IdToType[[item.MeleeWeapon.type_id]]], item.MeleeWeapon.id));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.MeleeWeapon.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.MeleeWeapon.deco2));
    row.appendChild(generateSelectCell(Database.jewel, item.MeleeWeapon.deco3));

    return row;
}

function generateRangedWeapon(item, id) {
    let equipLevel = item.RangedWeapon.lvl;

    const row = document.createElement("tr");
    row.id = id;

    row.appendChild(generateSelectCell(Database.type, item.RangedWeapon.type_id, true));
    row.appendChild(generateIntSelectCell(equipLevel + 1, 0, 32));
    row.appendChild(generateSelectCell(Database[IdToType[[item.RangedWeapon.type_id]]], item.RangedWeapon.id));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.RangedWeapon.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.RangedWeapon.deco2));
    row.appendChild(generateSelectCell(Database.jewel, item.RangedWeapon.deco3));

    return row;
}

function generateItemList(optName, optData) {
    const tbody = document.querySelector(`#${optName} tbody`);
    optData.forEach((data, index) => {
        const row = document.createElement("tr");
        row.id = `${optName}_slot${index}`;
        row.oninput = function(event) {
            if (event.target.tagName.toLowerCase() === "textarea" ) {
                SaveSlot[optName][index].id = Database.item.indexOf(event.target.value);
            }
            else if (event.target.tagName.toLowerCase() === "input")
            {
                SaveSlot[optName][index].qty = event.target.value;
            }
        };

        row.appendChild(generateSelectCell(Database.item, data.id));
        row.appendChild(generateIntSelectCell(data.qty, 0, 99));

        tbody.appendChild(row);
    });
}

function generateEquipBox(optName, optData) {
    const tbody = document.querySelector(`#${optName} tbody`);

    optData.forEach((item, index) => {
        if (item.hasOwnProperty('MeleeWeapon')) {
            tbody.appendChild(generateMeleeWeapon(item, `${optName}_slot${index}`));

        } else if (item.hasOwnProperty('RangedWeapon')) {
            tbody.appendChild(generateRangedWeapon(item, `${optName}_slot${index}`));

        } else if (item.hasOwnProperty('Armor')) {
            tbody.appendChild(generateArmor(item, `${optName}_slot${index}`));

        } else if (item.hasOwnProperty('ZeroSlotTalisman')) {
            tbody.appendChild(generateZeroSlotTalisman(item, `${optName}_slot${index}`));

        } else if (item.hasOwnProperty('OneSlotTalisman')) {
            tbody.appendChild(generateOneSlotTalisman(item, `${optName}_slot${index}`));

        } else if (item.hasOwnProperty('TwoSlotTalisman')) {
            tbody.appendChild(generateTwoSlotTalisman(item, `${optName}_slot${index}`));

        } else if (item.hasOwnProperty('ThreeSlotTalisman')) {
            tbody.appendChild(generateThreeSlotTalisman(item, `${optName}_slot${index}`));

        } else if (item.hasOwnProperty('BlankEquipSlot')) {
            tbody.appendChild(generateBlankEquipSlot(item, `${optName}_slot${index}`));

        } else {
            console.log("Something's Wrong!");
        }
    });
}

function swapPage(target) {
    let pageId = ["melee_pouch", "ranged_pouch", "item_box" ,"equip_box"][parseInt(target.value)];

    if (lastChoice != "" && lastChoice != pageId) {
        document.getElementById(lastChoice).style.display = "none";
    }

    document.getElementById(pageId).style.display = "block";

    lastChoice = pageId;
}

async function loadJson(url) {
    try {
        const response = await fetch(`${window.location.origin}/${url}`);

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const jsonObject = await response.json();

        return jsonObject;
    } catch (error) {
        console.error('Error fetching JSON data:', error);
        return null;
    }
}

Promise.all([
    loadJson('database.json'),
    loadJson('savefile.json')
])
.then(([dataJson, saveJson]) => {
    // Init

    Database = dataJson;
    SaveSlot = saveJson;

    // Read

    const gender = document.getElementById("gender")
    gender.value = SaveSlot.gender;
    gender.onchange = function () {
        SaveSlot.gender = gender.value;
    };

    const name = document.getElementById("name")
    name.value = String.fromCharCode.apply(null, SaveSlot.name);
    name.onchange = function () {
        SaveSlot.name = Array.from(name.value).map(char => char.charCodeAt(0));
    };

    const zenny = document.getElementById("zenny")
    zenny.value = SaveSlot.zenny;
    zenny.onchange = function () {
        SaveSlot.zenny = zenny.value;
    };

    const playtime = document.getElementById("playtime")
    playtime.value = SaveSlot.playtime;
    playtime.onchange = function () {
        SaveSlot.playtime = playtime.value;
    };

    const hrp = document.getElementById("hrp")
    hrp.value = SaveSlot.hrp;
    hrp.onchange = function () {
        SaveSlot.hrp = hrp.value;
    };

    const hr = document.getElementById("hr")
    hr.value = SaveSlot.hr;
    hr.onchange = function () {
        SaveSlot.hr = hr.value;
    };

    const face_type = document.getElementById("face_type")
    face_type.value    = SaveSlot.face_type + 1;
    face_type.onchange = function () {
        SaveSlot.face_type = face_type.value - 1;
    };

    const hair_type = document.getElementById("hair_type")
    hair_type.value    = SaveSlot.hair_type + 1;
    hair_type.onchange = function () {
        SaveSlot.hair_type = hair_type.value - 1;
    };

    const hair_color = document.getElementById("hair_color")
    hair_color.value   = rgbToHex(SaveSlot.hair_color[0], SaveSlot.hair_color[1], SaveSlot.hair_color[2]);
    hair_color.onchange = function () {
        SaveSlot.hair_color = hexToRgb(hair_color.value);
    };

    const cloth_type = document.getElementById("cloth_type")
    cloth_type.value   = SaveSlot.cloth_type + 1;
    cloth_type.onchange = function () {
        SaveSlot.cloth_type = cloth_type.value - 1;
    };

    const voice_type = document.getElementById("voice_type")
    voice_type.value   = SaveSlot.voice_type + 1;
    voice_type.onchange = function () {
        SaveSlot.voice_type = voice_type.value - 1;
    };

    const cloth_color = document.getElementById("cloth_color")
    cloth_color.value  = rgbToHex(SaveSlot.cloth_color[0], SaveSlot.cloth_color[1], SaveSlot.cloth_color[2]);
    cloth_color.onchange = function () {
        SaveSlot.cloth_color = hexToRgb(cloth_color.value);
    };

    const eye_color = document.getElementById("eye_color")
    eye_color.value    = SaveSlot.eye_color + 1;
    eye_color.onchange = function () {
        SaveSlot.eye_color = eye_color.value - 1;
    };

    const feature_type = document.getElementById("feature_type")
    feature_type.value = SaveSlot.feature_type;
    feature_type.onchange = function () {
        SaveSlot.feature_type = feature_type.value;
    };

    const skin_tone = document.getElementById("skin_tone")
    skin_tone.value    = SaveSlot.skin_tone;
    skin_tone.onchange = function () {
        SaveSlot.skin_tone = skin_tone.value;
    };

    generateItemList("melee_pouch", SaveSlot.melee_pouch);
    generateItemList("ranged_pouch", SaveSlot.ranged_pouch);
    generateItemList("item_box", SaveSlot.item_box);
    generateEquipBox("equip_box", SaveSlot.equip_box);

    swapPage(document.getElementById("page"));
})
.catch(error => {
    console.error('An error occurred:', error);
});