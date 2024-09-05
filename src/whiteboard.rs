use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::database::Database;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrawAction {
    pub x: f64,
    pub y: f64,
    pub color: String,
    #[serde(rename = "isEraser")]
    pub is_eraser: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WhiteboardAction {
    Draw(DrawAction),
    Clear,
    ChangeColor(String),
    Zoom(f64),
    InitialState(Vec<DrawAction>),
    StateUpdate(Vec<DrawAction>),
}

pub struct WhiteboardManager {
    current_state: Vec<DrawAction>,
    pending_actions: Vec<DrawAction>,
    db: Arc<Database>,
}

impl WhiteboardManager {
    pub fn new(db: Arc<Database>) -> Self {
        let current_state = match db.get_current_state() {
            Ok(state) => state,
            Err(e) => {
                log::error!("Failed to get current state from database: {}", e);
                Vec::new()
            }
        };
        log::info!("Initialized WhiteboardManager with {} actions", current_state.len());
        WhiteboardManager {
            current_state,
            pending_actions: Vec::new(),
            db,
        }
    }

    pub fn apply_action(&mut self, action: &WhiteboardAction) {
        match action {
            WhiteboardAction::Draw(draw_action) => {
                self.pending_actions.push(draw_action.clone());
                if let Err(e) = self.db.save_action(draw_action) {
                    log::error!("Failed to save action to database: {}", e);
                }
            }
            WhiteboardAction::Clear => {
                self.current_state.clear();
                self.pending_actions.clear();
                if let Err(e) = self.db.clear_whiteboard() {
                    log::error!("Failed to clear whiteboard in database: {}", e);
                }
            }
            _ => {}
        }
        log::info!("Applied action: {:?}", action);
    }

    pub fn get_current_state(&self) -> Vec<DrawAction> {
        log::info!("Getting current state with {} actions", self.current_state.len());
        self.current_state.clone()
    }

    pub fn combine_changes(&mut self) -> Vec<DrawAction> {
        let update = self.pending_actions.clone();
        self.current_state.extend(self.pending_actions.drain(..));
        log::info!("Combined {} new actions into current state", update.len());
        update
    }
}
