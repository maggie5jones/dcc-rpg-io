use rand;
use crate::character;

pub fn forage(character: &mut character::Character) {
    let findings = rand::random::<i32>() % 100;
    match findings {
        0..=10 => {
            if character.current_health + 5 >= character.max_health {
                println!();
                println!("you have found some berries and eaten them, you are now at full health!");
                println!();
                character.current_health = character.max_health;
                return;
            }
            else {
                character.current_health += 5;
                println!();
                println!("you have found some berries and eaten them, you have gained 5 health, your health is now {}!", character.current_health);
                println!();
            }
        }
        11..=30 => {
            if character.current_health <= 5 {
                println!();
                println!("you have found a snake and it bit you, you have lost {} health, your health is now {}!", character.current_health, 0);
                println!();
                character.current_health = 0;
                return;
            }
            else {
                character.current_health -= 5;
                println!();
                println!("you have found a snake and it bit you, you have lost 5 health, your health is now {}!", character.current_health);
                println!();
            }
        }
        31..=70 => {
            println!();
            println!("you found a new weapon!");
            let weapon = character::combat::weapon::Weapon::rand_new();
            character.weapon = Some(weapon);
            character::combat::weapon::print_weapon(&character.weapon);
            println!();
        }
        71..=90 => {
            let mut enemy = character::enemy::Enemy::new(character);
            println!();
            println!("you ran into a {}!", enemy.name);
            character::combat::fight(character, &mut enemy);
        }
        _ => {
            println!();
            println!("you found nothing, better luck next time!");
            println!();
        }
    }
}
