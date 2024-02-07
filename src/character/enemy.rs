use rand::Rng;
use super::Character;
// use super::combat::weapon::Weapon;

#[allow(dead_code)]
pub struct Enemy {
    pub name: String,
    pub current_health: i32,
    pub max_health: i32,
    pub level: i32,
    pub(super) bonuses: super::Bonuses,
    
    pub armor_class: i32,    
    // pub monster_weapon: Weapon, 
    
    pub fort_save: i32,
    pub ref_save: i32,
    pub will_save: i32,
}

impl Enemy {
    
    /*
    
    enemy creation

     */
    
    pub fn new(character: &mut Character ) -> Enemy {
        let gen = gen_enemies();
        let n = rand::thread_rng().gen_range(1..=9);
        let health = character.level + 2;

        Enemy {
            name: gen[n].0.to_string(),
            current_health: health,
            max_health: health,
            level: 0,
            bonuses: super::Bonuses {
                initiative_bonus: gen[n].1,
                attack_bonus: gen[n].2,
            },
            
            armor_class: gen[n].3,
            // monster_weapon: gen[n].4.clone(),
            
            fort_save: gen[n].4,
            ref_save: gen[n].5,
            will_save: gen[n].6,
        }
    }
}

// pub type Enemies = Vec<(String, i32, i32, i32, Weapon, i32, i32, i32)>;

// pub fn gen_enemies() -> Enemies {
//     vec![
//         ("giant".to_string(), 1, 5, 1, Weapon::rand_new(), 1, 1, 1),
//         ("skeleton".to_string(), 2, 10, 2, Weapon::rand_new(), 2, 2, 2),
//         ("dragon".to_string(), 3, 15, 3, Weapon::rand_new(), 3, 3, 3),
//         ("ghost".to_string(), 1, 5, 1, Weapon::rand_new(), 1, 1, 1),
//         ("golem".to_string(), 2, 10, 2, Weapon::rand_new(), 2, 2, 2),
//         ("witch".to_string(), 3, 15, 3, Weapon::rand_new(), 3, 3, 3),
//         ("mummy".to_string(), 1, 5, 1, Weapon::rand_new(), 1, 1, 1),
//         ("zombie".to_string(), 2, 10, 2, Weapon::rand_new(), 2, 2, 2),
//         ("demon".to_string(), 3, 15, 3, Weapon::rand_new(), 3, 3, 3),
//         ("vampire".to_string(), 1, 5, 1, Weapon::rand_new(), 2, 2, 2),
//     ]
// }

pub type Enemies = Vec<(String, i32, i32, i32, i32, i32, i32)>;

pub fn gen_enemies() -> Enemies {
    vec![
        ("giant".to_string(), 1, 5, 1, 1, 1, 1),
        ("skeleton".to_string(), 2, 10, 2, 2, 2, 2),
        ("dragon".to_string(), 3, 15, 3, 3, 3, 3),
        ("ghost".to_string(), 1, 5, 1, 1, 1, 1),
        ("golem".to_string(), 2, 10, 2, 2, 2, 2),
        ("witch".to_string(), 3, 15, 3, 3, 3, 3),
        ("mummy".to_string(), 1, 5, 1, 1, 1, 1),
        ("zombie".to_string(), 2, 10, 2, 2, 2, 2),
        ("demon".to_string(), 3, 15, 3, 3, 3, 3),
        ("vampire".to_string(), 1, 5, 1, 2, 2, 2),
    ]
}

