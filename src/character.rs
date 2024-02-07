pub mod basics;
pub mod enemy;
pub mod combat;
pub mod classes;

pub struct Bonuses {
    pub initiative_bonus: i32,
    pub attack_bonus: i32,
}

pub struct Character {
    pub name: String,
    pub class: i32,
    pub alignment: i32,
    pub level: i32,
    pub bonuses: Bonuses,

    pub strength: i32,
    pub strength_modifier: i32,

    pub agility: i32,
    pub agility_modifier: i32,

    pub stamina: i32,
    pub stamina_modifier: i32,

    pub personality: i32,
    pub personality_modifier: i32,

    pub intelligence: i32,
    pub intelligence_modifier: i32,

    pub luck: i32,
    pub luck_modifier: i32,

    pub max_spell_level: i32,

    pub xp: i32,
    pub armor_class: i32,
    
    pub max_health: i32,
    pub current_health: i32,
    pub weapon: Option<combat::weapon::Weapon>,
}

impl Character {
    /*

    character creation

    */
    
    pub fn new() -> Character {
        Character {
            name: String::new(),
            class: 0,
            alignment: 0,
            level: 1,
            bonuses: Bonuses {
                initiative_bonus: 0,
                attack_bonus: 0,
            },

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
            armor_class: 0,
            
            max_health: 0,
            current_health: 0,
            weapon: None,
        }
    }

    pub fn create(&mut self) {
        println!();
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

        self.max_health = basics::roll(1, 4) + self.stamina_modifier;
        if self.max_health < 1 {
            self.max_health = 1;
        }
        self.armor_class = 10 + self.agility_modifier;
        self.current_health = self.max_health;
        
        println!();
        println!("these are your attributes:");
        println!("strength: {}  mod: {}", self.strength, self.strength_modifier);
        println!("agility: {}   mod: {}", self.agility, self.agility_modifier);
        println!("stamina: {}  mod: {}", self.stamina, self.stamina_modifier);
        println!("personality: {}  mod: {}", self.personality, self.personality_modifier);
        println!("intelligence: {}  mod: {}", self.intelligence, self.intelligence_modifier);
        println!("luck: {}  mod: {}", self.luck, self.luck_modifier);
        println!();
        
        println!("type a number to choose your character's class!!");
        println!("(1) cleric");
        println!("(2) thief");
        println!("(3) warrior");
        println!("(4) wizard");
        println!("(5) dwarf");
        println!("(6) halfling");
        println!("(7) elf");
        println!();
        
        self.class = basics::get_user_input().parse().unwrap();

        println!();
        println!("now type a number to choose your character's alignment!!");
        println!("(1) lawful");
        println!("(2) neutral");
        println!("(3) chaotic");
        println!();

        self.alignment = basics::get_user_input().parse().unwrap();

        println!();
        println!("{} the {} {} has been created!", self.name, self.get_alignment_name(), self.get_class_name());
        self.print_character_art();
        println!("here is some more info about your {}:", self.get_class_name());
        if self.class == 1 {
            classes::cleric::print_cleric_shit(self);
        } else if self.class == 2 {
            classes::thief::gen_thief_skills(self);
            unsafe { classes::thief::print_thief_skills(self) };
        } else if self.class == 3 {
            classes::warrior::print_warrior_shit(self);
        } else if self.class == 4 {
            classes::wizard::print_wizard_shit(self);
        } else if self.class == 5 {
            classes::dwarf::print_dwarf_shit(self);
        } else if self.class == 6 {
            classes::halfling::print_halfling_shit(self);
        } else if self.class == 7 {
            classes::elf::print_elf_shit(self);
        } println!()
    }

    pub fn get_alignment_name(&self) -> &'static str {
        match self.alignment {
            1 => "lawful",
            2 => "neutral",
            3 => "chaotic",
            _ => "unknown",
        }
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

    pub fn print_character_art(&self) {
        // all ascii art from https://ascii.co.uk/art
        if self.class == 1 {
            println!();
            println!("  ,^.                      ");
            println!("  |||                      ");
            println!("  |||       _T_            ");
            println!("  |||   .-.[:|:].-.        ");
            println!("  ===_ /\\|  \"'\"  |/     ");
            println!("  E]_|\\/ \\--|-|'''''|    ");
            println!("  O  `'  '=[:]| _|_ |      ");
            println!("         /\"\"\"\"|  |  |  ");
            println!("        /\"\"\"\"\"`.___.' ");
            println!("        []\"/\"\"\"\\\"[]  ");
            println!("        | \\     / |       ");
            println!("        | |     | |        ");
            println!("      <\\\\\\)    (///>    ");
            println!();
        }
        else if self.class == 2 {
            println!();
            println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⡿⠿⠆⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢻⣿⣿⣿⠋⢀⡔⠁⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠿⠿⣿⢠⡞⠀⠀⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⠀⠀⠀⢀⣀⠀⠀⠀");
            println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣾⣿⣿⣿⣿⣿⣿⠀⣀⣤⣾⠿⠀⠀⠀");
            println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣘⣯⣿⣿⣿⣿⣿⣿⣿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣤⠀⠀⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠙⠛⠻⠿⣿⣿⠀⠀⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⠀⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⢀⣶⣾⣿⠟⠛⠻⠿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⣿⡟⠀⠀⠀⠀⠀⠀⠀");
            println!("⠀⠀⠀⡾⠟⠋⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠀⠀⠀⠸⢿⣧⣄⠀⠀⠀⠀⠀⠀");
            println!();

        }
        else if self.class == 3 {
            println!();
            println!("|\\             //");
            println!(" \\\\           _!_");
            println!("  \\\\         /___\\");
            println!("   \\\\        [+++]");
            println!("    \\\\    _ _\\^^^/_ _");
            println!("     \\\\/ (    '-'  ( )");
            println!("     /( \\/ |       /\\ \\");
            println!("       \\  / \\     / _> )");
            println!("        \"`   >:::;-'`;\"'-.");
            println!("            /:::/    |   \\");
            println!("           /  /||----|----|");
            println!("          (  / (\\    |    /");
            println!("          / /   \\'-._|_.-'");
            println!("        _/ /     \\ \\");
            println!("       /___|    /___|");
            println!();
        }
        else if self.class == 4 {
            println!();
            println!("                     ____");
            println!("                   .'* *.'");
            println!("                __/_*_*(_");
            println!("               / _______ \\");
            println!("              _\\_)/___\\(_/_");
            println!("             / _((\\- -/))_ \\");
            println!("             \\ \\())(-)(()/ /");
            println!("              ' \\(((()))/ '");
            println!("             / ' \\)).))/ ' \\");
            println!("            / _ \\ - | - /_  \\");
            println!("           (   ( .;''';. .'  )");
            println!("           _\\\"__ /    )\\ __\"/_");
            println!("             \\/  \\   ' /  \\/");
            println!("              .'  '...' ' )");
            println!("               / /  |  \\ \\");
            println!("              / .   .   . \\");
            println!("            /   .     .   \\");
            println!("           /   /   |   \\   \\");
            println!("         .'   /    b    '.  '.");
            println!("     _.-'    /     Bb     '-. '-._");
            println!(" _.-'       |      BBb       '-.  '-.");
            println!("(___________\\____.dBBBb.________)____)");
            println!();
        }
        else if self.class == 5 {
            println!();
            println!("      __      _");
            println!("     /__\\__  //");
            println!("    //_____\\///");
            println!("   _| /-_-\\)|/_");
            println!("  (___\\ _ //___\\");
            println!("  (  |\\\\_/// * \\\\");
            println!("   \\_| \\_((*   *))");
            println!("   ( |__|_\\\\  *//");
            println!("   (o/  _  \\_*_/");
            println!("   //\\__|__/\\");
            println!("  // |  | | |");
            println!(" //  _\\ | |__)");
            println!("//  (___|");
            println!();
        }
        else if self.class == 6 {
            println!();
            println!("           .-\"---.");
            println!("          /       \\");
            println!("         /\\___.-'./\\");
            println!("         \\/ O) (O \\/");
            println!("    __    |  (_)  |");
            println!("   / /  __/\\\\___//\\__");
            println!("   | (_/\\ \\/`---'\\/ /\\");
            println!("   \\ \\/  \\  \\   /  /  \\");
            println!("   /\\|   /  -| |-  \\   \\");
            println!("  | ||  /\\  -| |-  /\\   \\");
            println!("   \\/|_/ |  -|_\\-  |/   /");
            println!("   \\ \\   /  /B_B\\  \\\\  /");
            println!("   / (   \\_/  _  \\_/ \\/");
            println!("   \\ \\   /    |    \\_/");
            println!("   ) /   | __ | __ |");
            println!("   |(    \\    |    /");
            println!("   /|    /____|____\\");
            println!("   ||     |   ||   |");
            println!("   \\\\     ///\\\\//\\\\\\");
            println!("    \\|   (____)(____)");
            println!();
        }
        else if self.class == 7 {
            println!();
            println!("           .-----.");
            println!(" \\ ' /   _/    )/");
            println!("- ( ) -('---''--)");
            println!(" / . \\((()\\^_^/)()");
            println!("  \\\\_\\ (()_)-((()()");
            println!("   '- \\ )/\\ _./(()");
            println!("     '/\\/( X   ) \\");
            println!("     (___)|___/ ) \\");
            println!("          |.#_|(___)");
            println!("         /\\    \\ ( (_");
            println!("         \\/\\/\\/\\) \\\\");
            println!("         | / \\ |");
            println!("         |(   \\|");
            println!("        _|_)__|_\\_");
            println!("        )...()...(");
            println!("         | (   \\ |");
            println!("      .-'__,)  (  \\");
            println!("                '\\_-,");
            println!();
        }
    }

    /*
    
    character functionality

    */

    #[allow(dead_code)]
    pub fn rest(&mut self) {
        self.current_health += 1;
        if self.current_health > self.max_health {
            self.current_health = self.max_health;
        }
        if self.class == 2 {
            self.luck += self.level;
        } //TODO: implement time (when to quick rest vs long rest)
    }

    pub fn check_level_up(&mut self) {
        let levels = [0, 10, 11, 30, 40, 50, 60, 70, 80, 90, 100];

        if self.level > 10 {
            println!("you have reached the maximum level!");
        }
        else if (self.xp >= 100 && self.level == 9) ||
                 (self.xp >= 90 && self.level == 8) ||
                 (self.xp >= 80 && self.level == 7) ||
                 (self.xp >= 70 && self.level == 6) ||
                 (self.xp >= 60 && self.level == 5) ||
                 (self.xp >= 50 && self.level == 4) ||
                 (self.xp >= 40 && self.level == 3) ||
                 (self.xp >= 30 && self.level == 2) ||
                 (self.xp >= 11 && self.level == 1) {
            self.level_up();
            println!();
            println!("congratulations, {} the {} has leveled up to level {}!", self.name, self.get_class_name(), self.level);
            println!();
            println!("                                   .''.                   ");
            println!("       .''.      .        *''*    :_\\/_:     .             ");
            println!("      :_\\/_:   _\\(/_  .:.*_\\/_*   : /\\ :  .'.:.'.            ");
            println!("  .''.: /\\ :   ./)\\   ':'* /\\ * :  '..'.  -=:o:=-");
            println!(" :_\\/_:'.:::. ;  ' *''*    * '.\\'/.' _\\(/_'.':'.'");
            println!(" : /\\ : :::::  '  *_\\/_*     -= o =-  /)\\    '  *");
            println!("  '..'  ':::'     * /\\ *     .'/.\\'.     . '");
            println!("      *            *..*         :         *.");
            self.print_character_art();
        }
        else {
            println!();
            println!("you need {} more xp to level up!", levels[self.level as usize + 1] - self.xp);
            println!();
        }
    }
    
    pub fn level_up(&mut self) {
        self.level += 1;
        self.bonuses.attack_bonus += 1;
        
        if self.class == 1 {
            classes::cleric::level_up(self);

        } else if self.class == 2 {
            unsafe{ classes::thief::level_up(self) };

        } else if self.class == 3 {
            unsafe{ classes::warrior::level_up(self) };

        } else if self.class == 4 {
            classes::wizard::level_up(self);

        } else if self.class == 5 {
            classes::dwarf::level_up(self);

        } else if self.class == 6 {
            classes::halfling::level_up(self);

        } else if self.class == 7 {
            classes::elf::level_up(self);

        }
        self.current_health = self.max_health;
    }
}