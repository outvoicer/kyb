use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Query {
    pub name: String,
    pub personal_code: String,
    pub reg_code: String,
}

impl Query {
    pub fn new(name: String, personal_code: String, reg_code: String) -> Self {
        Self {
            name: name,
            personal_code: personal_code,
            reg_code: reg_code,
        }
    }
}
