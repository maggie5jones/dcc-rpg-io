// 5
use crate::basics;
use crate::character::Character;

pub fn print_dwarf_shit(_character: &mut Character) {
    println!();
    println!("may you do many mighty deeds!");
    //TODO: impliment mighty deeds :)
}

pub fn level_up(character: &mut Character) {
    character.max_health += basics::roll(1, 10);

    character.strength += basics::roll(1, 3);
    character.agility += basics::roll(1, 3);
    character.stamina += basics::roll(1, 3);
    character.personality += basics::roll(1, 3);
    character.intelligence += basics::roll(1, 3);
    character.luck += basics::roll(1, 3);
}