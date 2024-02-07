use crate::basics;
use std::collections::HashMap;
use crate::character::Character;

static mut THIEF_SKILLS: Option<HashMap<String, i32>> = None;
static mut LUCK_DIE: i32 = 3;

#[allow(dead_code)]
const THIEF_WEAPON_PROFICIENCY: [&str; 10] = ["blackjack", "blowgun", "crossbow", "dagger", "dart",
                                              "garrote", "longsword", "short sword", "sling", "staff",];

pub fn gen_thief_skills(character: &mut Character) {
    unsafe {
        if THIEF_SKILLS.is_none() {
            THIEF_SKILLS = Some(HashMap::new());
        }
        let thief_skills = THIEF_SKILLS.as_mut().unwrap();

        if character.alignment == 1 {
            thief_skills.insert("backstab".to_string(), 1);
            thief_skills.insert("sneak silently".to_string(), 1);
            thief_skills.insert("hide in shadows".to_string(), 3);
            thief_skills.insert("pick pocket".to_string(), 1);
            thief_skills.insert("climb sheer surfaces".to_string(), 3);
            thief_skills.insert("pick lock".to_string(), 1);
            thief_skills.insert("find trap".to_string(), 3);
            thief_skills.insert("disable trap".to_string(), 3);
            thief_skills.insert("forge document".to_string(), 0);
            thief_skills.insert("disguise self".to_string(), 0);
            thief_skills.insert("read languages".to_string(), 0);
            thief_skills.insert("handle poison".to_string(), 0);
            thief_skills.insert("cast spell from scroll (die)".to_string(), 10);
        } 
        else if character.alignment == 3 {
            thief_skills.insert("backstab".to_string(), 3);
            thief_skills.insert("sneak silently".to_string(), 3);
            thief_skills.insert("hide in shadows".to_string(), 1);
            thief_skills.insert("pick pocket".to_string(), 0);
            thief_skills.insert("climb sheer surfaces".to_string(), 1);
            thief_skills.insert("pick lock".to_string(), 1);
            thief_skills.insert("find trap".to_string(), 1);
            thief_skills.insert("disable trap".to_string(), 0);
            thief_skills.insert("forge document".to_string(), 0);
            thief_skills.insert("disguise self".to_string(), 3);
            thief_skills.insert("read languages".to_string(), 0);
            thief_skills.insert("handle poison".to_string(), 3);
            thief_skills.insert("cast spell from scroll (die)".to_string(), 10);
        } 
        else if character.alignment == 2 {
            thief_skills.insert("backstab".to_string(), 0);
            thief_skills.insert("sneak silently".to_string(), 3);
            thief_skills.insert("hide in shadows".to_string(), 1);
            thief_skills.insert("pick pocket".to_string(), 3);
            thief_skills.insert("climb sheer surfaces".to_string(), 3);
            thief_skills.insert("pick lock".to_string(), 1);
            thief_skills.insert("find trap".to_string(), 1);
            thief_skills.insert("disable trap".to_string(), 1);
            thief_skills.insert("forge document".to_string(), 3);
            thief_skills.insert("disguise self".to_string(), 0);
            thief_skills.insert("read languages".to_string(), 0);
            thief_skills.insert("handle poison".to_string(), 0);
            thief_skills.insert("cast spell from scroll (die)".to_string(), 12);
        }

        // how to access skill mods:
        // if let Some(skill_value) = thief_skills.get("backstab") {
        //     println!("backstab skill modifier: {}", skill_value);
        // }
    }
}

pub unsafe fn print_thief_skills(_ : &mut Character) {
    println!();
    println!("backstab: {}", THIEF_SKILLS.as_ref().unwrap().get("backstab").unwrap());
    println!("sneak silently: {}", THIEF_SKILLS.as_ref().unwrap().get("sneak silently").unwrap());
    println!("hide in shadows: {}", THIEF_SKILLS.as_ref().unwrap().get("hide in shadows").unwrap());
    // println!("pick pocket: {}", THIEF_SKILLS.as_ref().unwrap().get("pick pocket").unwrap());
    // println!("climb sheer surfaces: {}", THIEF_SKILLS.as_ref().unwrap().get("climb sheer surfaces").unwrap());
    // println!("pick lock: {}", THIEF_SKILLS.as_ref().unwrap().get("pick lock").unwrap());
    // println!("find trap: {}", THIEF_SKILLS.as_ref().unwrap().get("find trap").unwrap());
    // println!("disable trap: {}", THIEF_SKILLS.as_ref().unwrap().get("disable trap").unwrap());
    // println!("forge document: {}", THIEF_SKILLS.as_ref().unwrap().get("forge document").unwrap());
    // println!("disguise self: {}", THIEF_SKILLS.as_ref().unwrap().get("disguise self").unwrap());
    // println!("read languages: {}", THIEF_SKILLS.as_ref().unwrap().get("read languages").unwrap());
    // println!("handle poison: {}", THIEF_SKILLS.as_ref().unwrap().get("handle poison").unwrap());
    println!("cast spell from scroll: d{}", THIEF_SKILLS.as_ref().unwrap().get("cast spell from scroll (die)").unwrap());
}

pub unsafe fn level_up(character: &mut Character) {
    character.max_health += basics::roll(1, 6);

    character.agility_modifier += 1; // reflex save
    character.stamina_modifier += 1; // fortitude save
    character.personality_modifier += 1; // willpower save
    
    LUCK_DIE += 1;

    // update the skill values in the THIEF_SKILLS HashMap
    unsafe {
        if let Some(thief_skills) = THIEF_SKILLS.as_mut() {
            if let Some(backstab_skill) = thief_skills.get_mut("backstab") {
                *backstab_skill += 1; 
            }
            if let Some(sneak_silently_skill) = thief_skills.get_mut("sneak silently") {
                *sneak_silently_skill += 1;
            }
            if let Some(hide_in_shadows_skill) = thief_skills.get_mut("hide in shadows") {
                *hide_in_shadows_skill += 1;
            }
            if let Some(pick_pocket_skill) = thief_skills.get_mut("pick pocket") {
                *pick_pocket_skill += 1;
            }
            if let Some(climb_sheer_surfaces_skill) = thief_skills.get_mut("climb sheer surfaces") {
                *climb_sheer_surfaces_skill += 1;
            }
            if let Some(pick_lock_skill) = thief_skills.get_mut("pick lock") {
                *pick_lock_skill += 1;
            }
            if let Some(find_trap_skill) = thief_skills.get_mut("find trap") {
                *find_trap_skill += 1;
            }
            if let Some(disable_trap_skill) = thief_skills.get_mut("disable trap") {
                *disable_trap_skill += 1;
            }
            if let Some(forge_document_skill) = thief_skills.get_mut("forge document") {
                *forge_document_skill += 1;
            }
            if let Some(disguise_self_skill) = thief_skills.get_mut("disguise self") {
                *disguise_self_skill += 1;
            }
            if let Some(read_languages_skill) = thief_skills.get_mut("read languages") {
                *read_languages_skill += 1;
            }
            if let Some(handle_poison_skill) = thief_skills.get_mut("handle poison") {
                *handle_poison_skill += 1;
            }
            if let Some(cast_spell_from_scroll_die_skill) = thief_skills.get_mut("cast spell from scroll (die)") {
                *cast_spell_from_scroll_die_skill += 1;
            }
        }
    }
}

#[allow(dead_code)]
pub unsafe fn burn_luck_on_check(_: &mut Character, burn_amt: i32, roll_value: i32, dc: i32) -> bool {
    let total_roll = roll_value + burn_amt*(basics::roll(1, LUCK_DIE));

    return total_roll >= dc;
}