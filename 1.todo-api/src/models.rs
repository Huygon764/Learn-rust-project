use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            completed: false,
            created_at: now,
        }
    }

    pub fn update(&mut self, update_data: UpdateTodo) {
        if let Some(title) = update_data.title {
            self.title = title;
        }
        if let Some(description) = update_data.description {
            self.description = Some(description);
        }
        if let Some(completed) = update_data.completed {
            self.completed = completed;
        }
    }
}
