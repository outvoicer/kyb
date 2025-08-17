#[derive(Debug, serde::Deserialize, Clone)]
/// schema for company register
pub struct Company {
    pub legal_form: String,
    pub name: String,
    pub city: Option<String>,
    pub address: Option<String>,
    pub zip: Option<u32>,
    pub reg_code: String,
}
