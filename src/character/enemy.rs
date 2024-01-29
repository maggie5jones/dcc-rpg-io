use rand::Rng;

#[allow(dead_code)]
pub struct Enemy {
    pub name: String,
    pub health: u32,
    pub attack_power: u32,
}

impl Enemy {
    
    /*
    
    enemy creation

     */
    
    pub fn new(health: u32, attack_power:u32) -> Enemy {
        let names = vec!["Goblin", "Orc", "Troll", "Skeleton"];
        let name = names[rand::thread_rng().gen_range(0..names.len())];
        // let health = rand::thread_rng().gen_range(30..71);
        // let attack_power = rand::thread_rng().gen_range(3..8);

        Enemy {
            name: String::from(name),
            health: health,
            attack_power: attack_power,
        }
    }
}