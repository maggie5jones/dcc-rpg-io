use rand::{self, seq::IteratorRandom};
use core::fmt;
use std::slice::Iter;

use crate::character::basics;

pub struct Weapon {
    pub name: String,
    pub weapon_type: WeaponType,
    pub damage: i32,
    pub crit_range: (i32, i32),
    pub crit_multiplier: i32,
}

#[allow(dead_code)]
impl Weapon {
    pub fn rand_new() -> Weapon {
        let mut rng = rand::thread_rng();
        let w_type = WeaponType::iter().choose(&mut rng).unwrap();
        let w_info = get_weapon_info(w_type.clone());
        Weapon {
            name: w_info.name.clone(),
            weapon_type: w_info.weapon_type.clone(),
            damage: w_info.damage,
            crit_range: w_info.crit_range,
            crit_multiplier: w_info.crit_multiplier,
        }
    }

    pub fn sword() -> Weapon {
        Weapon {
            name: String::from("sword"),
            weapon_type: WeaponType::Sword,
            damage: basics::roll(1, 4),
            crit_range: (19, 20),
            crit_multiplier: 1,
        }
    }
    pub fn axe() -> Weapon {
        Weapon {
            name: String::from("axe"),
            weapon_type: WeaponType::Axe,
            damage: basics::roll(1, 10),
            crit_range: (18, 20),
            crit_multiplier: 1,
        }
    }
    pub fn dagger() -> Weapon {
        Weapon {
            name: String::from("dagger"),
            weapon_type: WeaponType::Dagger,
            damage: basics::roll(1, 10),
            crit_range: (20, 20),
            crit_multiplier: 1,
        }
    }
    pub fn bow() -> Weapon {
        Weapon {
            name: String::from("bow"),
            weapon_type: WeaponType::Bow,
            damage: basics::roll(1, 6),
            crit_range: (20, 20),
            crit_multiplier: 1,
        }
    }
    pub fn staff() -> Weapon {
        Weapon {
            name: String::from("staff"),
            weapon_type: WeaponType::Staff,
            damage: basics::roll(1, 4),
            crit_range: (19, 20),
            crit_multiplier: 1,
        }
    }
    pub fn club() -> Weapon {
        Weapon {
            name: String::from("club"),
            weapon_type: WeaponType::Club,
            damage: basics::roll(1, 4),
            crit_range: (18, 20),
            crit_multiplier: 1,
        }
    }
    pub fn shield() -> Weapon {
        Weapon {
            name: String::from("shield"),
            weapon_type: WeaponType::Shield,
            damage: basics::roll(1, 6),
            crit_range: (19, 20),
            crit_multiplier: 1,
        }
    }
}

pub fn get_weapon_info(w_type: WeaponType) -> Weapon {
    match w_type {
        WeaponType::Sword => Weapon::sword(),
        WeaponType::Axe => Weapon::axe(),
        WeaponType::Dagger => Weapon::dagger(),
        WeaponType::Bow => Weapon::bow(),
        WeaponType::Staff => Weapon::staff(),
        WeaponType::Club => Weapon::club(),
        WeaponType::Shield => Weapon::shield(),
    }
}

pub fn print_weapon(weapon: &Option<Weapon>) {
    match weapon {
        Some(w) => {
            println!("you have a {}!", w.name);
            println!("it does {} damage", w.damage);
            println!("it has a crit range of {}-{}", w.crit_range.0, w.crit_range.1);
            println!("it has a crit multiplier of {}", w.crit_multiplier);
        }
        None => {
            println!("you have no weapon!");
        }
    }
}

pub enum WeaponType {
    Sword,
    Axe,
    Dagger,
    Bow,
    Staff,
    Club,
    Shield,
}

impl WeaponType {
    pub fn iter() -> Iter<'static, WeaponType> {
        static WEAPON_TYPES: [WeaponType; 7] = [
            WeaponType::Sword,
            WeaponType::Axe,
            WeaponType::Dagger,
            WeaponType::Bow,
            WeaponType::Staff,
            WeaponType::Club,
            WeaponType::Shield,
        ];
        WEAPON_TYPES.iter()
    }
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WeaponType::Sword => write!(f, "sword"),
            WeaponType::Axe => write!(f, "axe"),
            WeaponType::Dagger => write!(f, "dagger"),
            WeaponType::Bow => write!(f, "bow"),
            WeaponType::Staff => write!(f, "staff"),
            WeaponType::Club => write!(f, "club"),
            WeaponType::Shield => write!(f, "shield"),
        }
    }
}

impl Clone for WeaponType {
    fn clone(&self) -> WeaponType {
        match self {
            WeaponType::Sword => WeaponType::Sword,
            WeaponType::Axe => WeaponType::Axe,
            WeaponType::Dagger => WeaponType::Dagger,
            WeaponType::Bow => WeaponType::Bow,
            WeaponType::Staff => WeaponType::Staff,
            WeaponType::Club => WeaponType::Club,
            WeaponType::Shield => WeaponType::Shield,
        }
    }
}

impl Clone for Weapon {
    fn clone(&self) -> Weapon {
        Weapon {
            name: self.name.clone(),
            weapon_type: self.weapon_type.clone(),
            damage: self.damage,
            crit_range: self.crit_range,
            crit_multiplier: self.crit_multiplier,
        }
    }
}