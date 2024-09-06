use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawAction {
    pub color: String,
    pub size: f32,
    pub points: Vec<Point>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Draw(DrawAction),
    Clear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Update(Vec<DrawAction>),
    Clear,
}
