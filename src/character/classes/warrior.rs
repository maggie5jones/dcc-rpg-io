use crate::basics;
use crate::character::Character;

static mut DEED_DIE: i32 = 3; // applies to attach AND damage rolls

#[allow(dead_code)]
const WARRIOR_WEAPON_PROFICIENCY: [&str; 18] = ["battleaxe", "club", "crossbow", "dagger", "dart", "handaxe", 
                                                "javelin", "longbow", "longsword","mace", "polearm", "shortbow", 
                                                "short sword", "sling", "spear", "staff", "two-handed sword", "warhammer"];

pub fn print_warrior_shit(_character: &mut Character) {
    println!();
    println!("may you do many mighty deeds!");
    //TODO: impliment mighty deeds :)
}
                                                
pub unsafe fn level_up(character: &mut Character) {
    DEED_DIE += 1;
    
    character.max_health += basics::roll(1, 12);

    character.strength += basics::roll(1, 3);
    character.agility += basics::roll(1, 3);
    character.stamina += basics::roll(1, 3);
    character.personality += basics::roll(1, 3);
    character.intelligence += basics::roll(1, 3);
    character.luck += basics::roll(1, 3);
}