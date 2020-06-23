use serde::{Deserialize, Serialize};

use crate::xi::ViewId;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AvailableLanguages {
    pub languages: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LanguageChanged {
    pub view_id: ViewId,
    pub language_id: String,
}
