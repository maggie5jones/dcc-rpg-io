// 7
use crate::basics;
use crate::character::Character;

pub fn print_elf_shit(character: &mut Character) {
    println!();
    println!("you know {} spell(s) and get a +2 bonus to your max health!", get_spell_count(character));
    character.max_health += 2;
    character.current_health = character.max_health;
    //TODO: impliment magic :)
}

pub fn get_spell_count(character: &mut Character) -> i32 {
    let mut spell_count = 0;
    match character.level {
        1 => spell_count = 2,
        2 => spell_count = 3,
        3 => spell_count = 4,
        4 => spell_count = 5,
        5 => spell_count = 6,
        6 => spell_count = 7,
        7 => spell_count = 8,
        8 => spell_count = 9,
        9 => spell_count = 10,
        10 => spell_count = 11,
        _ => println!("error: unknown level"),
    }
    spell_count
}

pub fn level_up(character: &mut Character) {
    character.max_health += basics::roll(1, 6);

    character.strength += basics::roll(1, 3);
    character.agility += basics::roll(1, 3);
    character.stamina += basics::roll(1, 3);
    character.personality += basics::roll(1, 3);
    character.intelligence += basics::roll(1, 3);
    character.luck += basics::roll(1, 3);
}