pub struct MonsterWeapon {
    pub name: String,
    pub damage: (i32, i32), // roll this many dice, with this many sides
    pub crit_range: (i32, i32),
    pub crit_multiplier: i32,
}

impl MonsterWeapon {
    pub fn new() -> MonsterWeapon {
        MonsterWeapon {
            name: String::from(""),
            damage: (0, 0),
            crit_range: (0, 0),
            crit_multiplier: 0,
        }
    }
}

impl Clone for MonsterWeapon {
    fn clone(&self) -> MonsterWeapon {
        MonsterWeapon {
            name: self.name.clone(),
            damage: self.damage,
            crit_range: self.crit_range,
            crit_multiplier: self.crit_multiplier,
        }
    }
}