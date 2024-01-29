mod character;

use character::basics;
use character::enemy;
use character::combat;

fn main() {
    println!("welcome to the mini dungeon crawler!");

    let mut character = character::Character::new();
    character.create();

    loop {
        println!("what would you like to do?");
        println!("1. fight");
        println!("2. quick rest");
        println!("3. super long rest to full health");
        println!("4. look deep in a mirror (check your stats)");
        println!("5. quit");

        let choice = basics::get_user_input().parse::<u32>().unwrap();

        match choice {
            1 => {
                let mut enemy = enemy::Enemy::new(character.level*basics::roll(1, 8), 
                                                  character.level+basics::roll(1, 8));
                combat::fight(&mut character, &mut enemy);
            }
            2 => {
                character.rest();
                println!("you have rested and regained some health, your health is now {}", character.current_health);
            }
            3 => {
                character.current_health = character.max_health;
                println!("you have rested and regained all of your health... better luck on the next battle");
            }
            4 => {
                println!("these are your attributes:");
                println!("strength: {}  mod: {}", character.strength, character.strength_modifier);
                println!("agility: {}   mod: {}", character.agility, character.agility_modifier);
                println!("stamina: {}  mod: {}", character.stamina, character.stamina_modifier);
                println!("personality: {}  mod: {}", character.personality, character.personality_modifier);
                println!("intelligence: {}  mod: {}", character.intelligence, character.intelligence_modifier);
                println!("luck: {}  mod: {}", character.luck, character.luck_modifier);
                println!(" ");
                println!("these are your stats:");
                println!("name: {}", character.name);
                println!("class: {}", character.get_class_name());
                println!("level: {}", character.level);
                println!("xp: {}", character.xp);
                println!("health: {}", character.current_health);
                println!(" ");
            }
            5 => {
                println!("goodbye!");
                break;
            }
            _ => {
                println!("invalid choice :( please try again!");
            }
        }
    }
}