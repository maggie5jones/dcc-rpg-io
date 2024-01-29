// 4
use super::basics;

pub fn level_up(character: &mut super::Character) {
    character.max_health += basics::roll(1, 4);

    character.strength += basics::roll(1, 3);
    character.agility += basics::roll(1, 3);
    character.stamina += basics::roll(1, 3);
    character.personality += basics::roll(1, 3);
    character.intelligence += basics::roll(1, 3);
    character.luck += basics::roll(1, 3);

    // TODO: update max spell level
}