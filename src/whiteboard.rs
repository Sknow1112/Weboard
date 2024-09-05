use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrawAction {
    pub x: f64,
    pub y: f64,
    pub color: String,
    pub is_eraser: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WhiteboardAction {
    Draw(DrawAction),
    Clear,
    ChangeColor(String),
    Zoom(f64),
}
