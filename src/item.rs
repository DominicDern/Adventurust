pub struct Item {
    name: String,
    description: String,
    // TODO add a way to add effects to items
    damage: Option<u16>,
    weight: u16,
}

impl Item {
    pub fn new(name: String, description: String, damage: Option<u16>, weight: u16) -> Self {
        Self {
            name,
            description,
            damage,
            weight,
        }
    }
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn damage(&self) -> &Option<u16> {
        &self.damage
    }

    pub fn weight(&self) -> &u16 {
        &self.weight
    }
}
