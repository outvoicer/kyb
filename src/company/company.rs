#[derive(Debug, serde::Deserialize, Clone)]
/// schema for company register
pub struct Company {
    pub reg_code: String,
    pub name: String,
    pub address: Option<String>,
    pub zip: Option<u32>,
    pub legal_form: String,
}
