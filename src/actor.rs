#[derive(Debug, PartialEq, Default)]
pub struct Actor {
    name: String,
    initiative_mod: i16,
    current_health: u16,
    max_health: u16,
}

impl Actor {
    pub fn new(name: &str, initiative_mod: i16, max_health: u16) -> Self {
        let name = name.to_string();
        let current_health = max_health.clone();
        Self {
            name,
            initiative_mod,
            current_health,
            max_health,
        }
    }
}

impl Actor {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn health(&self) -> String {
        format!("{}/{}", &self.current_health, &self.max_health)
    }

    pub fn current_health(&self) -> &u16 {
        &self.current_health
    }

    pub fn max_health(&self) -> &u16 {
        &self.max_health
    }

    pub fn initiative(&self) -> &i16 {
        &self.initiative_mod
    }

    pub fn damage(&mut self, amount: u16) {
        if self.current_health as i32 - amount as i32 <= 0 {
            self.current_health = 0;
        } else {
            self.current_health -= amount;
        }
    }

    pub fn heal(&mut self, amount: u16) {
        if self.current_health as i32 + amount as i32 >= self.max_health as i32 {
            self.current_health = self.max_health;
        } else {
            self.current_health += amount;
        }
    }
}
