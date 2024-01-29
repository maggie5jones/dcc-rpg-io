use rand::Rng;
use super::enemy;

pub fn fight(character: &mut super::Character, enemy: &mut enemy::Enemy) {
    println!("You are fighting a {}!", enemy.name);
    
    loop {
        println!("Your health: {}", character.current_health);
        println!("Enemy health: {}", enemy.health);
    
        // player's turn
        let player_damage = rand::thread_rng().gen_range(1..=enemy.health);
        enemy.health = enemy.health.saturating_sub(player_damage);
        println!("You attacked the {} for {} damage!", enemy.name, player_damage);
    
        if enemy.health <= 0 {
            println!("You have defeated the {}!", enemy.name);
            character.xp += enemy.health/2;
            character.check_level_up();
            break;
        }
    
        // enemy's turn
        let enemy_damage = rand::thread_rng().gen_range(1..=character.current_health);
        character.current_health = character.current_health.saturating_sub(enemy_damage);
        println!("The {} attacked you for {} damage!", enemy.name, enemy_damage);
    
        if character.current_health <= 0 {
            println!("You have been defeated!");
            break;
        }
    }
}