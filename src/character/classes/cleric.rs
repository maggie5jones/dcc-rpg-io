use crate::basics;
use crate::character::Character;
use rand::Rng;

pub fn print_cleric_shit(character: &mut Character) {
    println!();
    println!("your patron is: {}", get_patron(character));
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

pub fn level_up(character: &mut Character) {
    character.max_health += basics::roll(1, 8);

    character.strength += basics::roll(1, 3);
    character.agility += basics::roll(1, 3);
    character.stamina += basics::roll(1, 3);
    character.personality += basics::roll(1, 3);
    character.intelligence += basics::roll(1, 3);
    character.luck += basics::roll(1, 3);
}