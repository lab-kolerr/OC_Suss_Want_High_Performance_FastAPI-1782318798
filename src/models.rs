use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AIBehavior {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub behavior_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Swarm {
    pub id: String,
    pub agents: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectionEvent {
    pub id: String,
    pub timestamp: String,
    pub agent_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub id: String,
    pub settings: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub id: String,
    pub message: String,
    pub level: String,
    pub timestamp: String,
}