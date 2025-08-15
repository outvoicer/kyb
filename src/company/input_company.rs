#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
/// schema for register.cvs
pub struct InputCompany {
    pub regcode: String, //
    sepa: String,
    name: String,
    name_before_quotes: String,
    pub name_in_quotes: String, //
    name_after_quotes: String,
    without_quotes: Option<u8>,
    regtype: String,
    pub regtype_text: String,
    pub r#type: Option<String>,
    type_text: String,           //
    registered: String,          // NaiveDate,
    terminated: String,          // Option<NaiveDate>
    pub closed: String,          //
    pub address: Option<String>, //
    pub index: Option<u32>,
    addressid: u64,
    region: u64,
    city: Option<u32>,
    atvk: String,
    reregistration_term: Option<String>,
}
