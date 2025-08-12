#[derive(Debug)]
pub struct Sheet {
    pub name: String,
    pub health: u32,
    pub armor: u32,
    pub attack: u32,
}

impl Sheet {
    pub fn new(name: String, health: u32, armor: u32, attack: u32) -> Self {
        Self {
            name,
            health,
            armor,
            attack,
        }
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
            println!("{} takes {} damage", self.name, damage);
            self.health -= damage;
        }

        if self.health < damage  {
            self.health = 0;
        }
    }
}
