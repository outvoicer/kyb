#[derive(Debug, serde::Deserialize, Clone)]
/// schema for company register
pub struct Company {
    pub reg_code: String,
    pub name: String,
    pub address: Option<String>,
    pub zip: Option<u32>,
    pub legal_form: String,
    pub closed: Option<String>, // bool
}

impl Company {
    pub fn new(
        name: String,
        reg_code: String,
        address: String,
        zip: u32,
        legal_form: String,
        closed: String,
    ) -> Self {
        Self {
            name: name,
            reg_code: reg_code,
            address: Some(address),
            zip: Some(zip),
            legal_form: legal_form,
            closed: Some(closed),
        }
    }
}
