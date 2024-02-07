use super::enemy;
use std::io;

pub mod weapon;

pub fn fight(character: &mut super::Character, enemy: &mut enemy::Enemy) {
    println!();
    println!("you are fighting a {}!", enemy.name);
    enemy.current_health = enemy.max_health;

    let player_initiative = character.bonuses.initiative_bonus + super::basics::roll(1, 20);
    let enemy_initiative = enemy.bonuses.initiative_bonus + super::basics::roll(1, 20);

    combat(character, enemy, player_initiative, enemy_initiative);

    if character.current_health <= 0 {
        println!();
        println!("better luck next time!");
        println!();
    }
}

fn player_turn(character: &mut super::Character, enemy: &mut enemy::Enemy) -> i32 {
    println!();
    println!("it's your turn, what would you like to do?");
    println!("1. attack");
    println!("2. run away");
    println!();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let mut player_damage;

    match input.trim() {
        "1" => {
            if let Some(weapon) = &character.weapon {
                let player_weapon_damage = weapon.damage;
                player_damage = player_weapon_damage + character.bonuses.attack_bonus;
            } else {
                player_damage = 1 + character.bonuses.attack_bonus;
            }

            let roll_to_hit = super::basics::roll(1, 20) + character.bonuses.attack_bonus;
            if roll_to_hit >= enemy.armor_class {
                println!("you hit the {}!", enemy.name);
                
                if character.class == 6 {
                    player_damage += 5;
                }
            } else {
                println!("you missed the {}!", enemy.name);
                return 1;
            }
            enemy.current_health = enemy.current_health.saturating_sub(player_damage);
            println!("you attacked the {} for {} damage!", enemy.name, player_damage);
            println!();
            return 1;
        },
        "2" => {
            let chance = super::basics::roll(1, 20) + character.agility_modifier;
            let enemy_opportunity_attack = super::basics::roll(1, 20) + enemy.bonuses.attack_bonus;
            if chance >= enemy_opportunity_attack { // character successfully runs away
                println!("you successfully ran away!");
                return 0;
            } else {
                println!("you tried to run away, but the {} hit you on your way out!", enemy.name);
                enemy_turn(character, enemy);
                return 0;
            }
        }
        _ => {
            println!("invalid input :( try again!");
            return 1;
        }
    }
}

fn enemy_turn(character: &mut super::Character, enemy: &mut enemy::Enemy) {
    println!();
    println!("enemy's turn:");

    // let enemy_weapon_damage;
    let enemy_damage = character.level + 1;
    let roll_to_hit = super::basics::roll(1, 20) + enemy.bonuses.attack_bonus;
    if roll_to_hit >= enemy.armor_class {
        println!("the {} hit you!", enemy.name);
        // enemy_damage = super::basics::roll(enemy_weapon_damage.0, enemy_weapon_damage.1) + enemy.bonuses.attack_bonus;
        // enemy_damage = enemy_weapon_damage;
    } else {
        println!("the {} missed you!", enemy.name);
        return;
    }
    character.current_health = character.current_health.saturating_sub(enemy_damage);
    println!("the {} attacked you for {} damage!", enemy.name, enemy_damage);
    println!();
}

fn combat(character: &mut super::Character, enemy: &mut enemy::Enemy, player_initiative: i32, enemy_initiative: i32) {
    if player_initiative >= enemy_initiative {
        println!("you go first!");
        println!();
        loop {
            println!("your health: {}", character.current_health);
            println!("enemy health: {}", enemy.current_health);
        
            let val = player_turn(character, enemy);
            if val == 0 {
                break;
            }

            if enemy.current_health <= 0 {
                println!();
                println!("you have defeated the {}!", enemy.name);
                character.xp += enemy.max_health/2;
                character.check_level_up();
                break;
            }
            
            enemy_turn(character, enemy);
            if character.current_health <= 0 {
                println!("you have been defeated!");
                break;
            }
        }
    } else {
        println!("the {} goes first!", enemy.name);
        println!();
        loop {
            println!("your health: {}", character.current_health);
            println!("enemy health: {}", enemy.current_health);
            println!();
        
            enemy_turn(character, enemy);
            if character.current_health <= 0 {
                println!("you have been defeated!");
                break;
            }

            let val = player_turn(character, enemy);
            if val == 0 {
                break;
            }
            
            if enemy.current_health <= 0 {
                println!("you have defeated the {}!", enemy.name);
                character.xp += enemy.max_health/2;
                break;
            }
        }
    }
}