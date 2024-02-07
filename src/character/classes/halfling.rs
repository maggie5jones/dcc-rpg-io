// 6
use crate::basics;
use crate::character::Character;

pub fn print_halfling_shit(_character: &mut Character) {
    println!();
    println!("you can fight with two weapons!");
    //TODO: impliment two-weapon fighting! (for now halflings just do 5 extra damage each attack)
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