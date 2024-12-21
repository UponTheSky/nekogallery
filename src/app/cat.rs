use serde_derive::Serialize;

#[derive(Clone, Serialize)]
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
