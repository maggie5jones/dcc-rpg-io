pub mod basics;
pub mod enemy;
pub mod combat;

pub mod cleric;
pub mod thief;
pub mod warrior;
pub mod wizard;
pub mod dwarf;
pub mod halfling;
pub mod elf;


pub struct Character {
    pub name: String,
    pub class: u32,
    pub level: u32,

    pub strength: u32,
    pub strength_modifier: i32,

    pub agility: u32,
    pub agility_modifier: i32,

    pub stamina: u32,
    pub stamina_modifier: i32,

    pub personality: u32,
    pub personality_modifier: i32,

    pub intelligence: u32,
    pub intelligence_modifier: i32,

    pub luck: u32,
    pub luck_modifier: i32,

    pub max_spell_level: u32,

    pub xp: u32,
    pub max_health: u32,
    pub current_health: u32,
}

impl Character {
    /*

    character creation

    */
    
    pub fn new() -> Character {
        Character {
            name: String::new(),
            class: 0,
            level: 1,

            strength: 0,
            strength_modifier: 0,

            agility: 0,
            agility_modifier: 0,

            stamina: 0,
            stamina_modifier: 0,

            personality: 0,
            personality_modifier: 0,

            intelligence: 0,
            intelligence_modifier: 0,

            luck: 0,
            luck_modifier: 0,

            max_spell_level: 0,

            xp: 10,
            max_health: 0,
            current_health: 0,
        }
    }

    pub fn create(&mut self) {
        println!("please enter your character's name:");
        self.name = basics::get_user_input();

        self.strength = basics::roll(3, 6);
        self.agility = basics::roll(3, 6);
        self.stamina = basics::roll(3, 6);
        self.personality = basics::roll(3, 6);
        self.intelligence = basics::roll(3, 6);
        self.luck = basics::roll(3, 6);

        self.strength_modifier = basics::modifier(self.strength);
        self.agility_modifier = basics::modifier(self.agility);
        self.stamina_modifier = basics::modifier(self.stamina);
        self.personality_modifier = basics::modifier(self.personality);
        self.intelligence_modifier = basics::modifier(self.intelligence);
        self.luck_modifier = basics::modifier(self.luck);

        self.max_health = self.stamina;
        self.current_health = self.max_health;
        
        println!("these are your attributes:");
        println!("strength: {}  mod: {}", self.strength, self.strength_modifier);
        println!("agility: {}   mod: {}", self.agility, self.agility_modifier);
        println!("stamina: {}  mod: {}", self.stamina, self.stamina_modifier);
        println!("personality: {}  mod: {}", self.personality, self.personality_modifier);
        println!("intelligence: {}  mod: {}", self.intelligence, self.intelligence_modifier);
        println!("luck: {}  mod: {}", self.luck, self.luck_modifier);
        println!(" ");
        
        println!("type a number to choose your character's class!!");
        println!("(1) cleric");
        println!("(2) thief");
        println!("(3) warrior");
        println!("(4) wizard");
        println!("(5) dwarf");
        println!("(6) halfling");
        println!("(7) elf");
        
        self.class = basics::get_user_input().parse().unwrap();

        println!(
            "congratulations, {} the {} has been created!",
            self.name,
            self.get_class_name(),
        );
        
    }

    pub fn get_class_name(&self) -> &'static str {
        match self.class {
            1 => "cleric",
            2 => "thief",
            3 => "warrior",
            4 => "wizard",
            5 => "dwarf",
            6 => "halfling",
            7 => "elf",
            _ => "unknown",
        }
    }

    /*
    
    character functionality

    */

    pub fn rest(&mut self) {
        self.current_health += 1;
        if self.current_health > self.max_health {
            self.current_health = self.max_health;
        }
        if self.class == 2 {
            self.luck += self.level;
        }
    }

    pub fn check_level_up(&mut self) {
        if self.level > 10 {
            println!("you have reached the maximum level!");
        }
        else if self.xp >= 1090 && self.level == 10 {
            self.level_up();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level)
        }
        else if self.xp >= 890 && self.level == 9 {
            self.level_up();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level)
        }
        else if self.xp >= 710 && self.level == 8 {
            self.level_up();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level)
        }
        else if self.xp >= 550 && self.level == 7 {
            self.level_up();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level)
        }
        else if self.xp >= 410 && self.level == 6 {
            self.level_up();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level)
        }
        else if self.xp >= 290 && self.level == 5 {
            self.level_up();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level)
        }
        else if self.xp >= 190 && self.level == 3 {
            self.level_up();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level)
        }
        else if self.xp >= 110 && self.level == 2 {
            self.level_up();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level)
        }
        else if self.xp >= 50 && self.level == 1 {
            self.level_up();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level)
        }
        else {
            println!("you need more xp to level up!");
        }
    }
    
    pub fn level_up(&mut self) {
        self.level += 1;
        
        if self.class == 1 {
            cleric::level_up(self);

        } else if self.class == 2 {
            thief::level_up(self);

        } else if self.class == 3 {
            warrior::level_up(self);

        } else if self.class == 4 {
            wizard::level_up(self);

        } else if self.class == 5 {
            dwarf::level_up(self);

        } else if self.class == 6 {
            halfling::level_up(self);

        } else if self.class == 7 {
            elf::level_up(self);

        }
        self.current_health = self.max_health;
    }
}