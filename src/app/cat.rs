use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Cat {
    pub id: u32,
    pub name: String,
}

impl Default for Cat {
    fn default() -> Self {
        Self {
            id: 0,
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

// for temporary usage, such as testing
pub static CATS: std::sync::LazyLock<Vec<Cat>> = std::sync::LazyLock::new(|| {
    vec![
        Cat {
            id: 0,
            name: "miyako".to_string(),
        },
        Cat {
            id: 1,
            name: "shibaneko".to_string(),
        },
    ]
});
