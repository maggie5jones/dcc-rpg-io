mod character;
mod forage;

use character::basics;
use character::enemy;
use character::combat;

//TODO: implement threading? (avoid stack overflow)

fn main() {
    println!();
    println!("welcome to the mini dungeon crawler!");    

    let mut character = character::Character::new();
    character.create();
    character.current_health = character.max_health;

    loop {
        println!();
        println!("your health: {}", character.current_health);
        println!();
        println!("what would you like to do?");
        println!("1. fight");
        println!("2. forage");
        println!("3. rest & relaxation! (regain full health)");
        println!("4. look deep in a mirror (check your stats)");
        println!("5. quit");
        println!();

        let choice = basics::get_user_input().parse::<i32>().unwrap();

        match choice {
            1 => {
                if character.current_health <= 0 {
                    println!();
                    println!("you are dead, you cannot fight");
                    continue;
                }
                else {
                    let mut enemy = enemy::Enemy::new(&mut character);
                    combat::fight(&mut character, &mut enemy);
                }
            }
            // 2 => {
            //     character.rest();
            //     println!();
            //     println!("you have rested and regained some health, your health is now {}", character.current_health);
            // }
            2 => {
                if character.current_health <= 0 {
                    println!();
                    println!("you are dead, you cannot forage");
                    continue;
                }
                else {
                    forage::forage(&mut character);
                    println!();
                }
            }

            3 => {
                character.current_health = character.max_health;
                println!();
                println!("you have rested and regained all of your health... better luck on the next battle");
            }
            4 => {
                println!();
                println!("these are your attributes:");
                println!("strength: {}  mod: {}", character.strength, character.strength_modifier);
                println!("agility: {}   mod: {}", character.agility, character.agility_modifier);
                println!("stamina: {}  mod: {}", character.stamina, character.stamina_modifier);
                println!("personality: {}  mod: {}", character.personality, character.personality_modifier);
                println!("intelligence: {}  mod: {}", character.intelligence, character.intelligence_modifier);
                println!("luck: {}  mod: {}", character.luck, character.luck_modifier);
                println!();
                println!("these are your stats:");
                println!("name: {}", character.name);
                println!("class: {}", character.get_class_name());
                println!("level: {}", character.level);
                println!("xp: {}", character.xp);
                println!("health: {}", character.current_health);
                println!(" ");
            }
            5 => {
                println!();
                println!("goodbye!");
                println!();
                break;
            }
            _ => {
                println!();
                println!("invalid choice :( please try again!");
            }
        }
    }
}