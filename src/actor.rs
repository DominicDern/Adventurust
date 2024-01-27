use crate::inventory::Inventory;

pub enum Error {
    NoMoreActions,
}

#[derive(Debug, PartialEq, Default)]
pub struct Actor {
    name: String,
    initiative_mod: i16,
    current_health: u16,
    max_health: u16,
    actions: (u8, u8),
    bonus_actions: (u8, u8),
    reactions: (u8, u8),
    inventory: Inventory,
}

impl Actor {
    pub fn new(
        name: &str,
        initiative_mod: i16,
        max_health: u16,
        actions: u8,
        bonus_actions: u8,
        reactions: u8,
    ) -> Self {
        let name = name.to_string();
        let current_health = max_health;
        Self {
            name,
            initiative_mod,
            current_health,
            max_health,
            actions: (actions, actions),
            bonus_actions: (bonus_actions, bonus_actions),
            reactions: (reactions, reactions),
            inventory: Inventory::new(),
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

    pub fn actions(&self) -> &(u8, u8) {
        &self.actions
    }

    pub fn use_action(&mut self) -> Option<Error> {
        if self.actions().0 == 0 {
            Some(Error::NoMoreActions)
        } else {
            self.actions.0 -= 1;
            None
        }
    }

    pub fn use_bonus_action(&mut self) -> Option<Error> {
        if self.bonus_actions().0 == 0 {
            Some(Error::NoMoreActions)
        } else {
            self.bonus_actions.0 -= 1;
            None
        }
    }

    pub fn use_reaction(&mut self) -> Option<Error> {
        if self.reactions().0 == 0 {
            Some(Error::NoMoreActions)
        } else {
            self.reactions.0 -= 1;
            None
        }
    }

    pub fn reset_actions(&mut self) {
        self.actions.0 = self.actions.1;
        self.bonus_actions.0 = self.bonus_actions.1;
        self.reactions.0 = self.reactions.1;
    }

    pub fn add_actions(&mut self, amount: u8) {
        self.actions = (self.actions.0 + amount, self.actions.1 + amount);
    }

    pub fn remove_actions(&mut self, amount: u8) {
        self.actions = (self.actions.0 - amount, self.actions.1 - amount);
    }

    pub fn bonus_actions(&self) -> &(u8, u8) {
        &self.bonus_actions
    }

    pub fn add_bonus_actions(&mut self, amount: u8) {
        self.bonus_actions = (self.bonus_actions.0 + 1, self.bonus_actions.1 + 1);
    }

    pub fn remove_bonus_actions(&mut self, amount: u8) {
        self.bonus_actions = (self.bonus_actions.0 - 1, self.bonus_actions.1 - 1);
    }

    pub fn reactions(&self) -> &(u8, u8) {
        &self.reactions
    }

    pub fn add_reactions(&mut self, amount: u8) {
        self.reactions = (self.reactions.0 + 1, self.reactions.0 + 1);
    }

    pub fn remove_reactions(&mut self, amount: u8) {
        self.reactions = (self.reactions.0 - 1, self.reactions.1 - 1);
    }
}
