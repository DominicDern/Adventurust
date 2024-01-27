use crate::{actor::Actor, roll::IndividualRoll};

#[derive(Debug)]
pub struct Encounter {
    actors: Vec<(u8, Actor)>,
    current: Option<usize>,
}

impl Encounter {
    pub fn new() -> Self {
        Encounter {
            actors: Vec::new(),
            current: Some(0),
        }
    }
}

impl Encounter {
    /// Rolls initiative for an actor and inserts it in it's respective place in initiative.
    pub fn add_actor(&mut self, actor: Actor) {
        let initiative = IndividualRoll::new(None, 20, Some(*actor.initiative())).roll() as u8;
        let mut index = 0;

        if self.actors.is_empty() {
            self.actors.push((initiative, actor));
            return;
        }

        for (temp_initiative, _) in &self.actors {
            if *temp_initiative < initiative {
                self.actors.insert(index, (initiative, actor));
                return;
            } else {
                index += 1;
            }
        }
        self.actors.push((initiative, actor));
    }

    pub fn actors(&self) -> Vec<&Actor> {
        let mut actors = Vec::new();

        for (_, actor) in &self.actors {
            actors.push(actor);
        }
        actors
    }

    pub fn non_current_actors(&self) -> Vec<&Actor> {
        let mut actors = Vec::new();

        for (_, actor) in &self.actors {
            if actor != self.get_current() {
                actors.push(actor);
            }
        }

        actors
    }

    pub fn get_current(&self) -> &Actor {
        &self.actors.get(self.current.unwrap()).unwrap().1
    }

    fn get_current_mut(&mut self) -> &mut Actor {
        &mut self.actors.get_mut(self.current.unwrap()).unwrap().1
    }

    pub fn get(&self, name: String) -> Option<&Actor> {
        self.actors
            .iter()
            .map(|(_, actor)| actor)
            .find(|&actor| actor.name().to_lowercase() == name.to_lowercase())
    }

    fn get_mut(&mut self, name: String) -> Option<&mut Actor> {
        self.actors
            .iter_mut()
            .map(|(_, actor)| actor)
            .find(|actor| actor.name().to_lowercase() == name.to_lowercase())
    }

    pub fn damage(&mut self, name: String, amount: u16) {
        if let Some(actor) = self.get_mut(name) {
            actor.damage(amount);
        }
    }

    pub fn heal(&mut self, name: String, amount: u16) {
        if let Some(actor) = self.get_mut(name) {
            actor.heal(amount);
        }
    }

    pub fn actions(&self) -> &(u8, u8) {
        self.get_current().actions()
    }

    pub fn bonus_actions(&self) -> &(u8, u8) {
        self.get_current().bonus_actions()
    }

    pub fn reactions(&self) -> &(u8, u8) {
        self.get_current().reactions()
    }

    pub fn all_reactions(&self) -> Vec<(&Actor, &(u8, u8))> {
        let mut actors = Vec::new();
        for (_, actor) in &self.actors {
            let reactions = self.get(actor.name().to_string()).unwrap().reactions();
            actors.push((actor, reactions));
        }
        actors
    }

    pub fn add_actions(&mut self, name: String, amount: u8) {
        self.get_mut(name).unwrap().add_actions(amount);
    }

    pub fn remove_actions(&mut self, name: String, amount: u8) {
        self.get_mut(name).unwrap().remove_actions(amount);
    }

    pub fn add_bonus_actions(&mut self, name: String, amount: u8) {
        self.get_mut(name).unwrap().add_bonus_actions(amount);
    }

    pub fn remove_bonus_actions(&mut self, name: String, amount: u8) {
        self.get_mut(name).unwrap().remove_bonus_actions(amount);
    }

    pub fn add_reactions(&mut self, name: String, amount: u8) {
        self.get_mut(name).unwrap().add_reactions(amount);
    }

    pub fn remove_reactions(&mut self, name: String, amount: u8) {
        self.get_mut(name).unwrap().remove_reactions(amount);
    }

    /// Moves the initiative order to the next actor and resets their actions.
    pub fn end_turn(&mut self) {
        match self.current {
            Some(index) => {
                // Check to see if this index is last in the vector
                if index == self.actors.len() - 1usize {
                    self.current = Some(0);
                    self.get_current_mut().reset_actions();
                } else {
                    self.current = Some(self.current.unwrap() + 1);
                    self.get_current_mut().reset_actions();
                }
            }
            None => {
                println!("No actors in the initiative order!");
            }
        };
    }
}
