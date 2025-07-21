use rand::Rng;

pub struct Dice {
    sides: u32,
}

impl Dice {
    pub fn new(sides: u32) -> Dice {
        Dice { sides }
    }

    pub fn roll(&self) -> u32 {
        rand::thread_rng().gen_range(1..=self.sides)
    }
}

pub struct Player {
    name: String,
    health: u32,
    armor: u32,
    attack: u32,
}

impl Player {
    pub fn new(name: String, health: u32, armor: u32, attack: u32) -> Self {
        Self {
            name,
            health,
            armor,
            attack,
        }
    }

    pub fn get_attack(&self) -> u32 {
        self.attack
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: u32) {
        if damage < self.armor {
            return;
        }

        let damage = damage - self.armor;

        if self.health >= damage {
            self.health -= damage;
        }

        if self.health < damage  {
            self.health = 0;
        }
    }
}
