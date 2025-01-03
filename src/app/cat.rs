use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Cat {
    pub id: String,
    pub name: String,
}

impl Default for Cat {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::from("noname"),
        }
    }
}

#[derive(Deserialize)]
pub struct CatDraft {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CatUpdate {
    pub name: Option<String>,
}
