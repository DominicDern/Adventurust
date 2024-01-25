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
        let initiative =
            IndividualRoll::new(None, 20, Some(actor.initiative().clone())).roll() as u8;
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
        for (_, actor) in &self.actors {
            if actor.name().to_lowercase() == name.to_lowercase() {
                return Some(actor);
            }
        }
        None
    }

    fn get_mut(&mut self, name: String) -> Option<&mut Actor> {
        for (_, actor) in &mut self.actors {
            if actor.name().to_lowercase() == name.to_lowercase() {
                return Some(actor);
            }
        }
        None
    }

    pub fn damage(&mut self, name: String, amount: u16) {
        match self.get_mut(name) {
            Some(actor) => {
                actor.damage(amount);
            }
            None => {}
        }
    }

    pub fn heal(&mut self, name: String, amount: u16) {
        match self.get_mut(name) {
            Some(actor) => {
                actor.heal(amount);
            }
            None => {}
        }
    }

    /// Moves the initiative order to the next actor.
    pub fn end_turn(&mut self) {
        match self.current {
            Some(index) => {
                // Check to see if this index is last in the vector
                if index == self.actors.len() - 1 as usize {
                    self.current = Some(0);
                } else {
                    self.current = Some(self.current.unwrap() + 1);
                }
            }
            None => {
                println!("No actors in the initiative order!");
            }
        };
    }
}
