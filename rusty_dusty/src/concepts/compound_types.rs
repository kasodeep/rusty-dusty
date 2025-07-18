// Type alias for clarity and reusability
type Health = u32;
type Mana = u32;

// Enum to represent character classes
#[derive(Debug, PartialEq)]
enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
}

// Enum to represent character actions with associated data
#[derive(Debug)]
enum Action {
    Attack { damage: Health, target: String },
    CastSpell { spell_name: String, mana_cost: Mana },
    Dodge,
    Heal { amount: Health },
}

// Struct to represent a game character
#[derive(Debug)]
struct Character {
    name: String,
    health: Health,
    mana: Mana,
    class: CharacterClass,
    level: u32,
}

// Unit struct for default configuration
#[derive(Debug)]
struct DefaultCharacterConfig;

// Tuple struct for position in game world
#[derive(Debug, Clone, PartialEq)]
struct Position(i32, i32);

// Implementation for Character
impl Character {
    // Associated function to create a new character
    fn new(name: String, class: CharacterClass, level: u32) -> Self {
        let (health, mana) = match class {
            CharacterClass::Warrior => (100, 20),
            CharacterClass::Mage => (60, 80),
            CharacterClass::Rogue => (80, 40),
        };
        Character {
            name,
            health,
            mana,
            class,
            level,
        }
    }

    // Method to perform an action
    fn perform_action(&mut self, action: Action) -> String {
        match action {
            Action::Attack { damage, target } => {
                if self.health > 0 {
                    self.health = self.health.saturating_sub(10); // Simulate stamina cost
                    format!("{} attacks {} for {} damage!", self.name, target, damage)
                } else {
                    format!("{} is too weak to attack!", self.name)
                }
            }
            Action::CastSpell { spell_name, mana_cost } => {
                if self.mana >= mana_cost {
                    self.mana -= mana_cost;
                    format!("{} casts {}!", self.name, spell_name)
                } else {
                    format!("{} lacks mana to cast {}!", self.name, spell_name)
                }
            }
            Action::Dodge => format!("{} dodges an attack!", self.name),
            Action::Heal { amount } => {
                self.health = (self.health + amount).min(100);
                format!("{} heals for {} health!", self.name, amount)
            }
        }
    }

    // Method to move character to a new position
    fn move_to(&mut self, pos: Position) -> String {
        format!("{} moves to position ({}, {})", self.name, pos.0, pos.1)
    }
}

// Default implementation for config struct
impl Default for DefaultCharacterConfig {
    fn default() -> Self {
        DefaultCharacterConfig
    }
}

// Create default character
impl DefaultCharacterConfig {
    fn create_default_character(&self, name: String) -> Character {
        Character::new(name, CharacterClass::Warrior, 1)
    }
}

// Describe the action
impl Action {
    fn describe(&self) -> String {
        match self {
            Action::Attack { damage, target } => format!("Attack dealing {} damage to {}", damage, target),
            Action::CastSpell { spell_name, mana_cost } => format!("Cast {} costing {} mana", spell_name, mana_cost),
            Action::Dodge => "Dodge an incoming attack".to_string(),
            Action::Heal { amount } => format!("Heal for {} health", amount),
        }
    }
}

pub fn main() {
    println!("Game Character System\n");

    // Create a character using associated function
    let mut warrior = Character::new("Aragorn".to_string(), CharacterClass::Warrior, 5);
    println!("Created character: {:?}", warrior);

    // Create a default character
    let config = DefaultCharacterConfig::default();
    let default_char = config.create_default_character("Grok".to_string());
    println!("Default character: {:?}", default_char);

    // Perform actions
    let attack = Action::Attack { damage: 20, target: "Orc".to_string() };
    println!("Action description: {}", attack.describe());
    println!("Action result: {}", warrior.perform_action(attack));

    let spell = Action::CastSpell { spell_name: "Fireball".to_string(), mana_cost: 30 };
    println!("Action description: {}", spell.describe());
    println!("Action result: {}", warrior.perform_action(spell)); // Should fail due to low mana

    let heal = Action::Heal { amount: 30 };
    println!("Action result: {}", warrior.perform_action(heal));

    // Move character
    let new_position = Position(10, 15);
    println!("{}", warrior.move_to(new_position.clone()));

    // Pattern matching with enum
    match warrior.class {
        CharacterClass::Warrior => println!("{} is a mighty warrior!", warrior.name),
        CharacterClass::Mage => println!("{} is a powerful mage!", warrior.name),
        CharacterClass::Rogue => println!("{} is a stealthy rogue!", warrior.name),
    }

    // Destructuring a tuple struct
    let Position(x, y) = new_position;
    println!("Destructured position: x = {}, y = {}", x, y);

    // If let syntax for enum
    let action = Action::Dodge;
    if let Action::Dodge = action {
        println!("Action is a dodge!");
    }

    // While let loop for level progression
    let mut level = warrior.level;
    while let Some(new_level) = Some(level) {
        if new_level >= 10 {
            break;
        }
        println!("{} is at level {}", warrior.name, new_level);
        level += 1;
    }
}
