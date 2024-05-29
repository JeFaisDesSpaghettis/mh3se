let lastChoice = "";

let typeIdOptList = [];
let equipOptList = [];
let jewelOptList = [];
let skillOptList = [];

let IdToName = {};
let SaveSlot = {};

const pages = document.getElementById("pages");

const TypeIdToField = [
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

function onTypeChange(target) {
    let typeId = IdToName.type.indexOf(target.value);

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
        }));
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
        }));
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
        }));
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
        }));
    }
    else {
        row.replaceWith(generateBlankEquipSlot({
            BlankEquipSlot: {
                buf: [0,0,0,0,0,0,0,0,0,0,0,0]
            }
        }));
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
            }));
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
            }));
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
            }));
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
            }));
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
    blankText.innerHTML = 'X';
    blankCell.appendChild(blankText);
    return blankCell;
}

function generateSelectCell(optList, optSelect)
{
    const selectCell = document.createElement("td");
    const selectSelect = document.createElement("select");
    selectSelect.innerHTML = optList;
    selectSelect.value = optSelect;
    selectCell.appendChild(selectSelect);
    return selectCell;
}

function generateIntSelectCell(value, min, max)
{
    const intSelectCell = document.createElement("td");
    const intSelectInput = document.createElement("input");
    intSelectInput.type = "number";
    intSelectInput.min = min;
    intSelectInput.max = max;
    intSelectInput.value = value;
    intSelectCell.appendChild(intSelectInput);
    return intSelectCell;
}

function generateSlotCountCell(slotsCount)
{
    const slotCountCell = document.createElement("td");
    const slotCountInput = document.createElement("input");
    slotCountInput.onchange = function () {
        onSlotCountChange(slotCountInput);
    };
    slotCountInput.type = "number";
    slotCountInput.min = 0;
    slotCountInput.max = 3;
    slotCountInput.value = slotsCount;
    slotCountCell.appendChild(slotCountInput);

    return slotCountCell;
}

function generateTypeCell(typeName)
{
    const typeCell = document.createElement("td");
    const typeSelect = document.createElement("select");

    typeSelect.onchange = function () {
        onTypeChange(this);
    };

    typeSelect.innerHTML = typeIdOptList;
    typeSelect.value = typeName;
    typeCell.appendChild(typeSelect);

    return typeCell;
}

function generateBlankEquipSlot(item) {
    let typeName = IdToName.type[item.BlankEquipSlot.type_id];

    const row = document.createElement("tr");

    const typeCell = document.createElement("td");
    const typeSelect = document.createElement("select");

    typeSelect.onchange = function () {
        onTypeChange(this);
    };

    typeSelect.innerHTML = typeIdOptList;
    typeSelect.value = typeName;
    typeCell.appendChild(typeSelect);

    row.appendChild(typeCell);

    for (let i = 0; i < 7; i++) {
        row.appendChild(generateBlankCell());
    }

    return row;
}

function generateOneSlotTalisman(item) {
    let typeName = IdToName.type[item.OneSlotTalisman.type_id];
    let equipName = IdToName[TypeIdToField[item.OneSlotTalisman.type_id]][item.OneSlotTalisman.id];
    let slotsCount = item.OneSlotTalisman.slot_count;
    let skill1Pt = item.OneSlotTalisman.skill1_pt;
    let skill2Pt = item.OneSlotTalisman.skill2_pt;
    let skill1Name = IdToName.skill[item.OneSlotTalisman.skill1_id];
    let skill2Name = IdToName.skill[item.OneSlotTalisman.skill2_id];
    let deco1Name = IdToName.jewel[item.OneSlotTalisman.deco1];

    const row = document.createElement("tr");

    row.appendChild(generateTypeCell(typeName));
    row.appendChild(generateSlotCountCell(slotsCount));
    row.appendChild(generateSelectCell(equipOptList[item.OneSlotTalisman.type_id], equipName));
    row.appendChild(generateIntSelectCell(skill1Pt - 10, -10, 245));
    row.appendChild(generateIntSelectCell(skill2Pt - 10, -10, 245));
    row.appendChild(generateSelectCell(jewelOptList, deco1Name));
    row.appendChild(generateSelectCell(skillOptList, skill1Name));
    row.appendChild(generateSelectCell(skillOptList, skill2Name));

    return row;
}

function generateZeroSlotTalisman(item) {
    let typeName = IdToName.type[item.ZeroSlotTalisman.type_id];
    let equipName = IdToName[TypeIdToField[item.ZeroSlotTalisman.type_id]][item.ZeroSlotTalisman.id];
    let slotsCount = item.ZeroSlotTalisman.slot_count;
    let skill1Pt = item.ZeroSlotTalisman.skill1_pt;
    let skill2Pt = item.ZeroSlotTalisman.skill2_pt;
    let skill1Name = IdToName.skill[item.ZeroSlotTalisman.skill1_id];
    let skill2Name = IdToName.skill[item.ZeroSlotTalisman.skill2_id];

    const row = document.createElement("tr");

    row.appendChild(generateTypeCell(typeName));
    row.appendChild(generateSlotCountCell(slotsCount));
    row.appendChild(generateSelectCell(equipOptList[item.ZeroSlotTalisman.type_id], equipName));
    row.appendChild(generateIntSelectCell(skill1Pt - 10, -10, 245));
    row.appendChild(generateIntSelectCell(skill2Pt - 10, -10, 245));
    row.appendChild(generateSelectCell(skillOptList, skill1Name));
    row.appendChild(generateSelectCell(skillOptList, skill2Name));
    row.appendChild(generateBlankCell());

    return row;
}

function generateTwoSlotTalisman(item) {
    let typeName = IdToName.type[item.TwoSlotTalisman.type_id];
    let equipName = IdToName[TypeIdToField[item.TwoSlotTalisman.type_id]][item.TwoSlotTalisman.id];
    let slotsCount = item.TwoSlotTalisman.slot_count;
    let skill1Pt = item.TwoSlotTalisman.skill1_pt;
    let skill1Name = IdToName.skill[item.TwoSlotTalisman.skill1_id];
    let deco1Name = IdToName.jewel[item.TwoSlotTalisman.deco1];
    let deco2Name = IdToName.jewel[item.TwoSlotTalisman.deco2];

    const row = document.createElement("tr");

    row.appendChild(generateTypeCell(typeName));
    row.appendChild(generateSlotCountCell(slotsCount));
    row.appendChild(generateSelectCell(equipOptList[item.TwoSlotTalisman.type_id], equipName));
    row.appendChild(generateIntSelectCell(skill1Pt - 10, -10, 245));
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(jewelOptList, deco1Name));
    row.appendChild(generateSelectCell(jewelOptList, deco2Name));
    row.appendChild(generateSelectCell(skillOptList, skill1Name));

    return row;
}

function generateThreeSlotTalisman(item) {
    let typeName = IdToName.type[item.ThreeSlotTalisman.type_id];
    let equipName = IdToName[TypeIdToField[item.ThreeSlotTalisman.type_id]][item.ThreeSlotTalisman.id];
    let slotsCount = item.ThreeSlotTalisman.slot_count;
    let deco1Name = IdToName.jewel[item.ThreeSlotTalisman.deco1];
    let deco2Name = IdToName.jewel[item.ThreeSlotTalisman.deco2];
    let deco3Name = IdToName.jewel[item.ThreeSlotTalisman.deco3];

    const row = document.createElement("tr");

    row.appendChild(generateTypeCell(typeName));
    row.appendChild(generateSlotCountCell(slotsCount));
    row.appendChild(generateSelectCell(equipOptList[item.ThreeSlotTalisman.type_id], equipName));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(jewelOptList, deco1Name));
    row.appendChild(generateSelectCell(jewelOptList, deco2Name));
    row.appendChild(generateSelectCell(jewelOptList, deco3Name));

    return row;
}

function generateArmor(item) {
    let typeName = IdToName.type[item.Armor.type_id];
    let equipName = IdToName[TypeIdToField[item.Armor.type_id]][item.Armor.id];
    let equipLevel = item.Armor.lvl;
    let deco1Name = IdToName.jewel[item.Armor.deco1];
    let deco2Name = IdToName.jewel[item.Armor.deco2];
    let deco3Name = IdToName.jewel[item.Armor.deco3];

    const row = document.createElement("tr");

    row.appendChild(generateTypeCell(typeName));
    row.appendChild(generateIntSelectCell(equipLevel + 1, 0, 32));
    row.appendChild(generateSelectCell(equipOptList[item.Armor.type_id], equipName));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(jewelOptList, deco1Name));
    row.appendChild(generateSelectCell(jewelOptList, deco2Name));
    row.appendChild(generateSelectCell(jewelOptList, deco3Name));

    return row;
}

function generateMeleeWeapon(item) {
    let typeName = IdToName.type[item.MeleeWeapon.type_id];
    let equipName = IdToName[TypeIdToField[item.MeleeWeapon.type_id]][item.MeleeWeapon.id];
    let deco1Name = IdToName.jewel[item.MeleeWeapon.deco1];
    let deco2Name = IdToName.jewel[item.MeleeWeapon.deco2];
    let deco3Name = IdToName.jewel[item.MeleeWeapon.deco3];

    const row = document.createElement("tr");

    row.appendChild(generateTypeCell(typeName));
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(equipOptList[item.MeleeWeapon.type_id], equipName));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(jewelOptList, deco1Name));
    row.appendChild(generateSelectCell(jewelOptList, deco2Name));
    row.appendChild(generateSelectCell(jewelOptList, deco3Name));

    return row;
}

function generateRangedWeapon(item) {
    let typeName = IdToName.type[item.RangedWeapon.type_id];
    let equipName = IdToName[TypeIdToField[item.RangedWeapon.type_id]][item.RangedWeapon.id];
    let equipLevel = item.RangedWeapon.lvl;
    let deco1Name = IdToName.jewel[item.RangedWeapon.deco1];
    let deco2Name = IdToName.jewel[item.RangedWeapon.deco2];
    let deco3Name = IdToName.jewel[item.RangedWeapon.deco3];

    const row = document.createElement("tr");

    row.appendChild(generateTypeCell(typeName));
    row.appendChild(generateIntSelectCell(equipLevel + 1, 0, 32));
    row.appendChild(generateSelectCell(equipOptList[item.RangedWeapon.type_id], equipName));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(jewelOptList, deco1Name));
    row.appendChild(generateSelectCell(jewelOptList, deco2Name));
    row.appendChild(generateSelectCell(jewelOptList, deco3Name));

    return row;
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

function generateItemList(optName, optData) {
    const tbody = document.querySelector(`#${optName} tbody`);

    const itemOptList = IdToName.item.map(itemName => `<option value="${itemName}">${itemName}</option>`).join('');

    optData.forEach(data => {
        const row = document.createElement("tr");

        const itemCell = document.createElement("td");
        const itemSelect = document.createElement("select");
        itemSelect.innerHTML = itemOptList
        itemSelect.value = IdToName.item[data.id];
        itemCell.appendChild(itemSelect);

        const quantityCell = document.createElement("td");
        const quantityInput = document.createElement("input");
        quantityInput.type = "number";
        quantityInput.value = data.qty;
        quantityCell.appendChild(quantityInput);

        row.appendChild(itemCell);
        row.appendChild(quantityCell);

        tbody.appendChild(row);
    });
}

function generateEquipBox(optName, optData) {
    const tbody = document.querySelector(`#${optName} tbody`);

    optData.forEach(item => {
        if (item.hasOwnProperty('MeleeWeapon')) {
            tbody.appendChild(generateMeleeWeapon(item));

        } else if (item.hasOwnProperty('RangedWeapon')) {
            tbody.appendChild(generateRangedWeapon(item));

        } else if (item.hasOwnProperty('Armor')) {
            tbody.appendChild(generateArmor(item));

        } else if (item.hasOwnProperty('ZeroSlotTalisman')) {
            tbody.appendChild(generateZeroSlotTalisman(item));

        } else if (item.hasOwnProperty('OneSlotTalisman')) {
            tbody.appendChild(generateOneSlotTalisman(item));

        } else if (item.hasOwnProperty('TwoSlotTalisman')) {
            tbody.appendChild(generateTwoSlotTalisman(item));

        } else if (item.hasOwnProperty('ThreeSlotTalisman')) {
            tbody.appendChild(generateThreeSlotTalisman(item));

        } else if (item.hasOwnProperty('BlankEquipSlot')) {
            tbody.appendChild(generateBlankEquipSlot(item));

        } else {
            console.log("Something's Wrong!");
        }
    });
}

function swapPage(target) {
    let choice = parseInt(target.value);
    let pageId = null;
    let pageData = null;
    let pageFct = null;

    switch (choice) {
        case 1:
        case 2:
        case 3: {
            pageId = `mpouch${choice}_opt`;
            pageData = SaveSlot.melee_pouch.slice(8 * (choice - 1), 8 * choice);
            pageFct = generateItemList;
            break;
        }
        case 4:
        case 5:
        case 6:
        case 7: {
            pageId = `rpouch${choice - 3}_opt`;
            pageData = SaveSlot.ranged_pouch.slice(8 * (choice - 4), 8 * (choice - 3));
            pageFct = generateItemList;
            break;
        }
        case 8:
        case 9:
        case 10:
        case 11:
        case 12:
        case 13:
        case 14:
        case 15: {
            pageId = `ibox${choice - 7}_opt`;
            pageData = SaveSlot.item_box.slice(100 * (choice - 8), 100 * (choice - 7));
            pageFct = generateItemList;
            break;
        }
        case 16:
        case 17:
        case 18:
        case 19:
        case 20:
        case 21:
        case 22:
        case 23: {
            pageId = `ebox${choice - 15}_opt`;
            pageData = SaveSlot.equip_box.slice(100 * (choice - 16), 100 * (choice - 15));
            pageFct = generateEquipBox;
            break;
        }
        default:
            return;
    }

    // To be finished...
    if (lastChoice != "" && lastChoice != pageId) {
        let lastTable = document.getElementById(lastChoice);
        lastTable.style.display = "none";
    }
    let table = document.getElementById(pageId);
    if (table.getAttribute("init") == "no") {
        table.setAttribute("init", "yes");
        pageFct(pageId, pageData);
    }
    table.style.display = "block";
    lastChoice = pageId;
}


Promise.all([
    loadJson('data.json'),
    loadJson('save.json')
])
.then(([dataJson, saveJson]) => {
    IdToName = dataJson;
    SaveSlot = saveJson;

    typeIdOptList = IdToName.type.map(typeName => `<option value="${typeName}">${typeName}</option>`).join('');
    equipOptList = new Array(16);
    for (let i = 1; i <= 15; i++) {
        equipOptList[i] = IdToName[TypeIdToField[i]].map(equipName => `<option value="${equipName}">${equipName}</option>`).join('');
    }
    jewelOptList = IdToName.jewel.map(decoName => `<option value="${decoName}">${decoName}</option>`).join('');
    skillOptList = IdToName.skill.map(skillName => `<option value="${skillName}">${skillName}</option>`).join('');

    const gender_opt = document.getElementById("gender_opt");
    gender_opt.innerHTML = "";

    IdToName.gender.forEach(option => {
        const optionElement = document.createElement("option");
        optionElement.textContent = option;
        gender_opt.appendChild(optionElement);
    });

    gender_opt.value = IdToName.gender[SaveSlot.gender];

    document.getElementById("name_opt").value =
        String.fromCharCode.apply(null, SaveSlot.name);
    document.getElementById("zenny_opt").value =
        SaveSlot.zenny;
    document.getElementById("playtime_opt").value =
        SaveSlot.playtime;

    swapPage(document.getElementById("page_opt"));
})
.catch(error => {
    console.error('An error occurred:', error);
});