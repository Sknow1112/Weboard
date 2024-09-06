use crate::app::{DrawAction};
use std::collections::VecDeque;

pub struct Whiteboard {
    actions: VecDeque<DrawAction>,
    max_actions: usize,
}

impl Whiteboard {
    pub fn new() -> Self {
        Whiteboard {
            actions: VecDeque::new(),
            max_actions: 1000,
        }
    }

    pub fn add_action(&mut self, action: DrawAction) {
        if self.actions.len() >= self.max_actions {
            self.actions.pop_front();
        }
        self.actions.push_back(action);
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }

    pub fn get_actions(&self) -> Vec<DrawAction> {
        self.actions.iter().cloned().collect()
    }
}
