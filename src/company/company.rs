use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
/// schema for company register
pub struct Company {
    pub legal_form: String,
    pub name: String,
    pub city: Option<String>,
    pub address: Option<String>,
    pub zip: Option<u32>,
    pub public_sector: String,
    pub reg_code: String,
    pub vat: bool,
    pub vat_number: Option<String>,
}
