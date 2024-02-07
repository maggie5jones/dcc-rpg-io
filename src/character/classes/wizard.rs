// 4
use rand::Rng;
use crate::basics;
use crate::character::Character;

pub fn print_wizard_shit(character: &mut Character) {
    println!();
    println!("your patron is {}, and you know {} spell(s)!", get_patron(character), get_spell_count(character));
    // TODO: implement magic :)
}

#[allow(dead_code)]
pub fn get_patron(character: &mut Character) -> String {
    let aln = character.alignment;
    let mut patron = "";
    match aln {
        1 => {
            patron = match rand::thread_rng().gen_range(1..=3) {
                1 => "shul, god of the moon",
                2 => "klazath, god of war",
                3 => "daenthar, the mountain lord, greater god of earth and industry",
                _ => "unknown patron",
            }
        }
        2 => {
            patron = match rand::thread_rng().gen_range(1..=3) {
                1 => "cthulhu, priest of the old ones",
                2 => "amun tor, god of mysteries and riddles",
                3 => "pelagia, goddess of the sea",
                _ => "unknown patron",
            }
        }
        3 => {
            patron = match rand::thread_rng().gen_range(1..=3) {
                1 => "bobugbubilz, demon lord of evil amphibians",
                2 => "azi dahaka, demon prince of storms and waste",
                3 => "ahriman, god of death and disease",
                _ => "unknown patron",
            }
        }
        _ => {
            println!("error: unknown alignment")
        }
    }
    patron.to_string()
}

pub fn get_spell_count(character: &mut Character) -> i32 {
    let mut spell_count = 0;
    match character.level {
        1 => spell_count = 1,
        2 => spell_count = 2,
        3 => spell_count = 3,
        4 => spell_count = 4,
        5 => spell_count = 5,
        6 => spell_count = 6,
        7 => spell_count = 7,
        8 => spell_count = 8,
        9 => spell_count = 9,
        10 => spell_count = 10,
        _ => println!("error: unknown level"),
    }
    spell_count
}

pub fn level_up(character: &mut Character) {
    character.max_health += basics::roll(1, 4);

    character.strength += basics::roll(1, 3);
    character.agility += basics::roll(1, 3);
    character.stamina += basics::roll(1, 3);
    character.personality += basics::roll(1, 3);
    character.intelligence += basics::roll(1, 3);
    character.luck += basics::roll(1, 3);

    // TODO: update max spell level
}