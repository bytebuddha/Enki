use crate::actions::Action;

const MAX_EVENTS: usize = 150;

pub struct Actions(Vec<Action>);

impl Actions {
    pub fn push(&mut self, event: Action) {
        if self.0.len() == MAX_EVENTS {
            self.0.pop();
        }
        self.0.insert(0, event);
    }

    pub fn actions(&self) -> &Vec<Action> {
        &self.0
    }
}

impl Default for Actions {
    fn default() -> Actions {
        Actions(Vec::with_capacity(MAX_EVENTS))
    }
}
